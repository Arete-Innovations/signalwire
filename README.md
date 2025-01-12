# 📞 SignalWire SDK for Rust 🦀

The unofficial SDK for interacting with SignalWire's API using Rust. 
This library currently provides methods for authentication and managing phone numbers as it's still in development.

## 🚀 Features

- 🔐  **Authenticate**: Obtain JWT tokens for secure API access.
- 📞  **Phone Number Management**: Retrieve available and owned phone numbers.
- ⚡ **Asynchronous Support**: Built with async/await using Tokio.
- 🕛 **Blocking Support**: Support for synchronous operations.

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
signalwire = "0.1.5"
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
signalwire = { version = "0.1.5", features = ["blocking"] }
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

## 📚 Usage ( Async )

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

## 🛡️ Error Handling

The SDK provides a custom error type,  SignalWireError , to handle various error scenarios, such as:

- HttpError : Issues with HTTP requests.
- Unauthorized : Authentication failures.
- Unexpected : Other unexpected errors.

## 📜 License

This project is licensed under the [BSD-3-Clause License](LICENSE)

## 🤝 Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## 📧 Contact

For questions or feedback, reach out to chiarel@tragdate.ninja
