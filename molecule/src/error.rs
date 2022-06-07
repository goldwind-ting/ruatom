use graph::error::GraphError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, MoleculeError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MoleculeError {
    #[error("grapherror: `{0}`")]
    GraphError(#[from] GraphError),

    #[error("invalid ringbond")]
    InvalidRingBond,

    #[error("two vertices are not adjacent")]
    IlleageAdjacentVertix,

    #[error("invalid molecule: `{0}`")]
    IllegalMolecule(&'static str),

    #[error("cann't transform to another atom")]
    TransformError,

    #[error("not found isotope: `{0}`, {1}")]
    IsotopeError(&'static str, i16),

    #[error("not found symbol: `{0}`")]
    NotFoundSymbolError(String),
}
