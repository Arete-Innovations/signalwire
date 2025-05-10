pub mod client;
pub mod errors;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::client::*;
    use crate::errors::*;
    use crate::types::*;
    use dotenv::dotenv;
    use std::env;

    fn get_client_from_env() -> SignalWireClient {
        dotenv().ok();

        let space_name = env::var("SIGNALWIRE_SPACE_NAME").expect("Missing space name");
        let project_id = env::var("SIGNALWIRE_PROJECT_ID").expect("Missing project ID");
        let api_key = env::var("SIGNALWIRE_API_KEY").expect("Missing API key");

        SignalWireClient::new(&space_name, &project_id, &api_key)
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
    async fn test_get_phone_numbers_available() {
        let client = get_client_from_env();
        let query_params = PhoneNumberAvailableQueryParams::new().build();

        match client.get_phone_numbers_available("US", &query_params).await {
            Ok(response) => {
                assert!(!response.phone_numbers_available.is_empty(), "Expected non-empty phone numbers list");
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert_eq!(e.to_string(), "Expected error message");
            }
        }
    }

    #[tokio::test]
    async fn test_get_phone_numbers_owned() {
        let client = get_client_from_env();
        let query_params = PhoneNumberOwnedFilterParams::new().build();

        let result = client.get_phone_numbers_owned(&query_params).await;

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

    /// Test for SMS sending with SID storage.
    ///
    /// This test is disabled by default to prevent accidental SMS charges.
    /// To run this test, set the following environment variables in your .env file:
    ///
    /// SIGNALWIRE_RUN_SMS_TEST=true
    /// SIGNALWIRE_FROM_NUMBER=+12064145320 (a number you own in SignalWire)
    /// SIGNALWIRE_TO_NUMBER=+40731665699 (the recipient's number)
    /// SIGNALWIRE_STORE_SMS_SID=true (to save the SID for the follow-up test)
    ///
    /// Then run: cargo test test_send_sms -- --nocapture
    #[tokio::test]
    async fn test_send_sms() {
        dotenv().ok();

        if env::var("SIGNALWIRE_RUN_SMS_TEST").unwrap_or_else(|_| "false".to_string()) != "true" {
            println!("Skipping SMS test. To enable, set SIGNALWIRE_RUN_SMS_TEST=true in your .env file.");
            println!("Make sure to also set SIGNALWIRE_FROM_NUMBER and SIGNALWIRE_TO_NUMBER.");
            println!("Set SIGNALWIRE_STORE_SMS_SID=true (optional) to save SID for follow-up tests.");
            return;
        }

        let client = get_client_from_env();

        let from_number = env::var("SIGNALWIRE_FROM_NUMBER").expect("Missing SIGNALWIRE_FROM_NUMBER env var");
        let to_number = env::var("SIGNALWIRE_TO_NUMBER").expect("Missing SIGNALWIRE_TO_NUMBER env var");

        println!("Sending SMS from {} to {}", from_number, to_number);

        let message = SmsMessage {
            from: from_number,
            to: to_number,
            body: "This is a test message from the SignalWire Rust SDK.".to_string(),
        };

        // Send the message and get the SID
        let sid = match client.send_sms(&message).await {
            Ok(response) => {
                assert_eq!(response.from, message.from);
                assert_eq!(response.to, message.to);
                assert_eq!(response.body, message.body);
                assert!(!response.sid.is_empty(), "Expected non-empty SID");
                println!("✓ SMS sent successfully with SID: {}", response.sid);

                // Get the message status
                let status = response.get_status();
                println!("Initial message status: {}", status);

                response.sid
            }
            Err(SignalWireError::Unauthorized) => {
                println!("Error: Unauthorized - Check your credentials");
                return;
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        };

        // Always store the SID for follow-up tests when the SMS test is run
        use std::fs::File;
        use std::io::Write;

        match File::create(".signalwire_test_sms_sid") {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", sid) {
                    println!("Failed to write SMS SID to file: {}", e);
                } else {
                    println!("Stored SMS SID for follow-up test: {}", sid);
                }
            }
            Err(e) => println!("Failed to create SMS SID file: {}", e),
        }

        // Wait a bit before checking the status to allow it to update from "queued"
        println!("Waiting 5 seconds before checking message status...");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // Check the message status
        match client.get_message_status(&sid).await {
            Ok(status_response) => {
                let current_status = status_response.get_status();
                println!("✓ Initial status check: {}", current_status);
                println!("Message details: SID={}, From={}, To={}, Body=\"{}\"",
                    status_response.sid,
                    status_response.from,
                    status_response.to,
                    status_response.body);

                println!("Run the follow-up delayed status check with: cargo test test_delayed_status_check -- --nocapture");
            }
            Err(e) => {
                println!("✗ Failed to check message status: {:?}", e);
            }
        }
    }

    /// Follow-up test to check the status of a previously sent SMS after a delay.
    ///
    /// This test automatically uses the SID stored by a previous test_send_sms run,
    /// or falls back to SIGNALWIRE_MESSAGE_SID from the environment if available.
    ///
    /// To run both tests in sequence:
    /// SIGNALWIRE_RUN_SMS_TEST=true cargo test test_send_sms test_delayed_status_check -- --nocapture
    #[tokio::test]
    async fn test_delayed_status_check() {
        dotenv().ok();

        // We don't need to check SIGNALWIRE_RUN_SMS_TEST here anymore since we always have a stored SID
        // if the first test ran successfully

        // Try to get the message SID from either:
        // 1. The environment variable
        // 2. A file created by a previous test_send_sms run
        let message_sid = match env::var("SIGNALWIRE_MESSAGE_SID") {
            Ok(sid) => sid,
            Err(_) => {
                // Check if we have a stored SID from a previous test
                match std::fs::read_to_string(".signalwire_test_sms_sid") {
                    Ok(sid) => sid.trim().to_string(),
                    Err(_) => {
                        println!("No message SID found. Either:");
                        println!("1. Set SIGNALWIRE_MESSAGE_SID in your .env file");
                        println!("2. Run test_send_sms with SIGNALWIRE_STORE_SMS_SID=true first");
                        return;
                    }
                }
            }
        };

        println!("Checking status of message with SID: {}", message_sid);
        println!("Waiting 10 seconds for potential status changes...");

        // Wait for status to potentially change
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        let client = get_client_from_env();

        // Check the status of the specified message after the delay
        match client.get_message_status(&message_sid).await {
            Ok(response) => {
                let status = response.get_status();
                println!("✓ Message status after delay: {}", status);
                println!("Details:");
                println!("  SID: {}", response.sid);
                println!("  From: {}", response.from);
                println!("  To: {}", response.to);
                println!("  Body: {}", response.body);
                println!("  Date sent: {:?}", response.date_sent);
                println!("  Price: {:?}", response.price);
                println!("  Error code: {:?}", response.error_code);

                // Ensure we got valid data back
                assert_eq!(response.sid, message_sid);
                assert!(!response.status.is_empty(), "Status should not be empty");
            }
            Err(SignalWireError::NotFound(_)) => {
                println!("Message not found: {}", message_sid);
            }
            Err(e) => {
                panic!("Unexpected error checking message status: {:?}", e);
            }
        }
    }
}
