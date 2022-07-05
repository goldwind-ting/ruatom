use crate::molecule::configuration::{
    Configuration, AL1, AL2, ANTICLOCKWISE, CLOCKWISE, DB1, DB2, OH_MAP, SP1, SP2, SP3, TB_MAP,
    TH1, TH2, UNKNOWN,
};
use crate::molecule::{
    atom::*, bond::*, create, element::*, Molecule, HAS_AROM, HAS_ATM_STRO, HAS_BND_STRO,
    HAS_EXT_STRO, HAS_STRO,
};
use crate::{
    char_buff::CharBuffer,
    error::{Result, RuatomError},
};
use hashbrown::{HashMap, HashSet};

pub struct Parser {
    buf: CharBuffer,
    molecule: Molecule,
    current_bond: Bond,
    stack: Vec<u8>,
    adjacent_map: HashMap<u8, Vec<i8>>,
    last_bond_pos: Option<u8>,
    start: HashSet<u8>,
    configuration: Configuration,
    configurations: HashMap<u8, Configuration>,
    hastrix: bool,
    directional_bonds: HashMap<u8, bool>,
}

impl Parser {
    pub fn new(smi: &str) -> Self {
        Parser {
            buf: CharBuffer::from_str(smi),
            molecule: Molecule::new(),
            current_bond: IMPLICT,
            stack: Vec::new(),
            adjacent_map: HashMap::with_capacity(4),
            last_bond_pos: None,
            start: HashSet::new(),
            configuration: UNKNOWN,
            configurations: HashMap::new(),
            hastrix: false,
            directional_bonds: HashMap::new(),
        }
    }

    fn add_atom(&mut self, atom: Atom) -> Result<()> {
        let v = self.molecule.add_atom(atom)?;
        if !self.stack.is_empty() {
            let u = self.stack.pop().unwrap();
            if self.current_bond != DOT {
                if self.current_bond.direction() {
                    self.directional_bonds.insert(u, true);
                    self.directional_bonds.insert(v, true);
                }
                self.molecule.add_bond(u as u8, v, self.current_bond)?;
                self.set_adjacent(u, v as i8);
                self.set_adjacent(v, u as i8);
            }
        } else {
            self.start.insert(v);
        }
        self.stack.push(v);
        self.current_bond = IMPLICT;
        if self.configuration != UNKNOWN {
            self.molecule.set_flags(HAS_ATM_STRO);
            self.configurations.insert(v, self.configuration.clone());
            self.configuration = UNKNOWN;
        };
        Ok(())
    }

    pub fn parse(mut self) -> Result<Molecule> {
        self.read_smiles()?;
        self.molecule.rings_detection()?;
        if self.molecule.ring_num() > 0 {
            return Err(RuatomError::IllegalSMILES("unclosed ring"));
        }
        if self.stack.len() > 1 {
            return Err(RuatomError::IllegalSMILES("unclosed branch"));
        }
        if self.molecule.get_flag(HAS_STRO) != 0 {
            self.build_topologies()?;
        }
        if self.hastrix {
            self.molecule.trans_astrix_atom()?;
        }
        Ok(self.molecule)
    }

    fn build_topologies(&mut self) -> Result<()> {
        let configurations = self.configurations.clone();
        for (k, c) in configurations.iter() {
            let conf = self.molecule.to_explict_configuration(*k, c)?;
            self.add_topology(*k, conf)?;
        }
        let directional_bonds = self.directional_bonds.clone();
        self.molecule.validate_up_down(directional_bonds)?;
        Ok(())
    }

    fn add_topology(&mut self, u: u8, conf: Configuration) -> Result<()> {
        return match self.adjacent_map.get(&u) {
            None => Err(RuatomError::IllegalSMILES("no such atom in adjacent_map")),
            Some(arr) => {
                let mut us = arr.clone().to_owned();
                if conf.is_tetrahedral() {
                    us = self.modify_th_arrangement_order(u, us)?;
                } else if conf.is_trigonal() {
                    us = self.modify_db_arrangement_order(u, us)?;
                } else if conf.is_extend_tetrahedral() {
                    self.molecule.set_flags(HAS_EXT_STRO);
                    us = self.get_allene_carriers(u)?;
                } else if conf.is_square_plannar() && us.len() != 4 {
                    return Err(RuatomError::IllegalSMILES(
                        "SquarePlanar without 4 explict neighbors",
                    ));
                } else if conf.is_trigonal_bipyramidal() && us.len() != 5 {
                    return Err(RuatomError::IllegalSMILES(
                        "TrigonalBipyramidal without 4 explict neighbors",
                    ));
                } else if conf.is_octahedral() && us.len() != 6 {
                    return Err(RuatomError::IllegalSMILES(
                        "Octahedral without 4 explict neighbors",
                    ));
                }
                self.molecule.add_topology(create(u, conf, us)?);
                return Ok(());
            }
        };
    }

