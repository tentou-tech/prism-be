# Prism Backend Service

A Rust-based backend service that provides account management functionality with wallet addresses and cryptographic signatures.

## Features

- Account creation with wallet address and signature verification
- Key management for accounts
- Data storage with signature verification
- Health check endpoint

## API Endpoints

### Health Check
```http
GET /v1/health
```
Returns the health status of the service.

**Response**: `200 OK` with body "OK"

### Create Account
```http
POST /v1/account/create
```
Creates a new account with wallet address and signature verification.

**Request Body**:
```json
{
    "wallet_address": "string",
    "pub_key": "string",
    "signature": SignatureBundle
}
```

**Response**: `200 OK`
```json
{
    "id": "string"
}
```

### Add Key
```http
POST /v1/account/add_key
```
Adds a new public key to an existing account.

**Request Body**:
```json
{
    "wallet_address": "string",
    "pub_key": "string",
    "signature": SignatureBundle
}
```

**Response**: `200 OK`
```json
{
    "id": "string"
}
```

### Add Data
```http
POST /v1/account/add_data
```
Adds data to an account with signature verification.

**Request Body**:
```json
{
    "wallet_address": "string",
    "data": "bytes",
    "data_signature": SignatureBundle,
    "signature": SignatureBundle
}
```

**Response**: `200 OK`
```json
{
    "id": "string"
}
```

## Technical Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - A modern Rust web framework
- **Runtime**: [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust
- **Serialization**: [Serde](https://serde.rs/) - Serialization/deserialization framework
- **Cryptography**: Custom signature verification using `SignatureBundle` and `VerifyingKey`

## Configuration

The service configuration includes:
- Server binding to `0.0.0.0` with configurable port
- State management using `Arc<AppState>`
- Configuration managed through `Config` struct

## Development

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

### Running the Service

1. Clone the repository
2. Build the project:
```bash
cargo build
```
3. Run the service:
```bash
RUST_LOG=debug cargo run
```

## Security Features

The service implements several security measures:
- Cryptographic signature verification for all account operations
- Wallet address validation
- Public key verification
- Separate signatures for data and operations
- Thread-safe state management using `Arc`

## Error Handling

Current implementation uses `unwrap()` for error handling. In production, it's recommended to:
- Implement proper error propagation
- Add custom error types
- Handle all potential failure cases
- Add proper logging and monitoring

## Project Structure

```
src/
  ├── server.rs    - Main server implementation with route handlers
  ├── app.rs       - Application state management
  ├── config.rs    - Configuration handling
  └── ops/         - Core operations implementation
```

## Contributing

[Add contribution guidelines here]

## License

[Add your license information here]
```

This updated README maintains the existing structure while adding:
1. More detailed API documentation
2. Technical implementation details
3. Security considerations
4. Project structure
5. Clear development instructions

Would you like me to:
1. Add more specific details about any section?
2. Include additional sections?
3. Modify the formatting or structure?
4. Add examples of using the API endpoints?