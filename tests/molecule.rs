#[cfg(test)]
mod test {
    use hashbrown::HashMap;
    use ruatom::molecule::{atom::Atom, bond::*, element::*, Molecule};

    #[test]
    fn test_add_atom() {
        let mut m = Molecule::new();
        let c1 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert_eq!(c1, 1);
        let c2 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert_eq!(c2, 2);
        let o = m.add_atom(Atom::new_aliphatic(O)).unwrap();
        assert_eq!(o, 3);
        assert_eq!(m.order(), 3);
    }

    #[test]
    fn test_add_bond() {
        let mut m = Molecule::new();
        let c1 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert_eq!(c1, 1);
        let c2 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert_eq!(c2, 2);
        let o = m.add_atom(Atom::new_aliphatic(O)).unwrap();
        assert_eq!(o, 3);
        assert_eq!(m.order(), 3);
        assert!(m.add_bond(c1, c2, SINGLE).unwrap());
        assert!(m.add_bond(c2, o, SINGLE).unwrap());
        assert_eq!(m.hydrogen_count(&c1).unwrap(), 3);
        assert_eq!(m.hydrogen_count(&c2).unwrap(), 2);
        assert_eq!(m.hydrogen_count(&o).unwrap(), 1);
    }

    #[test]
    fn test_hydrogen_count() {
        let mut m = Molecule::new();
        let c1 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert_eq!(c1, 1);
        let c2 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert_eq!(c2, 2);
        let o = m.add_atom(Atom::new_aliphatic(O)).unwrap();
        assert_eq!(o, 3);
        assert_eq!(m.order(), 3);
        assert!(m.add_bond(c1, c2, IMPLICT).unwrap());
        assert!(m.add_bond(c2, o, IMPLICT).unwrap());
        assert_eq!(m.hydrogen_count(&c1).unwrap(), 3);
        assert_eq!(m.hydrogen_count(&c2).unwrap(), 2);
        assert_eq!(m.hydrogen_count(&o).unwrap(), 1);
    }

    #[test]
    fn test_find_extend_tetrahedral_ends() {
        let mut m = Molecule::new();
        let c1 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c2 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c3 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c4 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c5 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert!(m.add_bond(c1, c2, SINGLE).unwrap());
        assert!(m.add_bond(c2, c3, DOUBLE).unwrap());
        assert!(m.add_bond(c3, c4, DOUBLE).unwrap());
        assert!(m.add_bond(c4, c5, SINGLE).unwrap());
        assert_eq!(m.find_extend_tetrahedral_ends(c3).unwrap(), vec![2, 4]);
        assert_eq!(m.hydrogen_count(&c1).unwrap(), 3);
        assert_eq!(m.hydrogen_count(&c2).unwrap(), 1);
        assert_eq!(m.hydrogen_count(&c3).unwrap(), 0);
        assert_eq!(m.hydrogen_count(&c4).unwrap(), 1);
        assert_eq!(m.hydrogen_count(&c5).unwrap(), 3);
    }

    #[test]
    fn test_ring() {
        // C1CCCCC1
        let mut m = Molecule::new();
        let c1 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert!(m.enable_open(1));
        m.open_ring(1, IMPLICT, None, 1);
        let c2 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert_eq!(m.ring_num(), 1);
        assert!(!m.enable_open(1));
        assert!(m.add_bond(c1, c2, IMPLICT).unwrap());
        let c3 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert!(m.add_bond(c2, c3, IMPLICT).unwrap());
        let c4 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert!(m.add_bond(c3, c4, IMPLICT).unwrap());
        let c5 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert!(m.add_bond(c4, c5, IMPLICT).unwrap());
        let c6 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert!(m.add_bond(c5, c6, IMPLICT).unwrap());
        m.close_ring(1, 6, IMPLICT).unwrap();
        assert!(m.enable_open(1));
        assert_eq!(m.hydrogen_count(&c1).unwrap(), 2);
        assert_eq!(m.hydrogen_count(&c2).unwrap(), 2);
        assert_eq!(m.hydrogen_count(&c3).unwrap(), 2);
        assert_eq!(m.hydrogen_count(&c4).unwrap(), 2);
        assert_eq!(m.hydrogen_count(&c5).unwrap(), 2);
        assert_eq!(m.hydrogen_count(&c6).unwrap(), 2);
        assert_eq!(m.ring_num(), 0);
    }

    #[test]
    fn test_validate_up_down() {
        let mut m = Molecule::new();
        let c1 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c2 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c3 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c4 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c5 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c6 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c7 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let mut db = HashMap::new();
        assert!(m.add_bond(c1, c2, UP).unwrap());
        db.insert(c1, true);
        db.insert(c2, true);
        assert!(m.add_bond(c2, c3, DOUBLE).unwrap());
        assert!(m.add_bond(c3, c4, UP).unwrap());
        db.insert(c3, true);
        db.insert(c4, true);
        assert!(m.add_bond(c4, c5, DOWN).unwrap());
        db.insert(c4, true);
        db.insert(c5, true);
        assert!(m.add_bond(c5, c6, DOUBLE).unwrap());
        assert!(m.add_bond(c6, c7, DOWN).unwrap());
        db.insert(c6, true);
        db.insert(c7, true);
        assert!(m.validate_up_down(db).is_ok());

        let c1 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c2 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c3 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c4 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c5 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let mut db = HashMap::new();
        assert!(m.add_bond(c1, c2, UP).unwrap());
        db.insert(c1, true);
        db.insert(c2, true);
        assert!(m.add_bond(c2, c3, DOUBLE).unwrap());
        assert!(m.add_bond(c3, c4, UP).unwrap());
        db.insert(c3, true);
        db.insert(c4, true);
        assert!(m.add_bond(c3, c5, UP).unwrap());
        db.insert(c3, true);
        db.insert(c5, true);
        assert!(m.validate_up_down(db).is_err());
    }

    #[test]
    fn test_bond_degree() {
        let mut m = Molecule::new();
        let c1 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c2 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c3 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c4 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c5 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c6 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        let c7 = m.add_atom(Atom::new_aliphatic(C)).unwrap();
        assert!(m.add_bond(c1, c2, UP).unwrap());
        assert!(m.add_bond(c2, c3, DOUBLE).unwrap());
        assert!(m.add_bond(c3, c4, UP).unwrap());
        assert!(m.add_bond(c4, c5, DOWN).unwrap());
        assert!(m.add_bond(c5, c6, DOUBLE).unwrap());
        assert!(m.add_bond(c6, c7, DOWN).unwrap());
        assert_eq!(1, m.bond_degree_of(&1).unwrap());
        assert_eq!(3, m.bond_degree_of(&2).unwrap());
        assert_eq!(3, m.bond_degree_of(&3).unwrap());
        assert_eq!(2, m.bond_degree_of(&4).unwrap());
        assert_eq!(3, m.bond_degree_of(&5).unwrap());
        assert_eq!(3, m.bond_degree_of(&6).unwrap());
        assert_eq!(1, m.bond_degree_of(&7).unwrap());
    }
}