    fn read_smiles(&mut self) -> Result<()> {
        loop {
            let c = self.buf.next_with_progress();
            if let None = c {
                return Ok(());
            }
            match c.unwrap() {
                '*' => {
                    self.hastrix = true;
                    self.add_atom(Atom::new_any(ANY, true))?;
                }
                'B' => {
                    if self.buf.is_tar_with_progress('r') {
                        self.add_atom(Atom::new_aliphatic(BR, true))?;
                    } else {
                        self.add_atom(Atom::new_aliphatic(B, true))?;
                    }
                }
                'C' => {
                    if self.buf.is_tar_with_progress('l') {
                        self.add_atom(Atom::new_aliphatic(CL, true))?;
                    } else {
                        self.add_atom(Atom::new_aliphatic(C, true))?;
                    }
                }
                'N' => {
                    self.add_atom(Atom::new_aliphatic(N, true))?;
                }
                'O' => {
                    self.add_atom(Atom::new_aliphatic(O, true))?;
                }
                'P' => {
                    self.add_atom(Atom::new_aliphatic(P, true))?;
                }
                'S' => {
                    self.add_atom(Atom::new_aliphatic(S, true))?;
                }
                'F' => {
                    self.add_atom(Atom::new_aliphatic(F, true))?;
                }
                'I' => {
                    self.add_atom(Atom::new_aliphatic(I, true))?;
                }
                'b' => {
                    self.add_atom(Atom::new_aromatic(B, true))?;
                    self.molecule.set_flags(HAS_AROM);
                }
                'c' => {
                    self.add_atom(Atom::new_aromatic(C, true))?;
                    self.molecule.set_flags(HAS_AROM);
                }
                'n' => {
                    self.add_atom(Atom::new_aromatic(N, true))?;
                    self.molecule.set_flags(HAS_AROM);
                }
                'o' => {
                    self.add_atom(Atom::new_aromatic(O, true))?;
                    self.molecule.set_flags(HAS_AROM);
                }
                'p' => {
                    self.add_atom(Atom::new_aromatic(P, true))?;
                    self.molecule.set_flags(HAS_AROM);
                }
                's' => {
                    self.add_atom(Atom::new_aromatic(S, true))?;
                    self.molecule.set_flags(HAS_AROM);
                }
                'H' => {
                    self.add_atom(Atom::new_aliphatic(H, true))?;
                }
                '[' => {
                    let btom = self.read_bracket_atoms()?;
                    self.add_atom(btom)?;
                }
                ch if ch.is_digit(10) => {
                    self.build_ring((ch.to_digit(10).unwrap() - 0) as u8)?;
                }
                '%' => {
                    match self.buf.to_sub_number(&mut 2) {
                        None => return Err(RuatomError::IllegalSMILES("need a digit after '%'")),
                        Some(n) => {
                            if n < 10 {
                                return Err(RuatomError::IllegalSMILES(
                                    "digit must less than 10 after '%'",
                                ));
                            };
                            self.build_ring(n as u8)?;
                            self.last_bond_pos = Some(self.buf.position() as u8);
                        }
                    };
                }
                '-' => {
                    if self.current_bond != IMPLICT {
                        return Err(RuatomError::IllegalSMILES("bond conflict"));
                    }
                    self.current_bond = SINGLE;
                    self.last_bond_pos = Some(self.buf.position() as u8);
                }
                '=' => {
                    if self.current_bond != IMPLICT {
                        return Err(RuatomError::IllegalSMILES("bond conflict"));
                    }
                    self.current_bond = DOUBLE;
                    self.last_bond_pos = Some(self.buf.position() as u8);
                }
                '#' => {
                    if self.current_bond != IMPLICT {
                        return Err(RuatomError::IllegalSMILES("bond conflict"));
                    }
                    self.current_bond = TRIPLE;
                    self.last_bond_pos = Some(self.buf.position() as u8);
                }
                '$' => {
                    if self.current_bond != IMPLICT {
                        return Err(RuatomError::IllegalSMILES("bond conflict"));
                    }
                    self.current_bond = QUADRUPLE;
                    self.last_bond_pos = Some(self.buf.position() as u8);
                }
                ':' => {
                    if self.current_bond != IMPLICT {
                        return Err(RuatomError::IllegalSMILES("bond conflict"));
                    }
                    self.current_bond = AROMATIC;
                    self.molecule.set_flags(HAS_AROM);
                    self.last_bond_pos = Some(self.buf.position() as u8);
                }
                '/' => {
                    if self.current_bond != IMPLICT {
                        return Err(RuatomError::IllegalSMILES("bond conflict"));
                    }
                    self.current_bond = UP;
                    self.molecule.set_flags(HAS_BND_STRO);
                    self.last_bond_pos = Some(self.buf.position() as u8);
                }
                '\\' => {
                    if self.current_bond != IMPLICT {
                        return Err(RuatomError::IllegalSMILES("bond conflict"));
                    }
                    self.current_bond = DOWN;
                    self.molecule.set_flags(HAS_BND_STRO);
                    self.last_bond_pos = Some(self.buf.position() as u8);
                }
                '.' => {
                    if self.current_bond != IMPLICT {
                        return Err(RuatomError::IllegalSMILES("bond conflict"));
                    }
                    self.current_bond = DOT;
                }
                '(' => {
                    if self.stack.is_empty() {
                        return Err(RuatomError::IllegalSMILES(
                            "failed to open new branch after '('",
                        ));
                    }
                    self.stack.push(self.stack[0]);
                }
                ')' => {
                    if self.stack.len() < 2 {
                        return Err(RuatomError::IllegalSMILES(
                            "failed to close branch after ')'",
                        ));
                    }
                    self.stack.pop();
                }
                _ => {
                    return Err(RuatomError::IllegalSMILES("unexpected character"));
                }
            }
        }
    }

