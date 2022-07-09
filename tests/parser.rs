mod test {
    use ruatom::molecule::{TopologySeq, AL1, AL2, DB1, DB2};
    use ruatom::{error::RuatomError, parser::Parser};

    #[test]
    fn test_parser_unclosed_ring() {
        let p = Parser::new("C1CCCCC");
        let err = p.parse();
        assert_eq!(
            err.err().unwrap(),
            RuatomError::IllegalSMILES("unclosed ring")
        );
    }

    #[test]
    fn test_parser_unclosed_ring_with_closed() {
        let p = Parser::new("C1CCCCC1CCCC1CCCC");
        let err = p.parse();
        assert_eq!(
            err.err().unwrap(),
            RuatomError::IllegalSMILES("unclosed ring")
        );
    }

    #[test]
    fn test_parser_unclosed_branch_left() {
        let p = Parser::new("CCCC(CCCC");
        let err = p.parse();
        assert_eq!(
            err.err().unwrap(),
            RuatomError::IllegalSMILES("unclosed branch")
        );
    }

    #[test]
    fn test_parser_unclosed_branch_right() {
        let p = Parser::new("CCCC)CCCC");
        let err = p.parse();
        assert_eq!(
            err.err().unwrap(),
            RuatomError::IllegalSMILES("failed to close branch after ')'")
        );
    }

    #[test]
    fn test_parser_unclosed_branch_with_closed() {
        let p = Parser::new("CCC(C)C(CCC");
        let err = p.parse();
        assert_eq!(
            err.err().unwrap(),
            RuatomError::IllegalSMILES("unclosed branch")
        );
    }

    #[test]
    fn test_invalid_tetrahedral_subtract() {
        let p = Parser::new("[C@-](N)(O)(C)");
        let m = p.parse().unwrap();
        assert_eq!(
            TopologySeq::UnknownTopology,
            m.topology_at(&1).unwrap().seq()
        );
    }

    #[test]
    fn test_invalid_tetrahedral() {
        let p = Parser::new("[C@](N)(O)(C)");
        let m = p.parse().unwrap();
        assert_eq!(
            TopologySeq::UnknownTopology,
            m.topology_at(&1).unwrap().seq()
        );
    }

    #[test]
    fn test_tellurophene() {
        let p = Parser::new("c1cc[te]c1");
        let m = p.parse().unwrap();
        assert_eq!(m.order(), 5);
        assert_eq!(m.size(), 5);
    }

    #[test]
    fn test_aromatic_kekule() {
        let p = Parser::new("C:1:C:C:C:C:C1");
        let m = p.parse().unwrap();
        m.map_bonds(|_e, b| {
            assert!(b.is_aromatic());
        })
        .unwrap();
    }

    #[test]
    fn test_hydrogen() {
        let p = Parser::new("HH");
        let m = p.parse().unwrap();
        assert_eq!(m.order(), 2);
        assert_eq!(m.total_hs(true).unwrap(), 2);
    }

    #[test]
    fn test_tellurium() {
        let p = Parser::new("[te]");
        let m = p.parse().unwrap();
        assert_eq!(m.order(), 1);
        let a = m.atom_at(&1).unwrap();
        assert!(a.is_aromatic());
        assert!(a.is("Te"));
    }

    #[test]
    fn test_anti_clockwise_extended_tetrahedral() {
        let p = Parser::new("C(C)C=[C@]=CC");
        let m = p.parse().unwrap();
        assert_eq!(m.topology_at(&4).unwrap().configuration().unwrap(), AL1);
    }

    #[test]
    fn test_clockwise_extended_tetrahedral() {
        let p = Parser::new("C(C)C=[C@@]=CC");
        let m = p.parse().unwrap();
        assert_eq!(m.topology_at(&4).unwrap().configuration().unwrap(), AL2);
    }

    #[test]
    fn test_up_down() {
        let p = Parser::new("C/C=C/C\\C=C/C");
        p.parse().unwrap();
    }

    #[test]
    fn test_up_down_with_multiple() {
        let p = Parser::new("C/C=C(/C)/C");
        assert_eq!(
            RuatomError::IllegalMolecule("invalid Cis/Trans specification"),
            p.parse().err().unwrap()
        )
    }

    #[test]
    fn test_up_down_with_invalid() {
        let p = Parser::new("C\\=C");
        assert_eq!(
            RuatomError::IllegalSMILES("bond conflict"),
            p.parse().err().unwrap()
        )
    }

    #[test]
    fn test_selenium_th() {
        let p = Parser::new("[Se@](=O)(C)CC");
        p.parse().unwrap();
    }

    #[test]
    fn test_selenium_ion_th() {
        let p = Parser::new("[Se@+](=O)(C)CC");
        p.parse().unwrap();
    }

    #[test]
    fn test_difluoroethene() {
        let p = Parser::new("F[C@H]=[C@H]F");
        let m = p.parse().unwrap();
        let ty = m.topology_at(&2).unwrap();
        assert_eq!(ty.configuration().unwrap(), DB1);
        assert_eq!(m.topology_at(&2).unwrap().configuration().unwrap(), DB1);
    }

    #[test]
    fn test_difluoroethene_db2() {
        let p = Parser::new("F[C@@H]=[C@@H]F");
        let m = p.parse().unwrap();
        let ty = m.topology_at(&2).unwrap();
        assert_eq!(ty.configuration().unwrap(), DB2);
        assert_eq!(m.topology_at(&3).unwrap().configuration().unwrap(), DB2);
    }

    #[test]
    fn test_difluoroethene_db1_and_db2() {
        let p = Parser::new("F[C@H]=[C@@H]F");
        let m = p.parse().unwrap();
        let ty = m.topology_at(&2).unwrap();
        assert_eq!(ty.configuration().unwrap(), DB1);
        assert_eq!(m.topology_at(&3).unwrap().configuration().unwrap(), DB2);
    }

    #[test]
    fn test_difluoroethene_explict_db1_and_db2() {
        let p = Parser::new("F[C@DB1H]=[C@DB2H]F");
        let m = p.parse().unwrap();
        let ty = m.topology_at(&2).unwrap();
        assert_eq!(ty.configuration().unwrap(), DB1);
        assert_eq!(m.topology_at(&3).unwrap().configuration().unwrap(), DB2);
    }

    #[test]
    fn test_bracket_uranium() {
        let p = Parser::new("[U]");
        let m = p.parse().unwrap();
        assert!(m.atom_at(&1).unwrap().is("U"))
    }

    #[test]
    fn test_bracket_uranium_238() {
        let p = Parser::new("[238U]");
        let m = p.parse().unwrap();
        let atom = m.atom_at(&1).unwrap();
        assert!(atom.is("U"));
        assert_eq!(m.hydrogen_count(&1).unwrap(), 0);
        assert_eq!(0, atom.explicit_hydrogens());
        assert_eq!(238, atom.isotope());
        assert_eq!(atom.charge(), 0);
    }

    #[test]
    fn test_bracket_lead() {
        let p = Parser::new("[Pb]");
        let m = p.parse().unwrap();
        assert!(m.atom_at(&1).unwrap().is("Pb"))
    }

    #[test]
    fn test_bracket_unknown() {
        let p = Parser::new("[*]");
        let m = p.parse().unwrap();
        assert!(m.atom_at(&1).unwrap().ele_is_any())
    }

    #[test]
    fn test_bracket_hydrogen_anion() {
        let p = Parser::new("[OH1-]");
        let m = p.parse().unwrap();
        let atom = m.atom_at(&1).unwrap();
        assert!(atom.is("O"));
        assert_eq!(m.hydrogen_count(&1).unwrap(), 1);
        assert_eq!(1, atom.explicit_hydrogens());
        assert_eq!(atom.charge(), -1);
    }

    #[test]
    fn test_bracket_hydrogen_anion_alt() {
        let p = Parser::new("[OH-1]");
        let m = p.parse().unwrap();
        let atom = m.atom_at(&1).unwrap();
        assert!(atom.is("O"));
        assert_eq!(m.hydrogen_count(&1).unwrap(), 1);
        assert_eq!(1, atom.explicit_hydrogens());
        assert_eq!(atom.charge(), -1);
    }

    #[test]
    fn test_bracket_copper_cation() {
        let p = Parser::new("[Cu+2]");
        let m = p.parse().unwrap();
        let atom = m.atom_at(&1).unwrap();
        assert!(atom.is("Cu"));
        assert_eq!(m.hydrogen_count(&1).unwrap(), 0);
        assert_eq!(0, atom.explicit_hydrogens());
        assert_eq!(atom.charge(), 2);
    }

    #[test]
    fn test_bracket_copper_cation_alt() {
        let p = Parser::new("[Cu++]");
        let m = p.parse().unwrap();
        let atom = m.atom_at(&1).unwrap();
        assert!(atom.is("Cu"));
        assert_eq!(m.hydrogen_count(&1).unwrap(), 0);
        assert_eq!(0, atom.explicit_hydrogens());
        assert_eq!(atom.charge(), 2);
    }

    #[test]
    fn test_bracket_methane_isotope() {
        let p = Parser::new("[13CH4]");
        let m = p.parse().unwrap();
        let atom = m.atom_at(&1).unwrap();
        assert!(atom.is("C"));
        assert_eq!(m.hydrogen_count(&1).unwrap(), 4);
        assert_eq!(4, atom.explicit_hydrogens());
        assert_eq!(13, atom.isotope());
        assert_eq!(atom.charge(), 0);
    }

    #[test]
    fn test_bracket_deuterium_ion() {
        let p = Parser::new("[2H+]");
        let m = p.parse().unwrap();
        let atom = m.atom_at(&1).unwrap();
        assert!(atom.is("H"));
        assert_eq!(m.hydrogen_count(&1).unwrap(), 0);
        assert_eq!(0, atom.explicit_hydrogens());
        assert_eq!(2, atom.isotope());
        assert_eq!(atom.charge(), 1);
    }

    #[test]
    fn test_bracket_chlorine36() {
        let p = Parser::new("[36Cl]");
        let m = p.parse().unwrap();
        let atom = m.atom_at(&1).unwrap();
        assert!(atom.is("Cl"));
        assert_eq!(m.hydrogen_count(&1).unwrap(), 0);
        assert_eq!(0, atom.explicit_hydrogens());
        assert_eq!(36, atom.isotope());
        assert_eq!(atom.charge(), 0);
    }

    #[test]
    fn test_total_hs() {
        let p = Parser::new("[36Cl]");
        let m = p.parse().unwrap();
        let atom = m.total_hs(false).unwrap();
        assert_eq!(atom, 0);
        assert_eq!(m.total_hs(true).unwrap(), 0);
        let p = Parser::new("H");
        let m = p.parse().unwrap();
        let atom = m.total_hs(false).unwrap();
        assert_eq!(atom, 1);
        assert_eq!(m.total_hs(true).unwrap(), 1);

        let p = Parser::new("c1ccccc1");
        let m = p.parse().unwrap();
        assert_eq!(m.total_hs(false).unwrap(), 6);
        assert_eq!(m.total_hs(true).unwrap(), 6);

        let p = Parser::new("[12CH3]C");
        let m = p.parse().unwrap();
        assert_eq!(m.total_hs(false).unwrap(), 6);
        assert_eq!(m.total_hs(true).unwrap(), 6);

        let p = Parser::new("C([1H])C");
        let m = p.parse().unwrap();
        assert_eq!(m.total_hs(false).unwrap(), 5);
        assert_eq!(m.total_hs(true).unwrap(), 6);

        let p = Parser::new("C([1H])([2H])C");
        let m = p.parse().unwrap();
        assert_eq!(m.total_hs(false).unwrap(), 4);
        assert_eq!(m.total_hs(true).unwrap(), 6);
    }

    #[test]
    fn test_ring_hs() {
        let p = Parser::new("C1(C2C3C4C15)C6C7C2C8C3C9C%10C4C%11C5C6C%12C%11C%10C%13C%12C7C8C9%13");
        let m = p.parse().unwrap();
        assert_eq!(m.heavy_atom_amount("C").unwrap(), 24);
        assert_eq!(m.total_hs(false).unwrap(), 24)
    }

    #[test]
    fn test_mw() {
        let p = Parser::new("c1ccccc1");
        let m = p.parse().unwrap();
        assert_eq!(78.046950192, m.molecule_weight().unwrap());
        assert_eq!(m.exact_molecule_weight().unwrap(), 78.11184);

        let p = Parser::new("CC");
        let m = p.parse().unwrap();
        assert_eq!(30.046950192, m.molecule_weight().unwrap());
        assert_eq!(m.exact_molecule_weight().unwrap(), 30.06904);

        let p = Parser::new("[13CH3]C");
        let m = p.parse().unwrap();
        assert_eq!(30.046950192, m.molecule_weight().unwrap());
        assert_eq!(m.exact_molecule_weight().unwrap(), 31.06169484);

        let p = Parser::new("[13CH3]C([3H])");
        let m = p.parse().unwrap();
        assert_eq!(30.046950192, m.molecule_weight().unwrap());
        assert_eq!(m.exact_molecule_weight().unwrap(), 33.069804118);
    }

    #[test]
    fn test_heavy_atom_amount() {
        let p = Parser::new("c1ccccc1");
        let m = p.parse().unwrap();
        assert_eq!(m.heavy_atom_amount("C").unwrap(), 6);

        let p = Parser::new("[13CH3]C([3H])");
        let m = p.parse().unwrap();
        assert_eq!(m.heavy_atom_amount("C").unwrap(), 2);
        assert_eq!(m.total_hs(true).unwrap(), 6);

        let p = Parser::new("[C@-](N)(O)(C)");
        let m = p.parse().unwrap();
        assert_eq!(m.heavy_atom_amount("N").unwrap(), 1);
        assert_eq!(m.heavy_atom_amount("O").unwrap(), 1);
        assert_eq!(m.heavy_atom_amount("C").unwrap(), 2);
        assert_eq!(m.total_hs(true).unwrap(), 6);
        let p = Parser::new("CC(N)=O");
        let m = p.parse().unwrap();
        assert_eq!(m.heavy_atom_amount("N").unwrap(), 1);
        assert_eq!(m.heavy_atom_amount("O").unwrap(), 1);
        assert_eq!(m.heavy_atom_amount("C").unwrap(), 2);
        assert_eq!(m.total_hs(true).unwrap(), 5);
    }

    #[test]
    fn test_sssr() {
        let p = Parser::new("OC1C2C1CC2");
        let m = p.parse().unwrap();
        assert_eq!(2, m.n_ssr());

        let p = Parser::new("c1cc[te]c1");
        let m = p.parse().unwrap();
        assert_eq!(1, m.n_ssr());
    }

    #[test]
    fn test_ring_size() {
        let p = Parser::new("C2CCC1CCCC1C2");
        let mut m = p.parse().unwrap();
        assert!(!m.rings_detection().is_err());
    }

    #[test]
    fn test_bond_degree() {
        let p = Parser::new("c1ccccc1");
        let m = p.parse().unwrap();
        assert_eq!(2, m.bond_degree_of(&2).unwrap());
    }

    #[test]
    fn test_distance_count() {
        let p = Parser::new("c1ccccc1CN");
        let m = p.parse().unwrap();
        let distances = vec![122,122,122,122,122,122,1,1];
        for atom in 1..9{
            assert_eq!(distances[(atom-1) as usize], m.distance_count(&atom).unwrap());
        }
    }

    // #[test]
    // fn test_symmetry_detection() {
    //     let p = Parser::new("C1OC23COC45COC11COC67COC8(COC9(CO2)COC(CO1)(CO6)OCC(CO9)(OC4)OCC(CO5)(OC7)OC8)OC3");
    //     let m = p.parse().unwrap();
    //     let mark = vec![1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,2,1,2,3,1,2,1,2,2,1,3,1,2,2,1,2,1,3,1,2,2,1,2,1,2,1];
    //     for at in 1..46{
    //         assert_eq!(mark[(at-1) as usize], m.atom_at(&at).unwrap().rank());
    //     }
        
    // }

    #[test]
    fn test_aromaticity() {
        let p = Parser::new("c1ccccc1CN");
        let m = p.parse().unwrap();
        for i in 1..7{
            assert!(m.atom_at(&i).unwrap().is_aromatic());
        }
        for i in 7..9{
            assert!(!m.atom_at(&i).unwrap().is_aromatic());
        }
    }
}
