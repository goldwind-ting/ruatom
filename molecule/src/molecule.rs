use crate::configuration::*;
use crate::{
    atom::AtomKind,
    bond::{Bond, IMPLICT},
    element::Specification,
    error::{MoleculeError, Result},
    topology::Topology,
    Atom, RingBond,
};
use graph::{self, Edge, Graph, GraphError};
use hashbrown::HashMap;

pub struct Molecule {
    graph: Graph<Atom, Bond>,
    atoms: Vec<u8>,
    ring_num: u8,
    ring_bonds: HashMap<u8, RingBond>,
    flag: u8,
    valences: HashMap<u8, u8>,
    topologies: HashMap<u8, Box<dyn Topology>>,
    ssr: u16,
}

impl Molecule {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            atoms: Vec::new(),
            ring_num: 0,
            ring_bonds: HashMap::new(),
            flag: 0,
            valences: HashMap::new(),
            topologies: HashMap::new(),
            ssr: 0
        }
    }

    pub fn add_atom(&mut self, atom: Atom) -> Result<u8> {
        let index = self.atoms.len() as u8;
        self.graph.add_vertex(index, atom)?;
        self.atoms.push(index);
        Ok(index)
    }

    pub fn add_bond(&mut self, u: u8, v: u8, bond: Bond) -> Result<bool> {
        let ok = self.graph.add_edge(u, v, bond).and_then(|ok| Ok(ok))?;
        let eu = self.valences.entry(u).or_insert(0);
        *eu += bond.electron();
        let ev = self.valences.entry(v).or_insert(0);
        *ev += bond.electron();
        Ok(ok)
    }

    pub fn open_ring(&mut self, rloc: u8, bond: Bond, pos: Option<u8>, u: u8) {
        if self.ring_bonds.contains_key(&rloc) {
            panic!()
        }
        let rb = RingBond::new(bond, u, pos);
        self.ring_bonds.insert(rloc, rb);
        self.ring_num += 1;
    }

    pub fn close_ring(&mut self, rloc: u8, u: u8, sbond: Bond) -> Result<u8> {
        let rb = self.ring_bonds.remove(&rloc);
        if let Some(rb) = rb {
            if self.graph.adjancent(rb.vertex(), u) {
                return Err(MoleculeError::IlleageAdjacentVertix);
            }
            let bond = self.decide_bond(sbond.inverse(), rb.bond())?;
            self.graph.add_edge(rb.vertex(), u, bond)?;
            let eu = self.valences.entry(rb.vertex()).or_insert(0);
            *eu += bond.electron();
            let ev = self.valences.entry(u).or_insert(0);
            *ev += bond.electron();
            self.ring_num -= 1;
            self.ssr == 1;
            return Ok(rb.vertex());
        }
        return Err(MoleculeError::InvalidRingBond);
    }
    pub fn ring_num(&self) -> u8 {
        self.ring_num
    }

    fn degree(&self, u: u8) -> Result<u8> {
        let deg = self.graph.outbound_count(&u)? + self.graph.inbound_count(&u)?;
        Ok(deg as u8)
    }

    pub fn is_open(&self, rloc: u8) -> bool {
        !self.ring_bonds.contains_key(&rloc) && rloc > self.ring_bonds.len() as u8
    }

    pub fn decide_bond(&mut self, a: Bond, b: Bond) -> Result<Bond> {
        if a == b {
            return Ok(a);
        } else if a == IMPLICT {
            return Ok(b);
        } else if b == IMPLICT {
            return Ok(a);
        }
        if b.inverse() != a {
            return Err(MoleculeError::IllegalMolecule("ring bond not match"));
        };
        return Ok(IMPLICT);
    }

    pub fn set_flags(&mut self, flag: u8) {
        self.flag |= flag;
    }
    pub fn get_flag(&self, mask: u8) -> u8 {
        self.flag & mask
    }

    pub fn add_topology(&mut self, t: Box<dyn Topology>) {
        if t.atom() != -1 {
            self.topologies.insert(t.atom() as u8, t);
        }
    }

    pub fn hydrogen_count(&self, loc: u8) -> Result<u8> {
        let atom = self.graph.vertex(&loc)?;
        let init_count = match atom.kind() {
            AtomKind::Bracket(_) => atom.hydrogens(),
            _ => 0,
        };
        let valence: u8;
        if self.valences.len() < 1 {
            valence = init_count + 0;
        } else {
            valence = init_count + self.bond_venlences(loc)?;
        }

        if atom.is_aromatic() && self.degree(loc)? == self.bond_venlences(loc)? {
            return Ok(atom.implict_hydrogen_amount(valence + 1));
        }
        return Ok(atom.implict_hydrogen_amount(valence));
    }

    pub fn trans_astrix_atom(&mut self) -> Result<()> {
        // add unit test
        let atoms = self.atoms.clone();
        for a in atoms.iter() {
            let atom = self.graph.vertex(a)?;
            if atom.ele_is_any() {
                let mut n_arom = 0;
                if self.order() < 2 {
                    return Ok(());
                }
                self.graph.in_neighbors(a).and_then(|vs| {
                    for v in vs {
                        if self.graph.edge_with_vertex(*a, *v)?.is_aromatic()
                            || self.graph.vertex(v)?.is_aromatic()
                        {
                            n_arom += 1;
                        }
                    }
                    Ok(())
                })?;
                self.graph.out_neighbors(a).and_then(|vs| {
                    for v in vs {
                        if self.graph.edge_with_vertex(*v, *a)?.is_aromatic()
                            || self.graph.vertex(v)?.is_aromatic()
                        {
                            n_arom += 1;
                        }
                    }
                    Ok(())
                })?;
                if n_arom > 2 {
                    if atom.is_aliphatic() {
                        let arom_atom = atom
                            .to_aromatic(Specification::OpenSMILES)
                            .ok_or(MoleculeError::TransformError)?;
                        self.update_atom(*a, arom_atom);
                    } else if atom.is_bracket_atom() {
                        let bracket_atom =
                            atom.to_bracket().ok_or(MoleculeError::TransformError)?;
                        self.update_atom(*a, bracket_atom);
                    }
                }
            }
        }
        Ok(())
    }

    fn bond_venlences(&self, u: u8) -> Result<u8> {
        let v = self
            .valences
            .get(&u)
            .ok_or(graph::GraphError::NoSuchVertex(u))?;
        Ok(*v)
    }

    fn update_atom(&mut self, loc: u8, atom: Atom) {
        self.graph.update_vertex(loc, atom);
    }

    fn find_double_bond(&self, u: u8, v: u8) -> Result<i8> {
        let mut another = -1;
        let inn = self.graph.in_neighbors(&u);
        let outn = self.graph.out_neighbors(&u);
        if inn.is_err() && outn.is_err() {
            return Err(MoleculeError::GraphError(GraphError::NoSuchVertex(u)));
        }
        if inn.is_ok() {
            for atom in inn.unwrap() {
                let other = *atom;
                let bond = self.graph.edge_with_vertex(other, u)?;
                if bond.is("=") && other != v {
                    another = other as i8;
                    break;
                }
            }
        }
        if another != -1 {
            return Ok(another);
        }
        if outn.is_ok() {
            for atom in outn.unwrap() {
                let other = *atom;
                let bond = self.graph.edge_with_vertex(u, other)?;
                if bond.is("=") && other != v {
                    another = other as i8;
                    break;
                }
            }
        }
        return Ok(another);
    }

    pub fn find_extend_tetrahedral_ends(&self, u: u8) -> Result<Vec<u8>> {
        if self.degree(u)? < 2 {
            return Err(MoleculeError::IllegalMolecule("invalid atom num"));
        }
        let mut nei = self.graph.neighbors(&u)?;
        let mut pre_e1 = u;
        let mut pre_e2 = u;
        let mut e1 = (nei.next().unwrap()) as i8;
        let mut e2 = (nei.next().unwrap()) as i8;
        let mut tmp: i8;
        while e1 >= 0 && e2 >= 0 {
            tmp = self.find_double_bond(e1 as u8, pre_e1)?;
            pre_e1 = e1 as u8;
            e1 = tmp;
            tmp = self.find_double_bond(e2 as u8, pre_e2)?;
            pre_e2 = e2 as u8;
            e2 = tmp;
        }
        Ok(vec![pre_e1, pre_e2])
    }

    pub fn edge_at(&self, u: u8, v: u8) -> Result<Bond> {
        if let Ok(inb) = self.graph.edge_with_vertex(u, v) {
            return Ok(inb.clone().to_owned());
        }
        if let Ok(outb) = self.graph.edge_with_vertex(v, u) {
            return Ok(outb.clone().to_owned());
        }
        return Err(MoleculeError::GraphError(GraphError::NoSuchEdge(u, v)));
    }

    pub fn atom_at(&self, u: &u8) -> Result<Atom> {
        let a = self.graph.vertex(u)?.clone().to_owned();
        Ok(a)
    }

    pub fn to_explict_configuration(
        // add unit test
        &self,
        atom: u8,
        conf: &Configuration,
    ) -> Result<Configuration> {
        if !conf.is_implict() {
            return Ok(conf.clone());
        }
        let deg = self.degree(atom)?;
        let venlences = deg + self.hydrogen_count(atom)?;
        if deg == 2 {
            let mut dc = 0;
            self.graph.map_edge(&atom, |bond, _| {
                if !bond.is("=") {
                    dc += 1;
                }
            })?;
            match dc {
                1 => {
                    if conf.is_anti_clockwise() {
                        return Ok(DB1);
                    }
                    return Ok(DB2);
                }
                _ => {
                    if conf.is_anti_clockwise() {
                        return Ok(AL1);
                    }
                    return Ok(AL2);
                }
            }
        }
        match venlences {
            4 => {
                if conf.is_anti_clockwise() {
                    return Ok(TH1);
                }
                return Ok(TH2);
            }
            3 => {
                let am = self.graph.vertex(&atom)?;
                match am.element().symbol() {
                    "S" | "Se" => {
                        let mut sc = 0;
                        let mut dc = 0;
                        self.graph.map_edge(&atom, |bond, _| {
                            let elec = bond.electron();
                            if elec == 1 {
                                sc += 1;
                            } else if elec == 2 {
                                dc += 1;
                            }
                        })?;
                        let charge = am.charge();
                        if charge == 0 && sc == 2 && dc == 1 || charge == 1 && sc == 3 {
                            if conf.is_anti_clockwise() {
                                return Ok(TH1);
                            } else {
                                return Ok(TH2);
                            }
                        }
                    }
                    "P" | "N" => {
                        if self.valences.get(&atom) == Some(&3)
                            && am.charge() == 0
                            && self.hydrogen_count(atom)? == 0
                        {
                            if conf.is_anti_clockwise() {
                                return Ok(TH1);
                            }
                            return Ok(TH2);
                        }
                    }
                    _ => {}
                }
                let mut dc = 0;
                self.graph.map_edge(&atom, |bond, _| {
                    if bond.is("=") {
                        dc += 1;
                    }
                })?;
                match dc {
                    1 => {
                        if conf.is_anti_clockwise() {
                            return Ok(DB1);
                        }
                        return Ok(DB2);
                    }
                    _ => {
                        return Ok(UNKNOWN);
                    }
                }
            }
            5 => {
                if conf.is_anti_clockwise() {
                    return Ok(TB1);
                }
                return Ok(TB2);
            }
            6 => {
                if conf.is_anti_clockwise() {
                    return Ok(OH1);
                }
                return Ok(OH2);
            }
            _ => {
                return Ok(UNKNOWN);
            }
        }
    }

    pub fn topology_at(&self, loc: &u8) -> Option<&Box<dyn Topology>> {
        self.topologies.get(loc)
    }

    pub fn validate_up_down(&self, directional_bonds: HashMap<u8, bool>) -> Result<()> {
        for v in directional_bonds.keys() {
            let mut n_up_v = 0;
            let mut n_down_v = 0;
            let mut n_up_w = 0;
            let mut n_down_w = 0;
            let mut w: i8 = -1;
            self.graph.map_edge(v, |bond, v| {
                if bond.is("/") {
                    n_up_v += 1;
                } else if bond.is("\\") {
                    n_down_v += 1;
                } else if bond.is("=") {
                    w = *v as i8;
                }
            })?;
            if w < 0 {
                continue;
            }
            self.graph.map_edge(&(w as u8), |bond, _| {
                if bond.is("/") {
                    n_up_w += 1;
                } else if bond.is("\\") {
                    n_down_w += 1;
                }
            })?;
            if n_up_v + n_down_v == 0 || n_up_w + n_down_w == 0 {
                continue;
            }
            if n_up_v > 1 || n_down_v > 1 || n_up_w > 1 || n_down_w > 1 {
                return Err(MoleculeError::IllegalMolecule(
                    "invalid Cis/Trans specification",
                ));
            }
        }
        return Ok(());
    }

    #[inline]
    pub fn order(&self) -> usize {
        return self.graph.order();
    }

    #[inline]
    pub fn size(&self) -> usize {
        return self.graph.size();
    }

    pub fn map_bonds<Func>(&self, f: Func) -> Result<()>
    where
        Func: FnMut(&Edge, &Bond),
    {
        self.graph
            .map_edges(f)
            .map_err(|e| MoleculeError::GraphError(e))
    }

    pub fn molecule_weight(&self) -> Result<f64> {
        let mut res = 0.0;
        for i in self.atoms.iter() {
            let at = self.atom_at(i)?;
            if at.is("H") || at.is("D") || at.is("T") {
                continue;
            } else {
                res += self.atom_at(i)?.get_mass()?;
            }
        }
        res += self.total_hs()? as f64;
        Ok(res)
    }

    pub fn molecule_formula(&self) -> &str {
        "a"
    }

    pub fn ssr(&self) -> u16 {
        return self.ssr;
    }

    pub fn total_hs(&self) -> Result<u8> {
        let mut hs = 0;
        for i in self.atoms.iter() {
            let at = self.atom_at(i)?;
            if at.is("H") || at.is("D") || at.is("T") {
                hs += 1;
            } else {
                hs += self.hydrogen_count(*i)?;
            }
        }
        Ok(hs)
    }
}