    fn set_adjacent(&mut self, key: u8, val: i8) {
        match self.adjacent_map.get_mut(&key) {
            None => {
                self.adjacent_map.insert(key, vec![val]);
            }
            Some(arr) => {
                arr.push(val);
            }
        }
    }

    fn read_element(&mut self) -> Option<Element> {
        self.buf.next_with_progress().map_or(None, |c| {
            let nxt = self.buf.next();
            let mut ele = c.to_string();
            if self.buf.is_remain() && nxt.unwrap().is_lowercase() {
                self.buf.next_with_progress();
                ele.push(nxt.unwrap());
                return Element::read(&ele);
            };
            return Element::read(&ele);
        })
    }

    fn read_hydrogens(&mut self) -> u8 {
        if self.buf.is_tar_with_progress('H') {
            return self.buf.to_number().map_or(1, |count| count as u8);
        };
        return 0;
    }

    fn read_charge(&mut self, acc: i8) -> i8 {
        if self.buf.is_tar_with_progress('+') {
            if self.buf.next_is_digit() {
                return acc + self.buf.to_number().map_or(0, |n| n as i8);
            } else {
                return self.read_charge(acc + 1);
            }
        } else if self.buf.is_tar_with_progress('-') {
            if self.buf.next_is_digit() {
                return acc - self.buf.to_number().map_or(0, |n| n as i8);
            } else {
                return self.read_charge(acc - 1);
            }
        }
        return acc;
    }

