//! Flutterwave gateway.
//!
//! Behaviour is unchanged from the code this replaced in `wallet_service.rs`:
//! verification is still a placeholder that approves every reference, and no
//! HTTP call is made yet.

use super::{DepositInitiation, PaymentError, PaymentProvider, PaymentStatus};
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, Default)]
pub struct FlutterwaveProvider;

#[async_trait]
impl PaymentProvider for FlutterwaveProvider {
    fn name(&self) -> &'static str {
        "flutterwave"
    }

    async fn initiate_deposit(
        &self,
        reference: &str,
        _amount: i64,
        _currency: &str,
    ) -> Result<DepositInitiation, PaymentError> {
        // TODO: POST https://api.flutterwave.com/v3/payments and return its
        // payment link.
        Ok(DepositInitiation {
            reference: reference.to_string(),
            authorization_url: None,
        })
    }

    async fn verify_deposit(
        &self,
        _transaction_id: &str,
        _expected_amount: i64,
    ) -> Result<bool, PaymentError> {
        // TODO: Implement actual Flutterwave API call
        tracing::warn!("Flutterwave verification not implemented, returning true for testing");
        Ok(true)
    }

    async fn initiate_withdrawal(
        &self,
        _reference: &str,
        _amount: i64,
        _currency: &str,
        _destination: &str,
    ) -> Result<PaymentStatus, PaymentError> {
        Err(PaymentError::NotImplemented {
            provider: "flutterwave",
            operation: "initiate_withdrawal",
        })
    }

    async fn check_status(&self, _reference: &str) -> Result<PaymentStatus, PaymentError> {
        Err(PaymentError::NotImplemented {
            provider: "flutterwave",
            operation: "check_status",
        })
    }
}
