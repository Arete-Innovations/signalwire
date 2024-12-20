# 📞 SignalWire SDK for Rust 🦀

The unofficial SDK for interacting with SignalWire's API using Rust. 
This library currently provides methods for authentication and managing phone numbers as it's still in development.

## 🚀 Features

- 🔐  **Authenticate**: Obtain JWT tokens for secure API access.
- 📞  **Phone Number Management**: Retrieve available and owned phone numbers.
- ⚡ **Asynchronous Support**: Built with async/await using Tokio.

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
signalwire = "0.1.1"
```

or install with cargo, in the root of your project:

```bash
cargo add signalwire
```

## ⚙️ Configuration

You can use environment variables to manage sensitive data. Create a  .env  file in your project root:

```bash
SIGNALWIRE_SPACE_NAME=your_space_name
SIGNALWIRE_PROJECT_ID=your_project_id
SIGNALWIRE_API_KEY=your_api_key
```

## 📚 Usage

### Initialize the Client

```rust
use signalwire::{client::Client, errors::SignalWireError};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), SignalWireError> {
    dotenv().ok();

    let space_name = env::var("SIGNALWIRE_SPACE_NAME").expect("Missing space name");
    let project_id = env::var("SIGNALWIRE_PROJECT_ID").expect("Missing project ID");
    let api_key = env::var("SIGNALWIRE_API_KEY").expect("Missing API key");

    let client = Client::new(&space_name, &project_id, &api_key);

    // Example: Get JWT
    let jwt_response = client.get_jwt().await?;
    println!("JWT Token: {}", jwt_response.jwt_token);

    Ok(())
}
```

### Get Available Phone Numbers

```rust
let query_params = vec![("AreaCode".to_string(), "212".to_string())];
let available_numbers = client.get_available_phone_numbers("US", &query_params).await?;
println!("Available numbers: {:?}", available_numbers);
```

### Get Owned Phone Numbers

```rust
let owned_numbers = client.get_owned_phone_numbers(&[]).await?;
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
