use crate::error::Result;
use crate::molecule::bond::{Bond, AROMATIC, IMPLICT, SINGLE};
use crate::molecule::{Atom, Molecule};
use crate::molecule::{DB1, DB2};

pub fn collapse(mol: &Molecule) -> Result<Molecule> {
    let mol = implicit_to_explicit(mol)?;
    let mol = to_subset_atoms(&mol)?;
    explicit_to_implicit(&mol)
}

pub fn expand(mol: &Molecule) -> Result<Molecule> {
    let mol = implicit_to_explicit(mol)?;
    let mol = from_subset_atoms(&mol)?;
    explicit_to_implicit(&mol)
}

pub fn atom_based_db_stereo(mol: &Molecule) -> Result<Molecule> {
    let mut result = Molecule::new();

    // Copy original topology information first (this is unchanged)
    for &u in mol.atoms() {
        if let Some(topo) = mol.topology_at(&u) {
            // Only copy non-trigonal topologies (they will be replaced)
            if topo.seq() != crate::molecule::topology::TopologySeq::Trigonal {
                let atom_idx = topo.atom() as u8;
                let conf = topo.configuration().unwrap_or(crate::molecule::UNKNOWN);
                let vs = match topo.seq() {
                    crate::molecule::topology::TopologySeq::Tetrahedral => {
                        // For tetrahedral, get the 4 vertices
                        vec![
                            atom_idx as i8,
                            atom_idx as i8,
                            atom_idx as i8,
                            atom_idx as i8,
                        ]
                    }
                    _ => vec![],
                };
                if let Ok(new_topo) = crate::molecule::topology::create(atom_idx, conf, vs) {
                    result.add_topology(new_topo);
                }
            }
        }
    }

    // For each double bond add trigonal topology based on directional labels
    let mut es = Vec::new();
    for &u in mol.atoms() {
        for v in mol.graph().neighbors(&u)? {
            let v = *v;
            if v > u {
                let b = mol.edge_at(u, v)?;
                if b.is("=") {
                    es.push((u, v));
                }
            }
        }
    }

    for (u, v) in es {
        // add to topologies
        let topo_u = to_trigonal(mol, u, v);
        let topo_v = to_trigonal(mol, v, u);
        if let Ok(t) = topo_u {
            eprintln!(
                "DEBUG: Added topology for atom {}: {:?}",
                u,
                t.configuration()
            );
            result.add_topology(t);
        }
        if let Ok(t) = topo_v {
            result.add_topology(t);
        }
    }

    // Copy atoms - convert subset atoms with topology to bracket atoms
    for &idx in mol.atoms() {
        let atom = mol.atom_at(&idx)?;
        let mut added = false;
        let has_topology = result.topology_at(&idx).is_some();
        if (atom.is_aliphatic() || atom.is_aromatic()) && has_topology {
            // compute sum of bond orders for implicit H calculation
            let mut sum = if atom.is_aromatic() { 1 } else { 0 };
            for n in mol.graph().neighbors(&idx)? {
                sum += mol.edge_at(idx, *n)?.electron() as i32;
            }
            let hydrogens = if atom.is_aromatic() {
                atom.element().implicit_aromatic_hydrogen_count(sum)
            } else {
                atom.element().implicit_hydrogen_count(sum)
            };
            let bracket = Atom::new_bracket(
                atom.element(),
                -1,
                hydrogens as u8,
                0,
                atom.is_aromatic(),
                atom.is_organogen(),
            );
            result.add_atom(bracket)?;
            added = true;
        }
        if !added {
            result.add_atom(atom.clone())?;
        }
    }

    // append edges, replacing directional edges with implicit
    for &idx in mol.atoms() {
        for neighbor in mol.graph().neighbors(&idx)? {
            if *neighbor > idx {
                let b = mol.edge_at(idx, *neighbor)?;
                let new_b = if b.direction() { IMPLICT } else { *b };
                result.add_bond(idx, *neighbor, new_b)?;
            }
        }
    }

    result.set_flags(mol.get_flag(0xFF));
    run_post_processing(&mut result)?;
    Ok(result)
}

