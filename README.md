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


## Signature Verification System

### SignatureBundle
The `SignatureBundle` is a crucial component used for cryptographic verification of operations. It contains:
- The signature data
- Additional metadata for verification

This structure is used in several contexts:
1. **Account Creation**: Verifies the ownership of the wallet address
2. **Key Addition**: Ensures only authorized users can add new keys
3. **Data Operations**: 
   - `data_signature`: Verifies the integrity of the data being stored
   - `signature`: Verifies the authorization of the operation itself

### Usage Examples

#### Creating an Account
```json
{
    "wallet_address": "0x123...abc",
    "pub_key": "base64_encoded_public_key",
    "signature": {
        // SignatureBundle fields
        // Include actual structure based on your implementation
    }
}
```

Adding Data with Double Signatures
```json
{
    "wallet_address": "0x123...abc",
    "data": "base64_encoded_data",
    "data_signature": {
        // SignatureBundle for data verification
    },
    "signature": {
        // SignatureBundle for operation authorization
    }
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

## Project Structure

```
src/
  ├── server.rs    - Main server implementation with route handlers
  ├── app.rs       - Application state management
  ├── config.rs    - Configuration handling
  └── ops/         - Core operations implementation
```
