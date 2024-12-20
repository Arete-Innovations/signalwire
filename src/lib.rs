pub mod client;
pub mod errors;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::errors::*;
    use crate::types::*;
    use dotenv::dotenv;
    use std::env;

    fn get_client_from_env() -> Client {
        dotenv().ok();

        let space_name = env::var("SIGNALWIRE_SPACE_NAME").expect("Missing space name");
        let project_id = env::var("SIGNALWIRE_PROJECT_ID").expect("Missing project ID");
        let api_key = env::var("SIGNALWIRE_API_KEY").expect("Missing API key");

        Client::new(&space_name, &project_id, &api_key)
    }

    #[tokio::test]
    async fn test_get_jwt() {
        let client = get_client_from_env();
        let result = client.get_jwt().await;

        match result {
            Ok(jwt_response) => {
                assert!(!jwt_response.jwt_token.is_empty(), "JWT token should not be empty");
                assert!(!jwt_response.refresh_token.is_empty(), "Refresh token should not be empty");
            }
            Err(e) => {
                eprintln!("Observed error with test credentials: {:?}", e);
                assert!(false, "Test should fail with invalid credentials");
            }
        }
    }

    #[tokio::test]
    async fn test_get_available_phone_numbers() {
        let client = get_client_from_env();
        let query_params = AvailablePhoneNumberQueryParams::new().build();

        match client.get_available_phone_numbers("US", &query_params).await {
            Ok(response) => {
                assert!(!response.available_phone_numbers.is_empty(), "Expected non-empty phone numbers list");
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert_eq!(e.to_string(), "Expected error message");
            }
        }
    }

    #[tokio::test]
    async fn test_get_owned_phone_numbers() {
        let client = get_client_from_env();
        let query_params = OwnedPhoneNumberFilterParams::new().build();

        let result = client.get_owned_phone_numbers(&query_params).await;

        match result {
            Ok(phone_numbers) => {
                assert!(!phone_numbers.data.is_empty(), "Expected non-empty phone numbers list");
            }
            Err(SignalWireError::Unauthorized) => {
                println!("Error: Unauthorized - Test passed as expected.");
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}