    fn read_configuration(&mut self) -> Result<Configuration> {
        if self.buf.is_tar_with_progress('@') {
            if self.buf.is_tar_with_progress('@') {
                return Ok(CLOCKWISE);
            } else if self.buf.is_tar_with_progress('1') {
                return Ok(ANTICLOCKWISE);
            } else if self.buf.is_tar_with_progress('2') {
                return Ok(CLOCKWISE);
            } else if self.buf.is_tar_with_progress('T') {
                if self.buf.is_tar_with_progress('H') {
                    if self.buf.is_tar_with_progress('1') {
                        return Ok(TH1);
                    } else if self.buf.is_tar_with_progress('2') {
                        return Ok(TH2);
                    }
                } else if self.buf.is_tar_with_progress('B') {
                    match self.buf.to_number() {
                        None => return Err(RuatomError::IllegalSMILES("'@TB' is invalid")),
                        Some(num) => {
                            return TB_MAP
                                .get(&num.to_string())
                                .cloned()
                                .ok_or(RuatomError::IllegalSMILES(
                                    "digit after '@TB' must be in range [1, 20]",
                                ))
                                .and_then(|conf| Ok(conf.to_owned()));
                        }
                    };
                }
                return Err(RuatomError::IllegalSMILES("'@T' is invalid"));
            } else if self.buf.is_tar_with_progress('D') {
                if self.buf.is_tar_with_progress('B') {
                    if self.buf.is_tar_with_progress('1') {
                        return Ok(DB1);
                    } else if self.buf.is_tar_with_progress('2') {
                        return Ok(DB2);
                    }
                }
                return Err(RuatomError::IllegalSMILES("'@D' is invalid"));
            } else if self.buf.is_tar_with_progress('A') {
                if self.buf.is_tar_with_progress('L') {
                    if self.buf.is_tar_with_progress('1') {
                        return Ok(AL1);
                    } else if self.buf.is_tar_with_progress('2') {
                        return Ok(AL2);
                    }
                }
                return Err(RuatomError::IllegalSMILES("'@A' is invalid"));
            } else if self.buf.is_tar_with_progress('S') {
                if self.buf.is_tar_with_progress('P') {
                    if self.buf.is_tar_with_progress('1') {
                        return Ok(SP1);
                    } else if self.buf.is_tar_with_progress('2') {
                        return Ok(SP2);
                    } else if self.buf.is_tar_with_progress('3') {
                        return Ok(SP3);
                    }
                }
                return Err(RuatomError::IllegalSMILES("'@S' is invalid"));
            } else if self.buf.is_tar_with_progress('O') {
                if self.buf.is_tar_with_progress('H') {
                    match self.buf.to_number() {
                        None => return Err(RuatomError::IllegalSMILES("'@OH' is invalid")),
                        Some(num) => {
                            return OH_MAP
                                .get(&num.to_string())
                                .cloned()
                                .ok_or(RuatomError::IllegalSMILES(
                                    "digit after '@OH' must be in range [1, 30]",
                                ))
                                .and_then(|conf| Ok(conf.to_owned()));
                        }
                    };
                }
                return Err(RuatomError::IllegalSMILES("'@O' is invalid"));
            } else {
                return Ok(ANTICLOCKWISE);
            }
        }
        return Ok(UNKNOWN);
    }

    fn read_bracket_atoms(&mut self) -> Result<Atom> {
        let isotope = self.buf.to_number();
        let is_aromatic = self.buf.next().map_or(false, |c| c.is_lowercase());
        let ele = self.read_element();
        if ele.is_none() {
            return Err(RuatomError::IllegalSMILES("need an element in bracket"));
        }

        let ele = ele.unwrap();
        if ele.symbol() == "*" {
            self.hastrix = true;
        }
        if is_aromatic && !ele.is_aromatic(Specification::OpenSMILES) {
            return Err(RuatomError::IllegalSMILES("element isn't aromatic"));
        }
        if is_aromatic {
            self.molecule.set_flags(HAS_AROM);
        }
        let isorganogen = ele.is_organogen();
        self.configuration = self.read_configuration()?;
        let hydrogens = self.read_hydrogens();
        let charge = self.read_charge(0);
        if !self.buf.is_tar_with_progress(']') {
            return Err(RuatomError::IllegalSMILES(
                "failed to close bracket, invalid bracket atom",
            ));
        }
        let b_atom = Atom::new_bracket(
            ele,
            isotope.map_or(-1, |n| n as i16),
            hydrogens,
            charge,
            is_aromatic,
            isorganogen,
        );
        return Ok(b_atom);
    }

    fn open_ring(&mut self, rloc: u8) {
        let u = self.stack[0];
        self.molecule
            .open_ring(rloc, self.current_bond.clone(), self.last_bond_pos, u);
        self.set_adjacent(u, -(rloc as i8));
        self.current_bond = IMPLICT;
    }