pub fn bond_based_db_stereo(mol: &Molecule) -> Result<Molecule> {
    use std::collections::HashMap;

    let mut visited: HashMap<u8, bool> = HashMap::new();
    let mut ordering: HashMap<u8, usize> = HashMap::new();
    let mut replacements: HashMap<(u8, u8), crate::molecule::bond::Bond> = HashMap::new();

    fn visit(
        mol: &Molecule,
        p: u8,
        u: u8,
        visited: &mut HashMap<u8, bool>,
        ordering: &mut HashMap<u8, usize>,
        replacements: &mut HashMap<(u8, u8), crate::molecule::bond::Bond>,
    ) {
        visited.insert(u, true);

        // collect neighbor list
        let mut es: Vec<u8> = Vec::new();
        if let Ok(neighbors) = mol.graph().neighbors(&u) {
            for n in neighbors {
                es.push(*n);
            }
        } else {
            return;
        }

        let mut offset: isize = -1;
        for (idx, &v) in es.iter().enumerate() {
            if !visited.get(&v).copied().unwrap_or(false) {
                visit(mol, u, v, visited, ordering, replacements);
            }
            ordering.insert(v, 2 + idx);
            if mol.edge_at(u, v).unwrap().is("=") {
                offset = idx as isize;
            }
        }

        ordering.insert(p, 0);
        ordering.insert(u, 1);

        if mol.topology_at(&u).is_some() {
            let t = mol.topology_at(&u).unwrap();
            // check if trigonal (double bond) topology
            if t.seq() == crate::molecule::topology::TopologySeq::Trigonal {
                if offset < 0 {
                    // nothing we can do
                    return;
                }

                // Build ordering vector for order_by
                let max_atom_idx = mol.atoms().iter().max().copied().unwrap_or(0) as usize;
                let mut ordering_vec: Vec<i8> = vec![0; max_atom_idx + 1];
                for (&k, &v) in ordering.iter() {
                    if k <= max_atom_idx as u8 {
                        ordering_vec[k as usize] = v as i8;
                    }
                }

                // order the topology to match traversal
                let maybe_top = t.order_by(&ordering_vec);
                if maybe_top.is_none() {
                    return;
                }
                let topology = maybe_top.unwrap();

                let mut j = if topology.configuration().unwrap().is_anti_clockwise() {
                    0
                } else {
                    1
                };

                // which end of the double bond we're looking from
                let other = es[offset as usize];
                if ordering.get(&other).copied().unwrap_or(0)
                    < ordering.get(&u).copied().unwrap_or(0)
                {
                    // no-op
                } else if es.len() == 2
                    && ordering.get(&u).copied().unwrap_or(0)
                        < ordering
                            .get(&es[(offset as usize + 1) % es.len()])
                            .copied()
                            .unwrap_or(0)
                {
                    j += 1;
                }

                let labels = [crate::molecule::bond::DOWN, crate::molecule::bond::UP];
                for k in 1..es.len() {
                    let eidx = es[(offset as usize + k) % es.len()];
                    let label = labels[j % 2];
                    j += 1;
                    // check for existing replacement conflict
                    let key = (u, eidx);
                    if let Some(existing) = replacements.get(&key) {
                        if *existing != label {
                            // on conflict: collect all current labels for inversion
                            let keys_to_invert: Vec<(u8, u8)> =
                                replacements.keys().cloned().collect();
                            for &k in &keys_to_invert {
                                if let Some(bond) = replacements.get(&k) {
                                    replacements.insert(k, bond.inverse());
                                }
                            }
                        }
                    }
                    replacements.insert(key, label);
                }
            }
        }
    }

    // start traversal - only visit atoms that actually exist
    for &u in mol.atoms() {
        if !visited.get(&u).copied().unwrap_or(false) {
            visit(mol, u, u, &mut visited, &mut ordering, &mut replacements);
        }
    }

    // build new molecule copying atoms but removing trigonal topologies
    let mut result = Molecule::new();
    for &idx in mol.atoms() {
        let atom = mol.atom_at(&idx)?;
        // Check if this atom has a trigonal (double bond) topology
        if let Some(topo) = mol.topology_at(&idx) {
            if topo.seq() == crate::molecule::topology::TopologySeq::Trigonal {
                // Remove the trigonal topology by reducing to subset atom
                let reduced_atom = to_subset(atom, mol, idx)?;
                result.add_atom(reduced_atom)?;
            } else {
                // Copy other topologies
                result.add_atom(atom.clone())?;
                // Recreate the topology
                let conf = topo.configuration().unwrap_or(crate::molecule::UNKNOWN);
                let vs = vec![]; // Simplified - proper reconstruction would need actual vertices
                if let Ok(new_topo) = crate::molecule::topology::create(idx, conf, vs) {
                    result.add_topology(new_topo);
                }
            }
        } else {
            result.add_atom(atom.clone())?;
        }
    }

    // Transfer directional bonds from the original molecule
    for &idx in mol.atoms() {
        for neighbor in mol.graph().neighbors(&idx)? {
            if *neighbor > idx {
                let bond = mol.edge_at(idx, *neighbor)?;
                if bond.direction() {
                    result.add_bond(idx, *neighbor, *bond)?;
                }
            }
        }
    }

    // append edges applying replacements
    for &idx in mol.atoms() {
        for neighbor in mol.graph().neighbors(&idx)? {
            if *neighbor > idx {
                let key = (idx, *neighbor);
                if let Some(b) = replacements.get(&key) {
                    result.add_bond(idx, *neighbor, *b)?;
                } else {
                    result.add_bond(idx, *neighbor, *mol.edge_at(idx, *neighbor)?)?;
                }
            }
        }
    }

    result.set_flags(mol.get_flag(0xFF));
    run_post_processing(&mut result)?;
    Ok(result)
}

