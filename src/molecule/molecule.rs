use super::{
    atom::AtomKind,
    bond::{Bond, IMPLICT},
    canon::{prime, rank, rank_matrix},
    element::{valid_element_symbol, Specification},
    leftpad_with,
    topology::Topology,
    Atom, RingBond,
};
use super::{configuration::*, H};
use crate::error::{Result, RuatomError};
use crate::graph::{Edge, Graph};
use hashbrown::HashMap;
use phf::phf_set;
use std::num::ParseIntError;

static RING_SIZE: phf::Set<u8> = phf_set! {
    5u8,
    6u8,
    7u8
};

pub struct Molecule {
    graph: Graph<Atom, Bond>,
    atoms: Vec<u8>,
    ring_num: u8,
    ring_bonds: HashMap<u8, RingBond>,
    flag: u8,
    valences: HashMap<u8, u8>,
    topologies: HashMap<u8, Box<dyn Topology>>,
    n_ssr: u16,
    bonds: Vec<[u8; 2]>,
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
            n_ssr: 0,
            bonds: Vec::new(),
        }
    }

    pub fn add_atom(&mut self, atom: Atom) -> Result<u8> {
        let index = self.atoms.len() as u8 + 1;
        self.graph.add_vertex(index, atom)?;
        self.valences.insert(index, 0);
        self.atoms.push(index);
        Ok(index)
    }

    pub fn add_bond(&mut self, u: u8, v: u8, bond: Bond) -> Result<bool> {
        let ok = self.graph.add_edge(u, v, bond).and_then(|ok| Ok(ok))?;
        let eu = self.valences.entry(u).or_insert(0);
        *eu += bond.electron();
        let ev = self.valences.entry(v).or_insert(0);
        *ev += bond.electron();
        self.graph.vertex_mut(&u)?.incr_degree(bond.electron());
        self.graph.vertex_mut(&v)?.incr_degree(bond.electron());
        self.bonds.push([u, v]);
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
                return Err(RuatomError::IlleageAdjacentVertix);
            }
            let bond = self.decide_bond(sbond.inverse(), rb.bond())?;
            self.graph.add_edge(rb.vertex(), u, bond)?;
            self.valences
                .entry(rb.vertex())
                .and_modify(|e| *e += bond.electron());
            self.valences.entry(u).and_modify(|e| *e += bond.electron());
            self.graph
                .vertex_mut(&rb.vertex())?
                .incr_degree(bond.electron());
            self.graph.vertex_mut(&u)?.incr_degree(bond.electron());
            self.bonds.push([u, rb.vertex()]);
            self.ring_num -= 1;
            self.n_ssr += 1;
            return Ok(rb.vertex());
        }
        return Err(RuatomError::InvalidRingBond);
    }
    pub fn ring_num(&self) -> u8 {
        self.ring_num
    }

    fn degree(&self, u: &u8) -> Result<u8> {
        let deg = self.graph.bound_count(u)?;
        Ok(deg as u8)
    }

    pub fn enable_open(&self, rloc: u8) -> bool {
        !self.ring_bonds.contains_key(&rloc)
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
            return Err(RuatomError::IllegalMolecule("ring bond not match"));
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

    pub fn valence(&self, loc: &u8) -> Result<u8> {
        let atom = self.graph.vertex(&loc)?;
        let init_count = match atom.kind() {
            AtomKind::Bracket(_) => atom.hydrogens(),
            _ => 0,
        };
        let valence: u8;
        if self.valences.len() < 1 {
            valence = init_count + 0;
        } else {
            valence = init_count + self.bond_valences(loc)?;
        }
        if atom.is_aromatic() && self.degree(&loc)? == self.bond_valences(loc)? {
            return Ok(valence + 1);
        }
        return Ok(valence);
    }

    pub fn hydrogen_count(&self, loc: &u8) -> Result<u8> {
        let atom = self.graph.vertex(&loc)?;
        return Ok(atom.implict_hydrogen_amount(self.valence(loc)?));
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
                self.graph.neighbors(a).and_then(|vs| {
                    for v in vs {
                        if self.graph.edge_with_vertex(*a, *v)?.is_aromatic()
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
                            .ok_or(RuatomError::TransformError)?;
                        self.update_atom(*a, arom_atom);
                    } else if atom.is_bracket_atom() {
                        let bracket_atom = atom.to_bracket().ok_or(RuatomError::TransformError)?;
                        self.update_atom(*a, bracket_atom);
                    }
                }
            }
        }
        Ok(())
    }

    fn bond_valences(&self, u: &u8) -> Result<u8> {
        let v = self.valences.get(u).ok_or(RuatomError::NoSuchVertex(*u))?;
        Ok(*v)
    }

    fn update_atom(&mut self, loc: u8, atom: Atom) {
        self.graph.update_vertex(loc, atom);
    }

    fn find_double_bond(&self, u: u8, v: u8) -> Result<i8> {
        let mut another = -1;
        let neighbors = self.graph.neighbors(&u);
        if neighbors.is_err() {
            return Err(RuatomError::NoSuchVertex(u));
        }
        if neighbors.is_ok() {
            for atom in neighbors.unwrap() {
                let other = *atom;
                let bond = self.graph.edge_with_vertex(other, u)?;
                if bond.is("=") && other != v {
                    another = other as i8;
                    break;
                }
            }
        }
        return Ok(another);
    }

    pub fn find_extend_tetrahedral_ends(&self, u: u8) -> Result<Vec<u8>> {
        if self.degree(&u)? < 2 {
            return Err(RuatomError::IllegalMolecule("invalid atom num"));
        }
        let mut nei = self.graph.neighbors(&u)?;
        let mut pre_e1 = u;
        let mut pre_e2 = u;
        let mut e1 = *(nei.next().unwrap()) as i8;
        let mut e2 = *(nei.next().unwrap()) as i8;
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

    pub fn edge_at(&self, u: u8, v: u8) -> Result<&Bond> {
        let b = self.graph.edge_with_vertex(u, v)?;
        return Ok(b);
    }

    pub(crate) fn edge_mut(&mut self, u: u8, v: u8) -> Result<&mut Bond> {
        let e = Edge::new(u, v);
        let b = self.graph.edge_mut(&e)?;
        return Ok(b);
    }

    pub fn atom_at(&self, u: &u8) -> Result<&Atom> {
        let a = self.graph.vertex(u)?;
        Ok(a)
    }

    pub fn atom_mut(&mut self, loc: &u8) -> Result<&mut Atom> {
        self.graph.vertex_mut(&loc)
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
        let deg = self.degree(&atom)?;
        let venlences = deg + self.hydrogen_count(&atom)?;
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
                            && self.hydrogen_count(&atom)? == 0
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
                return Err(RuatomError::IllegalMolecule(
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
        self.graph.map_edges(f).map_err(|e| e)
    }

    pub fn molecule_weight(&self) -> Result<f64> {
        let mut res = 0.0;
        for i in self.atoms.iter() {
            let at = self.atom_at(i)?;
            if at.is("H") {
                continue;
            } else {
                res += self.atom_at(i)?.get_mass();
            }
        }
        res += self.total_hs(true)? as f64 * H.get_mass();
        Ok(res)
    }

    pub fn exact_molecule_weight(&self) -> Result<f64> {
        let mut res = 0.0;
        for i in self.atoms.iter() {
            let at = self.atom_at(i)?;
            if at.is("H") && at.isotope() < 0 {
                continue;
            } else {
                res += self.atom_at(i)?.get_exact_mass()?;
            }
        }
        res += self.total_hs(false)? as f64 * H.get_exact_mass(-1)?;
        Ok(res)
    }

    pub fn heavy_atom_amount(&self, symbol: &str) -> Result<u16> {
        let mut amount = 0;
        if !valid_element_symbol(symbol) {
            return Err(RuatomError::NotFoundSymbolError(symbol.to_string()));
        }
        for at in self.atoms.iter() {
            if self.atom_at(at)?.is(symbol) {
                amount += 1;
            }
        }
        return Ok(amount);
    }

    pub fn n_ssr(&self) -> u16 {
        return self.n_ssr;
    }

    pub fn total_hs(&self, isotope: bool) -> Result<u8> {
        let mut hs = 0;
        for i in self.atoms.iter() {
            let at = self.atom_at(i)?;
            if at.is("H") {
                if isotope || !isotope && at.isotope() < 0 {
                    hs += 1;
                } else {
                    continue;
                }
            } else {
                hs += self.hydrogen_count(i)?;
            }
        }
        Ok(hs)
    }

    #[inline]
    fn connectivity(&self, loc: &u8) -> Result<u8> {
        let con = self.bond_degree_of(&loc)? + self.hydrogen_count(loc)?;
        Ok(con)
    }

    pub(crate) fn ring_size_of(&self, a: u8, b: u8) -> Result<u8> {
        let mut distance = 1;
        let mut visited: Vec<usize> = vec![0; self.atoms.len() + 1];
        let mut queue = vec![a, 0];
        let mut ix = 0;
        let mut one = 0;
        while ix <= queue.len() {
            one = queue[ix];
            ix += 1;
            if one == 0 {
                queue.push(0);
                distance += 1;
                one = queue[ix];
                ix += 1;
            }
            if one == b || one == 0 {
                break;
            }
            visited[one as usize] = 1;
            for j in self.graph.neighbors(&one)? {
                let cj = *j;
                if one == a && cj == b {
                    continue;
                }
                if self.atom_at(j)?.ring_membership() == 1
                    && self.edge_at(one, cj)?.electron() > 0
                    && visited[cj as usize] == 0
                {
                    queue.push(cj);
                }
            }
        }
        if one == b {
            return Ok(distance);
        }
        return Ok(0);
    }

    pub fn rings_detection(&mut self) -> Result<()> {
        let atoms = self.atoms.clone();
        for atom in atoms.iter() {
            if self.bond_degree_of(&atom)? >= 2 {
                self.atom_mut(atom).unwrap().set_membership(1);
            }
        }
        let bonds = self.bonds.clone();
        for b in bonds.iter() {
            let rs = self.ring_size_of(b[0], b[1])?;
            let bond = self.edge_mut(b[0], b[1])?;
            bond.set_ring_membership(1);
            bond.set_ring_size(rs);
            let bond = self.edge_mut(b[1], b[0])?;
            bond.set_ring_membership(1);
            bond.set_ring_size(rs);
        }
        for atom in atoms.iter() {
            let g = self.graph.clone();
            if self.atom_at(atom)?.ring_membership() == 1 {
                self.update_atom_ring_info(atom, g)?;
            }
        }
        return Ok(());
    }

    fn update_atom_ring_info(&mut self, loc: &u8, graph: Graph<Atom, Bond>) -> Result<()> {
        let nei = graph.neighbors(loc)?;
        let mut n = self.atoms.len() as u8;
        for j in nei {
            let rm = self.edge_at(*loc, *j)?.ring_membership();
            self.atom_mut(loc)?.incr_ring_connectivity(rm);
            let rs = self.edge_at(*loc, *j)?.ring_size();
            if rs < n {
                n = rs;
            }
            if rs > self.atom_at(loc)?.max_bonds_ringsize() {
                self.atom_mut(loc)?.set_max_bonds_ringsize(rs);
            }
        }
        if self.atoms.len() > n as usize {
            self.atom_mut(loc)?.set_ring_size(n);
        };
        let con = self.atom_at(loc)?.ring_connectivity();
        self.atom_mut(loc)?.set_membership(con);
        return Ok(());
    }

    pub fn init_rank(&self, loc: &u8) -> Result<usize> {
        let atom = self.atom_at(loc)?;
        let mut irank = String::from("");
        irank.push_str(&self.degree(&loc)?.to_string());
        irank.push_str(&leftpad_with(
            atom.element().atomic_number().to_string(),
            3,
            '0',
        ));
        irank.push_str(&self.hydrogen_count(&loc)?.to_string());
        let charge = self.atom_at(&loc)?.charge();
        if charge >= 0 {
            irank.push('0');
            irank.push_str(&charge.to_string());
        } else {
            irank.push('1');
            irank.push_str(&(0 - charge).to_string());
        }
        irank.push_str(&self.connectivity(&loc)?.to_string());
        irank.push_str(&self.valence(&loc)?.to_string());
        irank.push_str(&leftpad_with(
            self.atom_at(&loc)?.get_mass().floor().to_string(),
            3,
            '0',
        ));
        irank.push_str(&self.atom_at(&loc)?.chirality().to_string());
        Ok(irank
            .parse()
            .map_err(|e: ParseIntError| RuatomError::StdError(e.to_string()))?)
    }

    pub fn bond_degree_of(&self, loc: &u8) -> Result<u8> {
        let deg = self.atom_at(loc)?.bond_degree();
        Ok(deg)
    }

    pub fn distance_count(&self, loc: &u8) -> Result<usize> {
        if self.atom_at(&loc)?.ring_connectivity() == 0 {
            return Ok(1);
        };
        let mut distance: usize = 0;
        let mut level = 0;
        let mut visited: Vec<usize> = vec![0; self.atoms.len() + 1];
        let mut queue = vec![*loc, 0];
        visited[*loc as usize] = 1;
        let mut ix = 0;
        while ix <= queue.len() {
            let mut one = queue[ix];
            ix += 1;
            if one == 0 {
                queue.push(0);
                level += 1;
                one = queue[ix];
                ix += 1;
            }
            if one == 0 {
                break;
            }
            distance = distance + 10_usize.pow(level);
            for j in self.graph.neighbors(&one)? {
                let cj = *j;
                if self.edge_at(one, cj)?.ring_membership() > 0 && visited[cj as usize] == 0 {
                    queue.push(cj);
                    visited[cj as usize] = 1;
                }
            }
        }
        return Ok((distance - 1) / 10);
    }

    pub fn aromaticity_detection(&mut self) -> Result<()> {
        let mut sp2atoms: HashMap<u8, u8> = HashMap::new();
        for atom in self.atoms.iter() {
            sp2atoms.insert(*atom, 0);
            let at = self.atom_at(atom)?;
            let hc = self.hydrogen_count(atom)?;
            if at.is_aromatic() {
                sp2atoms.entry(*atom).and_modify(|e| *e = 1);
            }
            if !at.ele_is_any()
                && at.ring_membership() > 0
                && (RING_SIZE.contains(&at.ring_size())
                    || RING_SIZE.contains(&at.max_bonds_ringsize()))
                && Molecule::maybe_aromaticity(at, hc)
            {
                let mut count = 0;
                for j in self.graph.neighbors(atom)? {
                    if self.edge_at(*atom, *j)?.electron() == 2 {
                        count += 1;
                    }
                }
                if count == 1 {
                    sp2atoms.entry(*atom).and_modify(|e| *e = 1);
                }
            }
        }
        for atom in self.atoms.iter() {
            let at = self.atom_at(atom)?;
            let exist = sp2atoms.get(atom).unwrap();
            if exist == &0
                && !at.ele_is_any()
                && at.ring_membership() > 0
                && (RING_SIZE.contains(&at.ring_size())
                    || RING_SIZE.contains(&at.max_bonds_ringsize()))
                && Molecule::maybe_aromaticity_nonsp2(at)
            {
                let mut count = 0;
                for j in self.graph.neighbors(atom)? {
                    if self.edge_at(*atom, *j)?.electron() == 1 {
                        count += sp2atoms.get(j).unwrap();
                    }
                }
                if count >= 2 {
                    sp2atoms.entry(*atom).and_modify(|e| *e = 1);
                }
            }
        }
        loop {
            let mut flag = 0;
            for atom in self.atoms.iter() {
                if sp2atoms.get(atom).unwrap() == &1 {
                    let mut count = 0;
                    for j in self.graph.neighbors(atom)? {
                        if self.edge_at(*atom, *j)?.electron() == 1 {
                            count += sp2atoms.get(j).unwrap();
                        }
                    }
                    if count < 2 {
                        sp2atoms.entry(*atom).and_modify(|e| *e = 0);
                        flag = 1;
                    }
                }
            }
            if flag == 0 {
                break;
            }
        }
        let atoms = self.atoms.clone();
        let g = self.graph.clone();
        for atom in atoms.iter() {
            if sp2atoms.get(atom).unwrap() == &1 {
                let nei = g.neighbors(atom)?;
                for j in nei {
                    let bond = self.edge_mut(*atom, *j)?;
                    if sp2atoms.get(j).unwrap() == &1 && bond.electron() == 2 {
                        bond.to_single();
                    }
                }
                self.atom_mut(atom)?.set_aromatic();
            }
        }
        Ok(())
    }

    fn maybe_aromaticity(atom: &Atom, hc: u8) -> bool {
        match atom.element().atomic_number() {
            5 => atom.is_organogen(),
            6 => atom.is_organogen() || hc == 0 && (atom.charge() == 1 || atom.charge() == -1),
            7 | 15 => atom.is_organogen() || atom.charge() == 1,
            8 | 16 | 34 | 52 => atom.charge() == 1,
            33 => hc == 0 && atom.charge() == 0,
            _ => false,
        }
    }

    fn maybe_aromaticity_nonsp2(atom: &Atom) -> bool {
        match atom.element().atomic_number() {
            6 => atom.charge() == -1,
            7 | 15 => atom.is_organogen() || atom.charge() == -1,
            8 | 16 => atom.is_organogen(),
            33 | 34 | 52 => atom.charge() == 0,
            _ => false,
        }
    }

    pub fn symmetry_detection(&mut self) -> Result<()> {
        let mut inv: Vec<[usize; 3]> = Vec::new();
        let atoms = self.atoms.clone();
        for atom in atoms.iter() {
            let mut ring_invariant = 1;
            for j in self.graph.neighbors(atom)? {
                let rs = self.edge_at(*atom, *j)?.ring_size() as usize;
                if rs > 0 {
                    ring_invariant = ring_invariant * rs;
                }
            }
            inv.push([
                ring_invariant,
                self.init_rank(atom)?,
                self.distance_count(atom)?,
            ]);
        }
        let ranks = rank_matrix(&mut inv);

        for ix in 0..atoms.len() {
            self.atom_mut(&atoms[ix])?.set_rank(ranks[ix]);
        }
        self.canon()?;
        for atom in atoms.iter() {
            self.atom_mut(atom)?.set_symmetry_class();
        }
        Ok(())
    }

    fn canon(&mut self) -> Result<usize> {
        if self.atoms.len() < 2 {
            return Ok(1);
        }
        let mut ranks = Vec::new();
        for atom in self.atoms.iter() {
            ranks.push(self.atom_at(atom)?.rank());
        }
        let mut dist = 0;
        rank(&mut ranks, &mut dist);
        let mut predist = dist - 1;
        while dist < self.atoms.len() && dist > predist {
            let preranks = ranks.clone();
            predist = dist;
            for ix in 0..self.atoms.len() {
                ranks[ix] = ranks[ix].pow(8);
                for n in self.graph.neighbors(&self.atoms[ix])? {
                    let b = self.edge_at(self.atoms[ix], *n)?;
                    match b.electron() {
                        1 => ranks[ix] = ranks[ix] * prime(preranks[*n as usize - 1]),
                        2 => ranks[ix] = ranks[ix] * prime(preranks[*n as usize - 1]).pow(2),
                        3 => ranks[ix] = ranks[ix] * prime(preranks[*n as usize - 1]).pow(3),
                        4 => ranks[ix] = ranks[ix] * prime(preranks[*n as usize - 1]).pow(4),
                        _ => unreachable!(),
                    };
                }
            }
            rank(&mut ranks, &mut dist);
        }
        for ix in 0..ranks.len() {
            self.atom_mut(&(ix as u8 + 1))?.set_rank(ranks[ix]);
        }
        Ok(dist)
    }
}

#[test]
fn test_degree() {
    let mut m = Molecule::new();
    let c1 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    assert_eq!(c1, 1);
    let c2 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    assert_eq!(c2, 2);
    let o = m
        .add_atom(Atom::new_aliphatic(super::element::O, true))
        .unwrap();
    assert_eq!(o, 3);
    assert_eq!(m.order(), 3);
    assert!(m.add_bond(c1, c2, super::bond::SINGLE).unwrap());
    assert!(m.add_bond(c2, o, super::bond::SINGLE).unwrap());
    assert_eq!(m.hydrogen_count(&c1).unwrap(), 3);
    assert_eq!(m.hydrogen_count(&c2).unwrap(), 2);
    assert_eq!(m.hydrogen_count(&o).unwrap(), 1);
    assert_eq!(m.degree(&c1).unwrap(), 1);
    assert_eq!(m.degree(&c2).unwrap(), 2);
    assert_eq!(m.degree(&o).unwrap(), 1);
}

#[test]
fn test_find_double_bond() {
    let mut m = Molecule::new();
    let c1 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c2 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c3 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c4 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    assert!(m.add_bond(c1, c2, super::bond::SINGLE).unwrap());
    assert!(m.add_bond(c2, c3, super::bond::DOUBLE).unwrap());
    assert!(m.add_bond(c3, c4, super::bond::SINGLE).unwrap());
    assert_eq!(m.find_double_bond(c2, c1).unwrap(), 3);
    assert_eq!(m.find_double_bond(c2, c3).unwrap(), -1);
}

#[test]
fn test_bond_venlence() {
    let mut m = Molecule::new();
    let c1 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c2 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c3 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c4 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    assert!(m.add_bond(c1, c2, super::bond::SINGLE).unwrap());
    assert!(m.add_bond(c2, c3, super::bond::DOUBLE).unwrap());
    assert!(m.add_bond(c3, c4, super::bond::SINGLE).unwrap());
    assert_eq!(m.bond_valences(&c2).unwrap(), 3);
    assert_eq!(m.bond_valences(&c1).unwrap(), 1);
    assert_eq!(m.bond_valences(&c3).unwrap(), 3);
    assert_eq!(m.bond_valences(&c4).unwrap(), 1);
}

#[test]
fn test_connectivity() {
    let mut m = Molecule::new();
    let c1 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c2 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c3 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c4 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    assert!(m.add_bond(c1, c2, super::bond::SINGLE).unwrap());
    assert!(m.add_bond(c2, c3, super::bond::DOUBLE).unwrap());
    assert!(m.add_bond(c3, c4, super::bond::SINGLE).unwrap());
    assert_eq!(4, m.connectivity(&1).unwrap());
    assert_eq!(4, m.connectivity(&2).unwrap());
    assert_eq!(4, m.connectivity(&3).unwrap());
    assert_eq!(4, m.connectivity(&4).unwrap());
}

#[test]
fn test_distance_count() {
    let mut m = Molecule::new();
    let c1 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c2 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c3 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    let c4 = m
        .add_atom(Atom::new_aliphatic(super::element::C, true))
        .unwrap();
    assert!(m.add_bond(c1, c2, super::bond::SINGLE).unwrap());
    assert!(m.add_bond(c2, c3, super::bond::DOUBLE).unwrap());
    assert!(m.add_bond(c3, c4, super::bond::SINGLE).unwrap());
    m.rings_detection().unwrap();
    assert_eq!(12, m.distance_count(&2).unwrap());
    assert_eq!(12, m.distance_count(&3).unwrap());
    assert_eq!(1, m.distance_count(&1).unwrap());
}
