use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GraphError {
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
}