#[test]
fn test_degree() {
    let mut m = Molecule::new();
    let c1 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    assert_eq!(c1, 0);
    let c2 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    assert_eq!(c2, 1);
    let o = m.add_atom(Atom::new_aliphatic(crate::element::O)).unwrap();
    assert_eq!(o, 2);
    assert_eq!(m.order(), 3);
    assert!(m.add_bond(c1, c2, crate::bond::SINGLE).unwrap());
    assert!(m.add_bond(c2, o, crate::bond::SINGLE).unwrap());
    assert_eq!(m.hydrogen_count(c1).unwrap(), 3);
    assert_eq!(m.hydrogen_count(c2).unwrap(), 2);
    assert_eq!(m.hydrogen_count(o).unwrap(), 1);
    assert_eq!(m.degree(c1).unwrap(), 1);
    assert_eq!(m.degree(c2).unwrap(), 2);
    assert_eq!(m.degree(o).unwrap(), 1);
}

#[test]
fn test_find_double_bond() {
    let mut m = Molecule::new();
    let c1 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    let c2 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    let c3 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    let c4 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    assert!(m.add_bond(c1, c2, crate::bond::SINGLE).unwrap());
    assert!(m.add_bond(c2, c3, crate::bond::DOUBLE).unwrap());
    assert!(m.add_bond(c3, c4, crate::bond::SINGLE).unwrap());
    assert_eq!(m.find_double_bond(c2, c1).unwrap(), 2);
    assert_eq!(m.find_double_bond(c2, c3).unwrap(), -1);
}

#[test]
fn test_bond_venlence() {
    let mut m = Molecule::new();
    let c1 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    let c2 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    let c3 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    let c4 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    assert!(m.add_bond(c1, c2, crate::bond::SINGLE).unwrap());
    assert!(m.add_bond(c2, c3, crate::bond::DOUBLE).unwrap());
    assert!(m.add_bond(c3, c4, crate::bond::SINGLE).unwrap());
    assert_eq!(m.bond_venlences(c2).unwrap(), 3);
    assert_eq!(m.bond_venlences(c1).unwrap(), 1);
    assert_eq!(m.bond_venlences(c3).unwrap(), 3);
    assert_eq!(m.bond_venlences(c4).unwrap(), 1);
}

#[test]
fn test_hs() {
    let mut m = Molecule::new();
    let c1 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    let c2 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    let c3 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    let c4 = m.add_atom(Atom::new_aliphatic(crate::element::C)).unwrap();
    assert!(m.add_bond(c1, c2, crate::bond::SINGLE).unwrap());
    assert!(m.add_bond(c2, c3, crate::bond::DOUBLE).unwrap());
    assert!(m.add_bond(c3, c4, crate::bond::SINGLE).unwrap());
    assert_eq!(m.total_hs().unwrap(), 8);
}
