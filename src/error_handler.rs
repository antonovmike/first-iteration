#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to open spreadsheet file: {0}")]
    TableErr(#[from] calamine::Error),

    #[error("carapax ExecuteError: {0}")]
    CarapaxErr(#[from] carapax::ExecuteError),

    #[error("TryFromIntError: {0}")]
    VectorErr(#[from] std::num::TryFromIntError),

    #[error("InputFileErr: {0}")]
    InputFileErr(#[from] std::io::Error),

    #[error("sql error: {0}")]
    SqlError(#[from] sqlite::Error),

    #[error("CARAPAX_TOKEN is not set: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("failled to create API: {0}")]
    ApiError(#[from] carapax::ApiError),
}
