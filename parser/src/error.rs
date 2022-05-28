use molecule::error::MoleculeError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RuatomError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RuatomError {
    #[error("empty SMILES")]
    MoleculeError(#[from] MoleculeError),

    #[error("invalid SMILES: `{0}`")]
    IllegalSMILES(&'static str),
}
