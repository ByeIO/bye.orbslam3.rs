use rust_mcp_schema::schema_utils::SdkError;
use tokio::time::{timeout, Duration};

use crate::error::{TransportError, TransportResult};

pub async fn await_timeout<F, T, E>(operation: F, timeout_duration: Duration) -> TransportResult<T>
where
    F: std::future::Future<Output = Result<T, E>>, // The operation returns a Result
    E: Into<TransportError>, // The error type must be convertible to TransportError
{
    match timeout(timeout_duration, operation).await {
        Ok(result) => result.map_err(|err| err.into()), // Convert the error type into TransportError
        Err(_) => Err(SdkError::request_timeout(timeout_duration.as_millis()).into()), // Timeout error
    }
}