fn to_trigonal(
    mol: &Molecule,
    u: u8,
    v: u8,
) -> Result<Box<dyn crate::molecule::topology::Topology + Sync>> {
    // Collect neighbors into a Vec to allow indexing
    let es: Vec<u8> = mol.graph().neighbors(&u)?.copied().collect();
    let offset = es.iter().position(|&x| x == v);
    if offset.is_none() {
        return crate::molecule::topology::create(u, crate::molecule::UNKNOWN, vec![]);
    }
    let offset = offset.unwrap();

    let mut vs: Vec<i8> = vec![v as i8, u as i8, u as i8];

    // Helper functions to check bond direction
    fn is_up(bond: &crate::molecule::bond::Bond) -> bool {
        bond.is("/") || bond == &crate::molecule::bond::UP
    }

    fn is_down(bond: &crate::molecule::bond::Bond) -> bool {
        bond.is("\\") || bond == &crate::molecule::bond::DOWN
    }

    if es.len() == 2 {
        let e1 = es[(offset + 1) % 2];
        let bond = mol.edge_at(u, e1)?;
        if is_up(&bond) {
            vs[1] = e1 as i8;
        } else if is_down(&bond) {
            vs[2] = e1 as i8;
        }
    } else if es.len() == 3 {
        let e1 = es[(offset + 1) % 3];
        let e2 = es[(offset + 2) % 3];
        let b1 = mol.edge_at(u, e1)?;
        let b2 = mol.edge_at(u, e2)?;

        // Check if b1 is single/implicit (non-directional)
        if b1.is("-") || b1.is("") {
            if is_up(&b2) {
                vs[1] = e2 as i8;
                vs[2] = e1 as i8;
            } else if is_down(&b2) {
                vs[1] = e1 as i8;
                vs[2] = e2 as i8;
            }
        } else {
            if is_up(&b1) {
                vs[1] = e1 as i8;
                vs[2] = e2 as i8;
            } else if is_down(&b1) {
                vs[1] = e2 as i8;
                vs[2] = e1 as i8;
            }
        }
    }

    if vs[1] == vs[2] {
        return crate::molecule::topology::create(u, crate::molecule::UNKNOWN, vec![]);
    }

    // Use the corrected logic:
    // Configuration is DB2 if the double bond partner is at a lower index than the current atom
    // This ensures the left carbon in SMILES gets DB1 and the right carbon gets DB2
    let double_bond_partner = es[offset as usize];
    let conf = if double_bond_partner < u { DB2 } else { DB1 };
    crate::molecule::topology::create(u, conf, vs)
}

