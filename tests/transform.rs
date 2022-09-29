use ruatom::{molecule::expand, Parser};

#[test]
fn test_expand_ethanol() {
    let mol = Parser::new("CCO").parse().unwrap();
    let mut expanded = expand(&mol).unwrap();
    let smiles = expanded.to_smiles().unwrap();
    assert_eq!(smiles, "[CH3][CH2][OH]");
}

#[test]
fn test_expand_benzene() {
    let mol = Parser::new("c1ccccc1").parse().unwrap();
    let mut expanded = expand(&mol).unwrap();
    let smiles = expanded.to_smiles().unwrap();
    assert!(smiles.contains("c"));
}

#[test]
fn test_expand_methane() {
    let mol = Parser::new("C").parse().unwrap();
    let mut expanded = expand(&mol).unwrap();
    let smiles = expanded.to_smiles().unwrap();
    assert_eq!(smiles, "[CH4]");
}

#[test]
fn test_expand_ammonia() {
    let mol = Parser::new("N").parse().unwrap();
    let mut expanded = expand(&mol).unwrap();
    let smiles = expanded.to_smiles().unwrap();
    assert_eq!(smiles, "[NH3]");
}

#[test]
fn test_expand_water() {
    let mol = Parser::new("O").parse().unwrap();
    let mut expanded = expand(&mol).unwrap();
    let smiles = expanded.to_smiles().unwrap();
    assert_eq!(smiles, "[OH2]");
}

#[test]
fn test_expand_methanol() {
    let mol = Parser::new("CO").parse().unwrap();
    let mut expanded = expand(&mol).unwrap();
    let smiles = expanded.to_smiles().unwrap();
    assert!(
        smiles.contains("[CH") && smiles.contains("[O"),
        "Expected expanded methanol with explicit H, got: {}",
        smiles
    );
}

#[test]
fn test_atom_based_db_stereo_to_trigonal() {
    use ruatom::molecule::transform::{
        atom_based_db_stereo, explicit_to_implicit, implicit_to_explicit,
    };
    let mol = Parser::new("F/C=C/F").parse().unwrap();
    let m1 = implicit_to_explicit(&mol).unwrap();
    let m2 = atom_based_db_stereo(&m1).unwrap();
    let mut m3 = explicit_to_implicit(&m2).unwrap();
    let smiles = m3.to_smiles().unwrap();
    assert_eq!(smiles, "F[C@H]=[C@@H]F");
}
