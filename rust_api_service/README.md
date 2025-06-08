# Rust RESTful API

A Rust implementation of a RESTful API service, migrated from the original Ruby Sinatra application.

## Requirements

- Rust 1.83.0 or later
- PostgreSQL (for production) or SQLite (for development)

## Installation

```bash
# Clone the repository
git clone https://github.com/dfbustosus/dot_rust.git
cd dot_rust/rust_api_service

# Copy the example environment file and edit as needed
cp .env.example .env

# Build the project
cargo build
```

## Starting the Server

```bash
# Run in development mode
cargo run

# Run in release mode
cargo run --release
```

The API will be available at [http://localhost:4567](http://localhost:4567)

## API Endpoints

### GET /

- Returns a welcome message
- Response: `{"message": "Welcome to the RESTful API"}`

### GET /api/items

- Returns all items
- Response: Array of item objects

### GET /api/items/:id

- Returns a specific item by ID
- Response: Item object or 404 error

### POST /api/items

- Creates a new item
- Request body: `{"name": "Item name", "description": "Item description"}`
- Response: Created item with status 201

### PUT /api/items/:id

- Updates an existing item
- Request body: `{"name": "Updated name", "description": "Updated description"}`
- Response: Updated item or 404 error

### DELETE /api/items/:id

- Deletes an item
- Response: Status 204 (No Content) or 404 error

## Project Structure

```
rust_api_service/
├── src/
│   ├── config.rs         # Application configuration
│   ├── db.rs             # Database connection and initialization
│   ├── errors.rs         # Error handling
│   ├── main.rs           # Application entry point
│   ├── models/           # Data models
│   │   ├── item.rs       # Item model
│   │   └── mod.rs        # Models module
│   └── routes/           # API routes
│       ├── items.rs      # Item routes
│       └── mod.rs        # Routes module
├── .env.example          # Example environment variables
├── Cargo.toml            # Project dependencies
└── README.md             # Project documentation
```

## Differences from Ruby Version

This Rust implementation provides the same functionality as the original Ruby version with the following technologies:

- **Web Framework**: Actix Web instead of Sinatra
- **ORM**: SQLx instead of Sequel
- **Database**: PostgreSQL/SQLite (same as original)
- **Middleware**: 
  - Actix CORS instead of Rack CORS
  - Actix Logger instead of RequestLogger
  - Actix Rate Limit instead of Rack::Attack