pub fn explicit_to_implicit(mol: &Molecule) -> Result<Molecule> {
    let mut result = Molecule::new();

    for &idx in mol.atoms() {
        let atom = mol.atom_at(&idx)?;
        result.add_atom(atom.clone())?;
    }

    // Copy topologies
    for &idx in mol.atoms() {
        if let Some(topo) = mol.topology_at(&idx) {
            let conf = topo.configuration().unwrap_or(crate::molecule::UNKNOWN);
            let vs = match topo.seq() {
                crate::molecule::topology::TopologySeq::Tetrahedral => {
                    vec![idx as i8, idx as i8, idx as i8, idx as i8]
                }
                crate::molecule::topology::TopologySeq::Trigonal => {
                    let mut vs = Vec::new();
                    for n in mol.graph().neighbors(&idx)? {
                        if mol.edge_at(idx, *n)?.is("=") {
                            vs.insert(0, *n as i8);
                        } else {
                            vs.push(*n as i8);
                        }
                    }
                    if vs.len() >= 3 {
                        vec![vs[0], vs[1], vs[2]]
                    } else {
                        vec![0, idx as i8, idx as i8]
                    }
                }
                _ => vec![],
            };
            if let Ok(new_topo) = crate::molecule::topology::create(idx, conf, vs) {
                result.add_topology(new_topo);
            }
        }
    }

    for &idx in mol.atoms() {
        for neighbor in mol.graph().neighbors(&idx)? {
            if *neighbor > idx {
                if let Ok(bond) = mol.edge_at(idx, *neighbor) {
                    let implicit_bond =
                        to_implicit_bond(mol.atom_at(&idx)?, mol.atom_at(neighbor)?, *bond);
                    result.add_bond(idx, *neighbor, implicit_bond)?;
                }
            }
        }
    }

    result.set_flags(mol.get_flag(0xFF));

    run_post_processing(&mut result)?;

    Ok(result)
}

fn to_implicit_bond(u: &Atom, v: &Atom, bond: Bond) -> Bond {
    if bond.is("-") || bond.is(":") {
        if u.is_aromatic() && v.is_aromatic() {
            if bond.is(":") {
                return IMPLICT;
            }
            return bond;
        } else {
            if bond.is(":") {
                return AROMATIC;
            }
            return IMPLICT;
        }
    }
    bond
}

pub fn implicit_to_explicit(mol: &Molecule) -> Result<Molecule> {
    let mut result = Molecule::new();

    for &idx in mol.atoms() {
        let atom = mol.atom_at(&idx)?;
        result.add_atom(atom.clone())?;
    }

    // Copy topologies
    for &idx in mol.atoms() {
        if let Some(topo) = mol.topology_at(&idx) {
            let conf = topo.configuration().unwrap_or(crate::molecule::UNKNOWN);
            let vs = match topo.seq() {
                crate::molecule::topology::TopologySeq::Tetrahedral => {
                    vec![idx as i8, idx as i8, idx as i8, idx as i8]
                }
                crate::molecule::topology::TopologySeq::Trigonal => {
                    let mut vs = Vec::new();
                    for n in mol.graph().neighbors(&idx)? {
                        if mol.edge_at(idx, *n)?.is("=") {
                            vs.insert(0, *n as i8);
                        } else {
                            vs.push(*n as i8);
                        }
                    }
                    if vs.len() >= 3 {
                        vec![vs[0], vs[1], vs[2]]
                    } else {
                        vec![0, idx as i8, idx as i8]
                    }
                }
                _ => vec![],
            };
            if let Ok(new_topo) = crate::molecule::topology::create(idx, conf, vs) {
                result.add_topology(new_topo);
            }
        }
    }

    for &idx in mol.atoms() {
        for neighbor in mol.graph().neighbors(&idx)? {
            if *neighbor > idx {
                if let Ok(bond) = mol.edge_at(idx, *neighbor) {
                    let explicit_bond =
                        to_explicit_bond(mol.atom_at(&idx)?, mol.atom_at(neighbor)?, *bond);
                    result.add_bond(idx, *neighbor, explicit_bond)?;
                }
            }
        }
    }

    result.set_flags(mol.get_flag(0xFF));

    run_post_processing(&mut result)?;

    Ok(result)
}

