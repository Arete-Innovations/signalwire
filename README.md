# 📞 SignalWire SDK for Rust 🦀

The unofficial SDK for interacting with SignalWire's API using Rust.
This library provides methods for authentication, phone number management, and messaging capabilities.

## 🚀 Features

- 🔐  **Authenticate**: Obtain JWT tokens for secure API access.
- 📞  **Phone Number Management**: Retrieve available and owned phone numbers.
- 📱  **SMS Messaging**: Send SMS messages and check delivery status.
- ⚡ **Asynchronous Support**: Built with async/await using Tokio.
- 🕛 **Blocking Support**: Support for synchronous operations.

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
signalwire = "0.1.6"
dotenv = "0.15.0"
tokio = { version = "1.42.0", features = ["full"] }
```

or install with cargo, in the root of your project:

```bash
cargo add signalwire
```

or you can request the blocking version:

```toml
[dependencies]
signalwire = { version = "0.1.6", features = ["blocking"] }
```

```bash
cargo add signalwire --features=blocking
```

## ⚙️ Configuration

You can use environment variables to manage sensitive data. Create a  .env  file in your project root:

```bash
SIGNALWIRE_SPACE_NAME=your_space_name
SIGNALWIRE_PROJECT_ID=your_project_id
SIGNALWIRE_API_KEY=your_api_key
```

## 📚 Usage (Async)

### Initialize the Client

```rust
use signalwire::{client::SignalWireClient, errors::SignalWireError};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), SignalWireError> {
    dotenv().ok();

    let space_name = env::var("SIGNALWIRE_SPACE_NAME").expect("Missing space name");
    let project_id = env::var("SIGNALWIRE_PROJECT_ID").expect("Missing project ID");
    let api_key = env::var("SIGNALWIRE_API_KEY").expect("Missing API key");

    let client = SignalWireClient::new(&space_name, &project_id, &api_key);

    // Example: Get JWT
    let jwt_response = client.get_jwt().await?;
    println!("JWT Token: {}", jwt_response.jwt_token);

    Ok(())
}
```

### Get Available Phone Numbers

```rust
let client = SignalWireClient::new(&space_name, &project_id, &api_key);
let query_params = PhoneNumberAvailableQueryParams::new().build();
let available_numbers = client.get_phone_numbers_available("US", &query_params).await?;
println!("Available numbers: {:?}", available_numbers);
```

### Get Owned Phone Numbers

```rust
let client = SignalWireClient::new(&space_name, &project_id, &api_key);
let query_params = PhoneNumberOwnedFilterParams::new().build();
let owned_numbers = client.get_phone_numbers_owned(&query_params).await?;
println!("Owned numbers: {:?}", owned_numbers);
```

### Send SMS Message

```rust
use signalwire::types::SmsMessage;

let client = SignalWireClient::new(&space_name, &project_id, &api_key);

// Create message
let message = SmsMessage {
    from: "+15551234567".to_string(),  // Your SignalWire phone number
    to: "+15557654321".to_string(),    // Recipient's phone number
    body: "Hello from SignalWire Rust SDK!".to_string(),
};

// Send the message
match client.send_sms(&message).await {
    Ok(response) => {
        println!("Message sent with SID: {}", response.sid);
        println!("Status: {}", response.status);
    },
    Err(e) => eprintln!("Failed to send message: {:?}", e),
}
```

### Check Message Status

```rust
// Check the status of a previously sent message
let message_sid = "previous-message-sid";

match client.get_message_status(message_sid).await {
    Ok(response) => {
        // Get enum representation of status
        let status = response.get_status();
        println!("Message status: {}", status);
        println!("Sent at: {:?}", response.date_sent);
        println!("Price: {:?}", response.price);
    },
    Err(e) => eprintln!("Failed to check message status: {:?}", e),
}
```

## 📚 Usage (Blocking)

With the `blocking` feature enabled, you can use synchronous versions of all methods:

```rust
use signalwire::{client::SignalWireClient, errors::SignalWireError, types::SmsMessage};
use dotenv::dotenv;
use std::env;

fn main() -> Result<(), SignalWireError> {
    dotenv().ok();

    let space_name = env::var("SIGNALWIRE_SPACE_NAME").expect("Missing space name");
    let project_id = env::var("SIGNALWIRE_PROJECT_ID").expect("Missing project ID");
    let api_key = env::var("SIGNALWIRE_API_KEY").expect("Missing API key");

    let client = SignalWireClient::new(&space_name, &project_id, &api_key);

    // Send SMS (blocking)
    let message = SmsMessage {
        from: "+15551234567".to_string(),
        to: "+15557654321".to_string(),
        body: "Hello from SignalWire Rust SDK!".to_string(),
    };

    let response = client.send_sms_blocking(&message)?;
    println!("Message sent with SID: {}", response.sid);

    // Check message status (blocking)
    let status_response = client.get_message_status_blocking(&response.sid)?;
    println!("Message status: {}", status_response.get_status());

    Ok(())
}
```

## 🛡️ Error Handling

The SDK provides a custom error type, `SignalWireError`, to handle various error scenarios, such as:

- `HttpError`: Issues with HTTP requests.
- `Unauthorized`: Authentication failures.
- `NotFound`: Resource not found (e.g., invalid message SID).
- `Unexpected`: Other unexpected errors.

## 📜 License

This project is licensed under the [BSD-3-Clause License](LICENSE)

## 🤝 Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## 📧 Contact

For questions or feedback, reach out to chiarel@tragdate.ninja

## 📝 Changelog

### 0.1.6
- Added SMS messaging functionality
- Added message status checking capability
- Added message status enum for better type safety
- Added NotFound error variant

### 0.1.5
- Initial release with JWT authentication
- Added phone number management
- Added blocking support
