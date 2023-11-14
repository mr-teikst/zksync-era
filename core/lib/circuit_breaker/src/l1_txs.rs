use crate::{CircuitBreaker, CircuitBreakerError};
use zksync_server_dal::ServerConnectionPool;

#[derive(Debug)]
pub struct FailedL1TransactionChecker {
    pub pool: ServerConnectionPool,
}

#[async_trait::async_trait]
impl CircuitBreaker for FailedL1TransactionChecker {
    async fn check(&self) -> Result<(), CircuitBreakerError> {
        if self
            .pool
            .access_storage()
            .await
            .unwrap()
            .eth_sender_dal()
            .get_number_of_failed_transactions()
            .await
            .unwrap()
            > 0
        {
            return Err(CircuitBreakerError::FailedL1Transaction);
        }
        Ok(())
    }
}
