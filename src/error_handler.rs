#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to open spreadsheet file: {0}")]
    Table(#[from] calamine::Error),

    #[error("carapax ExecuteError: {0}")]
    Carapax(#[from] carapax::ExecuteError),

    #[error("TryFromIntError: {0}")]
    Vector(#[from] std::num::TryFromIntError),

    #[error("InputFileErr: {0}")]
    InputFile(#[from] std::io::Error),

    #[error("sql error: {0}")]
    Sqlite(#[from] sqlite::Error),

    #[error("CARAPAX_TOKEN is not set: {0}")]
    Env(#[from] std::env::VarError),

    #[error("failled to create API: {0}")]
    Api(#[from] carapax::ApiError),

    #[error("failled to make caption bold: {0}")]
    Caption(#[from] carapax::types::TextEntityError),
}
