

1.  **Architecture:** Always think architecturally. Proactively create new files for distinct responsibilities and place them in a logical folder structure.
2.  **File Size:** Ideal: < 100 lines. Absolute Max: 200 lines.
3.  **SRP (Single Responsibility Principle):** Aggressively separate concerns. Extract logic into distinct modules for:
    *   Business Logic & State Management
    *   Data Access & API Services
    *   Utility & Helper Functions
    *   UI / Presentation Components
    *   Configuration & Constants
    *   Data Models & Type Definitions


use this folder structure:

rust_project/
├── .env
├── Cargo.toml
├── assets/
├── templates/
├── database/
|
└── src/
    ├── main.rs             // Minimal entry point. Initializes utils and router.
    |
    ├── routes/             // The API Layer: Defines URL endpoints.
    ├── handlers/           // The Logic Layer: Orchestrates requests.
    ├── services/           // The External I/O Layer (Stripe, S3, Email).
    ├── repositories/       // The Internal I/O Layer (Postgres, Redis).
    ├── models/             // The Data Definition Layer.
    ├── middleware/         // The Request Filtering Layer.
    |
    └── utils/              // The Foundational Layer: Shared infrastructure.
        ├── mod.rs
        ├── config.rs       // Configuration loading.
        ├── database.rs     // Connection pool setup.
        ├── errors.rs       // The central error type.
        └── state.rs        // The Axum AppState struct.

go_project/
├── .env                    // Local secrets and configuration
├── go.mod                  // Project manifest
├── assets/                 // (Optional) For static files like images, CSS
├── templates/              // (Optional) For server-side rendered HTML views
├── database/               // For all .sql assets (migrations, raw queries)
|
├── cmd/                    // The mandatory Go entry point directory
│   └── server/
│       └── main.go         // Minimal entry point: wires together packages from `internal/`
|
└── internal/               // The private application code. Contents mirror Rust's `src/`.
    |
    ├── routes/             // Package for defining the router and mapping URLs to handlers
    ├── handlers/           // Package for HTTP handlers that orchestrate requests
    ├── services/           // Package for external I/O logic (Stripe, S3, Email)
    ├── repositories/       // Package for internal data access logic (Postgres, Redis)
    ├── models/             // Package for data models (structs)
    ├── middleware/         // Package for request filtering logic
    |
    └── utils/              // The Foundational Layer: Shared infrastructure
        ├── config/         // Package for configuration
        ├── database/       // Package for DB connection setup
        └── errors/         // Package for custom error types
