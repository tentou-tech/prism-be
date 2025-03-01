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

### Request Create Account
```http
POST /v1/account/request-create
```
Initiates the account creation process by requesting a payload to sign.

**Request Body**:
```json
{
    "id": "string",
    "verifying_key": "string"
}
```

**Response**: `200 OK`
```json
{
    "payload": "bytes"
}
```

### Send Create Account
```http
POST /v1/account/send-create
```
Completes the account creation process with the signed payload.

**Request Body**:
```json
{
    "id": "string",
    "verifying_key": "string",
    "signature": "string"
}
```

**Response**: `200 OK`
```json
{
    "id": "string"
}
```

### Get Account
```http
GET /v1/account/get?id=string
```
Retrieves account information including keys, data, and nonce.

**Response**: `200 OK`
```json
{
    "id": "string",
    "nonce": "number",
    "data": ["string"],
    "keys": ["string"]
}
```

### Add Key
```http
POST /v1/account/add-key
```
Adds a new key to an existing account.

**Request Body**:
```json
{
    "id": "string",
    "verifying_key": "string"
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
POST /v1/account/add-data
```
Adds data to an existing account.

**Request Body**:
```json
{
    "id": "string",
    "data": "string"
}
```

**Response**: `200 OK`
```json
{
    "id": "string"
}
```

### Get Data
```http
GET /v1/account/get-data?id=string
```
Retrieves data associated with an account.

**Response**: `200 OK`
```json
{
    "data": ["string"]
}
```

### Get Key
```http
GET /v1/account/get-key?id=string
```
Retrieves keys associated with an account.

**Response**: `200 OK`
```json
{
    "key": ["string"]
}
```

### List Accounts
```http
GET /v1/account/list-accounts
```
Lists all accounts with their associated information.

**Response**: `200 OK`
```json
{
    "accounts": [
        {
            "id": "string",
            "nonce": "number",
            "data": ["string"],
            "keys": ["string"]
        }
    ]
}
```

### List Keys
```http
GET /v1/account/list-keys/:id
```
Lists all keys for a specific account.

**Response**: `200 OK`
```json
["string"]
```

### Add Account (Manual)
```http
POST /v1/account/add-manual
```
Manually adds an account (for administrative purposes).

**Request Body**:
```json
{
    "id": "string"
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
- **CORS**: Enabled for all origins, methods, and headers

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