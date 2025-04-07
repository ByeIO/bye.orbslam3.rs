use rust_mcp_schema::JsonrpcErrorError;
use rust_mcp_transport::error::TransportError;
use thiserror::Error;

pub type SdkResult<T> = core::result::Result<T, MCPSdkError>;

#[derive(Debug, Error)]
pub enum MCPSdkError {
    #[error("{0}")]
    JsonrpcErrorError(#[from] JsonrpcErrorError),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    TransportError(#[from] TransportError),
    #[error("{0}")]
    AnyErrorStatic(Box<(dyn std::error::Error + Send + Sync + 'static)>),
    #[error("{0}")]
    AnyError(Box<(dyn std::error::Error + Send + Sync + 'static)>),
    #[error("{0}")]
    SdkError(#[from] rust_mcp_schema::schema_utils::SdkError),
}
