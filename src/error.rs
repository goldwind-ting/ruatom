use thiserror::Error;

pub type Result<T> = std::result::Result<T, RuatomError>;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RuatomError {
    #[error("no such vertex `{0}`")]
    NoSuchVertex(u8),
    #[error("already existed vertex `{0}`")]
    ExistedVertex(u8),
    #[error("edge existed: : `{0}` `{1}`")]
    ExistedEdge(u8, u8),
    #[error("no such edge: `{0}` `{1}`")]
    NoSuchEdge(u8, u8),
    #[error("invalid edge: `{0}` `{1}`")]
    InvalidEdge(u8, u8),
    #[error("not any edge include this vertex: `{0}`")]
    NoEdgeInclude(u8),
    #[error("unknown error")]
    Unknown,

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

    #[error("invalid SMILES: `{0}`")]
    IllegalSMILES(&'static str),

    #[error("std error: `{0}`")]
    StdError(String),
}