    fn close_ring(&mut self, rloc: u8) -> Result<()> {
        let u = self.stack[0];
        let v = self
            .molecule
            .close_ring(rloc, u, self.current_bond.clone())?;
        self.adjacent_map
            .get_mut(&v)
            .and_then(|l| replace(l, -(rloc as i8), u as i8));
        self.set_adjacent(v, u as i8);
        self.current_bond = IMPLICT;
        Ok(())
    }

    fn build_ring(&mut self, rloc: u8) -> Result<()> {
        if rloc > 99 || self.current_bond.is(".") || self.stack.is_empty() {
            return Err(RuatomError::IllegalSMILES(""));
        }
        if self.molecule.enable_open(rloc) {
            self.open_ring(rloc);
        } else {
            self.close_ring(rloc)?;
        }
        Ok(())
    }

    fn modify_th_arrangement_order(&self, u: u8, arrangement: Vec<i8>) -> Result<Vec<i8>> {
        if arrangement.len() == 4 {
            return Ok(arrangement);
        }
        if arrangement.len() != 3 {
            return Err(RuatomError::IllegalSMILES(
                "invalid verticies number for TH*",
            ));
        }
        if self.start.contains(&u) {
            return Ok(vec![
                u as i8,
                arrangement[0],
                arrangement[1],
                arrangement[2],
            ]);
        } else {
            return Ok(vec![
                arrangement[0],
                u as i8,
                arrangement[1],
                arrangement[2],
            ]);
        }
    }

    fn modify_db_arrangement_order(&self, u: u8, arrangement: Vec<i8>) -> Result<Vec<i8>> {
        if arrangement.len() == 3 {
            return Ok(arrangement);
        }
        if arrangement.len() != 2 {
            return Err(RuatomError::IllegalSMILES(
                "invalid verticies number for DB*",
            ));
        }
        if self.start.contains(&u) {
            return Ok(vec![u as i8, arrangement[0], arrangement[1]]);
        } else {
            return Ok(vec![arrangement[0], u as i8, arrangement[1]]);
        }
    }

    fn get_allene_carriers(&self, u: u8) -> Result<Vec<i8>> {
        let mut carriers: Vec<i8> = Vec::with_capacity(4);
        let mut index = 0;
        let ends = self.molecule.find_extend_tetrahedral_ends(u)?;
        let beg = ends[0];
        let end = ends[1];
        let begh = self.molecule.hydrogen_count(&beg)? == 1;
        let endh = self.molecule.hydrogen_count(&end)? == 1;
        let mut beg_vertex = self.adjacent_map.get(&beg).unwrap().clone().to_owned();
        if begh {
            if self.start.contains(&beg) {
                beg_vertex.insert(0, -1);
            } else {
                beg_vertex.insert(1, -1);
            }
        }
        for bv in beg_vertex.iter() {
            if bv == &-1 {
                carriers.insert(index, beg as i8);
                index += 1;
                continue;
            }

            if self.molecule.edge_at(beg, *bv as u8)?.is("=") {
                let mut end_vertex = self.adjacent_map.get(&end).unwrap().clone().to_owned();
                if endh {
                    end_vertex.insert(1, -1);
                }
                for ev in end_vertex.iter() {
                    if ev == &-1 {
                        carriers.insert(index, end as i8);
                        index += 1;
                    } else if !self.molecule.edge_at(end, *ev as u8)?.is("=") {
                        carriers.insert(index, *ev);
                        index += 1;
                    }
                }
            } else {
                carriers.insert(index, *bv);
                index += 1;
            }
        }

        if index != 4 {
            return Err(RuatomError::IllegalSMILES("invalid allene topology"));
        }
        Ok(carriers)
    }
}

fn replace<T: PartialEq + Eq>(l: &mut Vec<T>, old: T, new: T) -> Option<T> {
    for i in 0..l.len() {
        if l[i] == old {
            return Some(std::mem::replace(&mut l[i], new));
        }
    }
    return None;
}

mod test {
    #[test]
    fn test_replace() {
        let mut arrangement = vec![1, 2, 3, 4, 5];
        assert_eq!(super::replace(&mut arrangement, 3, 9), Some(3));
        assert_eq!(super::replace(&mut arrangement, 3, 9), None);
    }
}
