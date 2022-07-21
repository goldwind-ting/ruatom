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
        assert!(!a.is_aromatic());
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
    fn test_symmetry_detection() {
        let p = Parser::new(
            "C1OC23COC45COC11COC67COC8(COC9(CO2)COC(CO1)(CO6)OCC(CO9)(OC4)OCC(CO5)(OC7)OC8)OC3",
        );
        let _m = p.parse().unwrap();
        // let mark = vec![1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,2,1,2,3,1,2,1,2,2,1,3,1,2,2,1,2,1,3,1,2,2,1,2,1,2,1];
        // for at in 1..46{
        //     assert_eq!(mark[(at-1) as usize], m.atom_at(&at).unwrap().rank());
        // }
    }

    #[test]
    fn test_aromaticity() {
        let p = Parser::new("c1ccccc1CN");
        let m = p.parse().unwrap();
        for i in 1..7 {
            assert!(m.atom_at(&i).unwrap().is_aromatic());
        }
        for i in 7..9 {
            assert!(!m.atom_at(&i).unwrap().is_aromatic());
        }
    }

    #[test]
    fn test_is_stereocenter() {
        let p = Parser::new("CC(Cl)CO");
        let m = p.parse().unwrap();
        let mark = vec![false, true, false, false, false];
        for i in 1..6 {
            assert_eq!(
                m.atom_at(&i).unwrap().is_stereocenter(),
                mark[i as usize - 1]
            );
        }
        let p = Parser::new("c1ccccc1CN");
        let m = p.parse().unwrap();
        let mark = vec![
            false, false, false, false, false, false, false, false, false,
        ];
        for i in 1..9 {
            assert_eq!(
                m.atom_at(&i).unwrap().is_stereocenter(),
                mark[i as usize - 1]
            );
        }
    }

    #[test]
    fn test_stereocenter() {
        let p = Parser::new("COc1ccc2c(c1)[nH]c(n2)[S@@](=O)Cc1ncc(c(c1C)OC)C");
        let _m = p.parse().unwrap();
    }

    #[test]
    fn test_chirality() {
        let p = Parser::new("COc1ccc2c(c1)[nH]c(n2)[S@@](=O)Cc1ncc(c(c1C)OC)C");
        let m = p.parse().unwrap();
        assert_eq!(m.chirality(&12).unwrap(), 1);

        let p = Parser::new("c1ccccc1C(=O)[C@H](C)Cl");
        let m = p.parse().unwrap();
        assert_eq!(m.chirality(&9).unwrap(), 2);
        assert_eq!(1, m.chiralatoms_count());
    }

    #[test]
    fn test_to_smiles() {
        let p = Parser::new("c1c(CN)cccc1");
        let mut m = p.parse().unwrap();
        let smiles = m.to_smiles().unwrap();
        assert_eq!("NCc1ccccc1", smiles);

        let p = Parser::new("c1ccc(CN)cc1");
        let mut m = p.parse().unwrap();
        let smiles = m.to_smiles().unwrap();
        assert_eq!("NCc1ccccc1", smiles);

        let p = Parser::new("c1cc(CN)ccc1");
        let mut m = p.parse().unwrap();
        let smiles = m.to_smiles().unwrap();
        assert_eq!("NCc1ccccc1", smiles);

        let p = Parser::new("c1cccc(CN)c1");
        let mut m = p.parse().unwrap();
        let smiles = m.to_smiles().unwrap();
        assert_eq!("NCc1ccccc1", smiles);

        let test_data: Vec<String> = vec![
            "CC=[C@AL1]=CCC",
            "Oc1ccccc1",
            "Oc1cccc2ccccc12",
            "CCn1c2ccc3cc2c2cc(ccc12)C(=O)c1ccc(cc1)Cn1cc[n+](c1)Cc1ccc(cc1)-c1cccc(-c2ccc(cc2)C[n+]2ccn(c2)Cc2ccc(cc2)C3=O)c1C(O)=O", // chembl 15,
            "CC(C)(CCCOc1cc(Cl)c(cc1Cl)OCCCC(C)(C)C(O)=O)C(O)=O", // 4631
            "C[N+](C)(CCCCCC[N+](C)(C)CCCN1C(=O)C2C3c4ccccc4C(c4ccccc34)C2C1=O)CCCN1C(=O)c2ccccc2C1=O", // 6053
            "OCCCCCNCc1c2ccccc2c(CNCCCCCO)c2ccccc12", // 23218
            "CC1(C)c2ccc([nH]2)C2(C)CCCCNC(=O)c3cccc(n3)C(=O)NCCCCC(C)(c3ccc1[nH]3)c1ccc([nH]1)C(C)(C)c1ccc2[nH]1", // 4971
            "O=C1NNC(=O)c2ccccc2SSc2ccccc2C(=O)NNC(=O)c2ccccc2SSc2ccccc12", // 140635
            "O=P1([O-])OC2C3OP(=O)([O-])OP(=O)([O-])OC3C3OP(=O)([O-])OP(=O)([O-])OC3C2OP(=O)([O-])O1", // 171007
            "C1CC1N1CN2c3nonc3N3CN(CN4c5nonc5N(C1)C2C34)C1CC1", // 199821
            "O=P1(O)OC2C3OP(=O)(O)OP(=O)(O)OC3C3OP(=O)(O)OP(=O)(O)OC3C2OP(=O)(O)O1", // 208361
            "BrC1CCC(Br)C(Br)CCC(Br)C(Br)CCC1Br", // 377203
            "C1C2CC3CC(CC1C3)C2", // example from nauty, https://pallini.di.uniroma1.it/Introduction.html
            "OCC(CO)(CO)NC(=O)CCCc1ccc(cc1)C1(c2ccccc2)C23c4c5c6c7c8c9c(c%10c%11c2c2c4c4c%12c5c5c6c6c8c8c%13c9c9c%10c%10c%11c%11c2c2c4c4c%12c%12c5c5c6c8c6c8c%13c9c9c%10c%10c%11c2c2c4c4c%12c5c6c5c8c9c%10c2c45)C137", // 267348
            "O=C(O)c1cc2cc(c1)Cc1cc(cc(c1)C(=O)O)Cc1cc(cc(c1)C(=O)O)Cc1cc(cc(c1)C(=O)O)C2", // graph reduction demo
            // r#"C[C@H](CC[C@@H]([C@@H]([C@H](C)C[C@H](C(=C)/C(=C/CO)/C)O)O)OS(=O)(=O)[O-])[C@H]([C@@H](C)[C@H]1[C@@H]([C@@H]([C@H]2[C@H](O1)[C@@H](C[C@]3([C@H](O2)C[C@H]4[C@H](O3)C[C@]5([C@H](O4)[C@H]([C@H]6[C@H](O5)C[C@H]([C@H](O6)[C@@H]([C@H](C[C@H]7[C@@H]([C@@H]([C@H]8[C@H](O7)C[C@H]9[C@H](O8)C[C@H]1[C@H](O9)[C@H]([C@@H]2[C@@H](O1)[C@@H]([C@H]([C@@H](O2)[C@H]1[C@@H]([C@H]([C@H]2[C@@H](O1)C[C@H]([C@@H](O2)[C@@H](C[C@H](C[C@H]1[C@@H]([C@H]([C@H]2[C@@H](O1)C[C@H]([C@@H](O2)[C@H]1[C@@H](C[C@]2([C@H](O1)[C@@H]([C@]1([C@H](O2)C[C@]2([C@H](O1)CC[C@]1([C@H](O2)C[C@]2([C@H](O1)C[C@H]1[C@H](O2)CC[C@H](O1)[C@]1([C@@H](C[C@H]2[C@](O1)(C[C@H]1[C@](O2)(CC[C@]2([C@H](O1)C[C@H]1[C@](O2)(C[C@H]2[C@H](O1)C/C=C\[C@H]1[C@H](O2)C[C@H]2[C@](O1)(C[C@]1([C@H](O2)C[C@H]2[C@](O1)(CC[C@H](O2)[C@H]([C@@H](C[C@@H](C)[C@@H](C)CC=C)O)O)C)C)C)C)C)C)C)O)C)C)C)C)C)O)C)O)O)O)O)O)O)O)O)O)O)O)O)O)OS(=O)(=O)[O-])O)O)O)O)C)C)O)O)O)O"#, // Maitotoxin
            "NC[CH]1O[CH](O[CH]2[CH](O)[CH](O[CH]3[CH](O)[CH](N)C[CH](N)[CH]3O[CH]3O[CH](CN)[CH](O)[CH](O)[CH]3N)O[CH]2CSCCNC(NCCCCN2C(=O)c3ccc4c5ccc6c7c(ccc(c8ccc(c3c48)C2=O)c57)C(=O)N(CCCCNC(NCCSC[CH]2O[CH](O[CH]3[CH](O)[CH](N)C[CH](N)[CH]3O[CH]3O[CH](CN)[CH](O)[CH](O)[CH]3N)[CH](O)[CH]2O[CH]2O[CH](CN)[CH](O)[CH](O)[CH]2N)=S)C6=O)=S)[CH](N)[CH](O)[CH]1O",
            "N[CH](Cc1cnc([nH]1)C12CC3CC(CC(C3)C1)C2)C(=O)N[CH](Cc1c[nH]c2ccccc12)C(=O)N[CH](Cc1cnc([nH]1)C12CC3CC(CC(C3)C1)C2)C(=O)NCc1ccccc1", // 7844
            "O1[C@TH1H](CSCCNC(NCCCCN2C(=O)c3ccc4c5ccc6c7c(ccc(c8ccc(c3c48)C2=O)c57)C(=O)N(CCCCNC(NCCSC[C@TH2H]2O[C@TH2H]([C@TH2H](O)[C@TH2H]2O[C@TH1H]2O[C@TH2H](CN)[C@TH2H](O)[C@TH2H](O)[C@TH1H]2N)O[C@TH2H]2[C@TH2H](O)[C@TH1H](N)C[C@TH1H](N)[C@TH1H]2O[C@TH1H]2O[C@TH1H](CN)[C@TH2H](O)[C@TH1H](O)[C@TH1H]2N)=S)C6=O)=S)[C@TH2H]([C@TH1H](O)[C@TH2H]1O[C@TH2H]1[C@TH2H](O)[C@TH1H](N)C[C@TH1H](N)[C@TH1H]1O[C@TH1H]1O[C@TH1H](CN)[C@TH2H](O)[C@TH1H](O)[C@TH1H]1N)O[C@TH1H]1O[C@TH2H](CN)[C@TH2H](O)[C@TH1H](O)[C@TH1H]1N", // 52881
            "CC[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)-[n+]1ccc(cc1)-c1cc[n+](cc1)-c1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC", // 826428
            "CC[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)-[n+]1ccc(cc1)-c1cc[n+](cc1)-c1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)Cc1cc(cc(c1)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC)C[n+]1ccc(cc1)-c1cc[n+](cc1)CC", // 1246825
            "CCC[CH]1CC[CH](CC1)[CH]1CC[CH](CC1)OC(=O)[CH]1[CH](c2ccc(O)cc2)[CH]([CH]1c1ccc(O)cc1)C(=O)O[CH]1CC[CH](CC1)[CH]1CC[CH](CCC)CC1", // CHEMBL415840
            "OC(c1ccccc1)C1(c2ccccc2)C23c4c5c6c7c8c9c(c%10c%11c2c2c4c4c%12c5c5c6c6c8c8c%13c9c9c%10c%10c%11c%11c2c2c4c4c%12c%12c5c5c6c8c6c8c%13c9c9c%10c%10c%11c2c2c4c4c%12c5c6c5c8c9c%10c2c45)C137", // 408840
       ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        for td in test_data.iter() {
            let p = Parser::new(td);
            let mut m = p.parse().unwrap();
            assert_eq!(td, &m.to_smiles().unwrap());
        }
    }

    #[test]
    fn test_performance() {
        let p = Parser::new(
            r#"C[C@H](CC[C@@H]([C@@H]([C@H](C)C[C@H](C(=C)/C(=C/CO)/C)O)O)OS(=O)(=O)[O-])[C@H2]C"#,
            //     
        );
        let mut m = p.parse().unwrap();
        println!("{}", m.to_smiles().unwrap());
    }
}
