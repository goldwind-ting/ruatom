extern crate molecule;
extern crate parser;

mod test {
    use molecule::{error::MoleculeError, TopologySeq, AL1, AL2};
    use parser::{error::RuatomError, Parser};

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
            m.topology_at(&0).unwrap().seq()
        );
    }

    #[test]
    fn test_invalid_tetrahedral() {
        let p = Parser::new("[C@](N)(O)(C)");
        let m = p.parse().unwrap();
        assert_eq!(
            TopologySeq::UnknownTopology,
            m.topology_at(&0).unwrap().seq()
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
    }

    #[test]
    fn test_deuterium() {
        let p = Parser::new("DD");
        let m = p.parse().unwrap();
        assert_eq!(m.order(), 2);
    }

    #[test]
    fn test_tritium() {
        let p = Parser::new("TTDDHH");
        let m = p.parse().unwrap();
        assert_eq!(m.order(), 6);
    }

    #[test]
    fn test_tellurium() {
        let p = Parser::new("[te]");
        let m = p.parse().unwrap();
        assert_eq!(m.order(), 1);
        let a = m.atom_at(&0).unwrap();
        assert!(a.is_aromatic());
        assert!(a.is("Te"));
    }

    #[test]
    fn test_anti_clockwise_extended_tetrahedral() {
        let p = Parser::new("C(C)C=[C@]=CC");
        let m = p.parse().unwrap();
        assert_eq!(m.topology_at(&3).unwrap().configuration().unwrap(), AL1);
    }

    #[test]
    fn test_clockwise_extended_tetrahedral() {
        let p = Parser::new("C(C)C=[C@@]=CC");
        let m = p.parse().unwrap();
        assert_eq!(m.topology_at(&3).unwrap().configuration().unwrap(), AL2);
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
            RuatomError::MoleculeError(MoleculeError::IllegalMolecule(
                "invalid Cis/Trans specification"
            )),
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
}
