//! Paystack gateway.
//!
//! Behaviour is unchanged from the code this replaced in `wallet_service.rs`:
//! verification is still a placeholder that approves every reference, and no
//! HTTP call is made yet. The intended Paystack call is kept below so it can
//! be filled in here without touching the wallet service.

use super::{DepositInitiation, PaymentError, PaymentProvider, PaymentStatus};
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, Default)]
pub struct PaystackProvider;

#[async_trait]
impl PaymentProvider for PaystackProvider {
    fn name(&self) -> &'static str {
        "paystack"
    }

    async fn initiate_deposit(
        &self,
        reference: &str,
        _amount: i64,
        _currency: &str,
    ) -> Result<DepositInitiation, PaymentError> {
        // TODO: POST https://api.paystack.co/transaction/initialize and return
        // its authorization_url. Until then the client completes payment with
        // the reference the wallet already records.
        Ok(DepositInitiation {
            reference: reference.to_string(),
            authorization_url: None,
        })
    }

    async fn verify_deposit(
        &self,
        _reference: &str,
        _expected_amount: i64,
    ) -> Result<bool, PaymentError> {
        // TODO: Implement actual Paystack API call
        //
        // let client = reqwest::Client::new();
        // let paystack_secret = std::env::var("PAYSTACK_SECRET")
        //     .map_err(|_| PaymentError::Provider("PAYSTACK_SECRET not set".into()))?;
        //
        // let response = client
        //     .get(&format!("https://api.paystack.co/transaction/verify/{}", reference))
        //     .header("Authorization", format!("Bearer {}", paystack_secret))
        //     .send()
        //     .await
        //     .map_err(|e| PaymentError::Provider(e.to_string()))?;
        //
        // let data: PaystackResponse = response.json().await
        //     .map_err(|e| PaymentError::Provider(e.to_string()))?;
        //
        // Ok(data.data.status == "success" && data.data.amount == expected_amount)
        tracing::warn!("Paystack verification not implemented, returning true for testing");
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
            provider: "paystack",
            operation: "initiate_withdrawal",
        })
    }

    async fn check_status(&self, _reference: &str) -> Result<PaymentStatus, PaymentError> {
        Err(PaymentError::NotImplemented {
            provider: "paystack",
            operation: "check_status",
        })
    }
}