fn to_explicit_bond(u: &Atom, v: &Atom, bond: Bond) -> Bond {
    if bond.is("") {
        if u.is_aromatic() && v.is_aromatic() {
            return AROMATIC;
        }
        return SINGLE;
    }
    bond
}

fn run_post_processing(mol: &mut Molecule) -> Result<()> {
    mol.rings_detection()?;
    mol.aromaticity_detection()?;
    mol.symmetry_detection()?;
    mol.stereocenter_detection()?;
    Ok(())
}

fn to_subset_atoms(mol: &Molecule) -> Result<Molecule> {
    let mut result = Molecule::new();

    for &idx in mol.atoms() {
        let atom = mol.atom_at(&idx)?;

        let new_atom = to_subset(atom, mol, idx)?;
        result.add_atom(new_atom)?;
    }

    for &idx in mol.atoms() {
        for neighbor in mol.graph().neighbors(&idx)? {
            if *neighbor > idx {
                if let Ok(bond) = mol.edge_at(idx, *neighbor) {
                    result.add_bond(idx, *neighbor, *bond)?;
                }
            }
        }
    }

    result.set_flags(mol.get_flag(0xFF));

    run_post_processing(&mut result)?;

    Ok(result)
}

fn to_subset(atom: &Atom, mol: &Molecule, idx: u8) -> Result<Atom> {
    if atom.is_aliphatic() || atom.is_aromatic() {
        return Ok(atom.clone());
    }

    if !atom.element().organic() {
        return Ok(atom.clone());
    }

    if atom.charge() != 0 || atom.isotope() >= 0 {
        return Ok(atom.clone());
    }

    let explicit_h = atom.explicit_hydrogens();
    let valence = mol.valence(&idx)?;
    let implied_h = atom.element().implicit_hydrogen_count(valence as i32);

    if explicit_h == implied_h as u8 {
        if atom.is_aromatic() {
            return Ok(Atom::new_aromatic(atom.element(), atom.is_organogen()));
        } else {
            return Ok(Atom::new_aliphatic(atom.element(), atom.is_organogen()));
        }
    }

    Ok(atom.clone())
}

fn from_subset_atoms(mol: &Molecule) -> Result<Molecule> {
    let mut result = Molecule::new();

    for &idx in mol.atoms() {
        let atom = mol.atom_at(&idx)?;
        let valence = mol.valence(&idx)?;

        let new_atom = from_subset(atom, valence);
        result.add_atom(new_atom)?;
    }

    for &idx in mol.atoms() {
        for neighbor in mol.graph().neighbors(&idx)? {
            if *neighbor > idx {
                if let Ok(bond) = mol.edge_at(idx, *neighbor) {
                    result.add_bond(idx, *neighbor, *bond)?;
                }
            }
        }
    }

    result.set_flags(mol.get_flag(0xFF));

    run_post_processing(&mut result)?;

    Ok(result)
}

fn from_subset(atom: &Atom, valence: u8) -> Atom {
    if atom.is_aliphatic() || atom.is_aromatic() {
        if atom.is_aromatic() {
            let implicit_h = atom
                .element()
                .implicit_aromatic_hydrogen_count(valence as i32);
            if implicit_h == 0 {
                return atom.clone();
            }
            return Atom::new_bracket(
                atom.element(),
                -1,
                implicit_h as u8,
                0,
                true,
                atom.is_organogen(),
            );
        } else {
            let implicit_h = atom.element().implicit_hydrogen_count(valence as i32);
            if implicit_h == 0 {
                return atom.clone();
            }
            return Atom::new_bracket(
                atom.element(),
                -1,
                implicit_h as u8,
                0,
                false,
                atom.is_organogen(),
            );
        }
    }

    atom.clone()
}
