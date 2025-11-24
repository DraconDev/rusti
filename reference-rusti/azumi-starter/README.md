# 🦀 Azumi Starter - Rust Authentication Platform

A **production-ready authentication platform** built with **Rust**, **Axum**, **SQLx**, and **Rusti**. Features multi-provider OAuth, PostgreSQL database, admin dashboard, and modern server-side rendering.

## 🎯 What This Is

- **🚀 Fast Rust foundation** with real authentication & database
- **📊 Admin dashboard** with live user analytics  
- **🔐 OAuth ready** with server sessions
- **🎨 Modern UI** with server-side rendering using Rusti
- **🐳 Docker ready** for production deployment

## ✨ Features

### 🔐 Authentication System
- **OAuth 2.0 Integration** with external auth microservice
- Google, GitHub, Discord, Microsoft OAuth support
- **HTTP-only cookies** for maximum security
- **Server session validation** with instant response times
- User profile pages with real OAuth data
- Session management middleware

### 💾 Database Integration
- PostgreSQL with type-safe SQLx queries
- User and preference tables with triggers
- Real user data tracking
- User registration and analytics
- Database migrations built-in

### 📊 Admin Dashboard
- Total users count from database
- Signups today/this week tracking
- Recent users list with avatars
- Real-time data updates

### 🎨 Modern Stack
- **Axum** - High-performance web framework
- **SQLx** - Compile-time checked SQL queries
- **Rusti** - Type-safe HTML templating (like Go Templ)
- **HTMX** - Dynamic interactions without JavaScript frameworks
- **Tailwind CSS** - Modern styling via CDN

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (`rustup update`)
- PostgreSQL database
- Auth microservice running (or configure OAuth directly)

### Setup

```bash
# Clone or navigate to the project
cd azumi-starter

# Copy environment configuration
cp .env.example .env

# Edit .env with your settings
# Required: DATABASE_URL, AUTH_SERVICE_URL (if using external auth service)

# Install sqlx-cli for migrations
cargo install sqlx-cli --no-default-features --features postgres

# Create database
createdb azumi_starter

# Run migrations (or they'll auto-run on startup)
sqlx migrate run

# Build and run
cargo run

# Or use cargo-watch for auto-reload during development
cargo install cargo-watch
cargo watch -x run
```

**Visit:** `http://localhost:8081`

## 🔧 Configuration

Edit `.env` file:

```bash
# Server
SERVER_PORT=8081

# Database (required)
DATABASE_URL=postgresql://user:password@localhost:5432/azumi_starter

# Auth Service - External microservice handling OAuth
AUTH_SERVICE_URL=http://localhost:8080

# Admin
ADMIN_EMAIL=admin@example.com

# Logging
RUST_LOG=info,azumi_starter=debug
```

## 📁 Project Structure

```
azumi-starter/
├── Cargo.toml              # Dependencies and package config
├── .env.example            # Environment template
├── migrations/             # SQL migrations
│   └── 001_initial_schema.sql
│
├── src/
│   ├── main.rs            # Application entry point
│   ├── config.rs          # Configuration management
│   ├── error.rs           # Error handling
│   ├── routes.rs          # Route definitions
│   │
│   ├── db/                # Database models
│   │   ├── mod.rs
│   │   └── models.rs
│   │
│   ├── repositories/      # Data access layer
│   │   ├── user.rs
│   │   └── preferences.rs
│   │
│   ├── services/          # Business logic
│   │   ├── auth.rs        # Auth service client
│   │   └── session.rs     # Session management
│   │
│   ├── middleware/        # HTTP middleware
│   │   └── auth.rs        # Authentication middleware
│   │
│   ├── handlers/          # Request handlers
│   │   ├── home.rs
│   │   ├── auth.rs
│   │   ├── profile.rs
│   │   └── admin.rs
│   │
│   └── templates/         # Rusti templates
│       ├── layout.rs      # Base layout
│       ├── home.rs        # Homepage
│       ├── login.rs       # Login page
│       ├── profile.rs     # User profile
│       └── admin.rs       # Admin dashboard
```

## 🌐 Routes

### Public Routes
- `GET /` - Homepage
- `GET /login` - Login page with OAuth providers
- `GET /auth/callback` - OAuth callback handler
- `GET /health` - Health check endpoint

### API Routes
- `POST /api/auth/exchange-code` - Exchange OAuth code for session
- `POST /api/auth/logout` - Logout and invalidate session

### Protected Routes (require authentication)
- `GET /profile` - User profile page
- `GET /admin` - Admin dashboard with analytics

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture

# Check code compiles
cargo check

# Run clippy lints
cargo clippy
```

## 🐳 Docker

Build and run with Docker:

```bash
# Build image
docker build -t azumi-starter .

# Run container
docker run -p 8081:8081 \
  -e DATABASE_URL=postgresql://... \
  -e AUTH_SERVICE_URL=http://auth:8080 \
  azumi-starter
```

## 🎨 Rusti Templates

Rusti provides type-safe, Rust-native HTML templating similar to Go's Templ:

```rust
use rusti::rusti;

pub fn hello(name: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div class="greeting">
            <h1>"Hello, " {name} "!"</h1>
            @if !name.is_empty() {
                <p>"Welcome back!"</p>
            }
        </div>
    }
}
```

Features:
- **Type-safe** - Compile-time template checking
- **Zero-cost** - Templates compile to efficient Rust code
- **Component composition** - Reusable components with `@component_name(args)`
- **Control flow** - `@if`, `@for`, `@match` expressions
- **Dynamic content** - Inject Rust expressions with `{expr}`

## 📊 Architecture

This platform follows a microservice architecture:

```
┌─────────────────────┐
│   Main App (8081)   │  ← This project
│  Rust + Axum +SQLx  │
│   Rusti Templates   │
└──────────┬──────────┘
           │
           │ HTTP
           ├─────────────┐
           │             │
┌──────────▼──────────┐  │
│  Auth MS (8080)     │  │
│  OAuth + Sessions   │  │
│  Redis-backed       │  │
└─────────────────────┘  │
                         │
                ┌────────▼─────────┐
                │  PostgreSQL      │
                │  User Data       │
                └──────────────────┘
```

## 📈 Performance

- **Response Times:** ~5-10ms with session validation
- **Database:** Type-safe SQLx queries with connection pooling
- **Templates:** Zero-cost Rusti templates compiled at build time
- **Memory:** Rust's efficient memory management
- **Concurrency:** Async/await with Tokio runtime

## 🔒 Security

- **HTTP-only cookies** - Prevents XSS attacks
- **Session validation** on every protected request
- **SQL injection protection** via SQLx prepared statements
- **Type safety** throughout the stack
- **No unsafe code** in application layer

## 🚧 Roadmap

- [ ] Email/password authentication
- [ ] Two-factor authentication (2FA)
- [ ] User roles and permissions
- [ ] Payment integration (Stripe)
- [ ] API rate limiting
- [ ] WebSocket support for real-time features
- [ ] Comprehensive test suite
- [ ] CI/CD pipeline
- [ ] Performance benchmarks

## 📝 License

MIT License

---

**Built with Rust. Fast, safe, and reliable.**
