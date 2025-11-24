# SaaS Starter - Setup Guide

This guide will help you set up and deploy your SaaS application.

## Prerequisites

- **Auth Microservice** running (provides OAuth login & session management)
- **Payment Microservice** running (handles Stripe subscriptions)
- **PostgreSQL** database
- **Go 1.21+** and **Node.js** (for development)

## Quick Start

### 1. Clone and Install

```bash
# Clone the repository
git clone <your-repo-url>
cd go-templ-htmx-ex

# Install dependencies
go mod download

# Install frontend tooling
npm install -g @tailwindcss/cli
```

### 2. Configure Environment

```bash
# Copy example env file
cp .env.example .env

# Edit .env with your values
nano .env
```

**Required Environment Variables:**
```bash
# Auth Service
AUTH_SERVICE_URL=http://localhost:8080
REDIRECT_URL=http://localhost:3000

# Database
DB_URL=postgresql://user:pass@host:5432/dbname?sslmode=disable

# Payment Microservice
PAYMENT_MS_URL=http://localhost:9000
PAYMENT_MS_API_KEY=your-payment-ms-api-key
```

### 3. Run Database Migrations

```bash
# Apply migrations
psql $DB_URL -f database/migrations/001_initial_schema.sql
psql $DB_URL -f database/migrations/002_user_preferences.sql

# Generate SQLC code
sqlc generate
```

### 4. Register Stripe Products

**Option A: Config File Setup (Recommended)**

```bash
# 1. Create your product config
cp scripts/setup-config.example.json scripts/setup-config.json

# 2. Edit the config with your product details
nano scripts/setup-config.json

# 3. Run setup script (reads from config)
chmod +x scripts/setup-products.sh
./scripts/setup-products.sh
```

**Option B: Interactive Setup**

```bash
# Run without config file - will prompt for details
./scripts/setup-products.sh
```

The script will:
- Read from `scripts/setup-config.json` if it exists
- Otherwise, prompt for your project name and use defaults
- Call Payment MS to create Stripe products
- Auto-update your `.env` with Product/Price IDs

**Note:** Uses the same `PAYMENT_MS_API_KEY` from your `.env` - no separate admin key needed.

**Option C: Manual Setup**

If Payment MS doesn't have the `/admin/products/register` endpoint yet:

1. Manually create products in Stripe Dashboard
2. Add IDs to `.env`:
   ```bash
   STRIPE_PRODUCT_PRO=prod_YourProductID
   STRIPE_PRICE_MONTHLY=price_YourMonthlyPriceID
   STRIPE_PRICE_YEARLY=price_YourYearlyPriceID
   ```

### Product Config File Reference

**`scripts/setup-config.json` format:**
```json
{
  "project_name": "my-saas-app",
  "plans": [
    {
      "name": "Pro Plan",
      "description": "Professional features",
      "features": ["unlimited_projects", "api_access"],
      "pricing": {
        "monthly": 2900,
        "yearly": 29000
      }
    }
  ]
}
```

**Benefits:**
- ✅ No prompts - fully automated
- ✅ Repeatable setups
- ✅ Version control your product config
- ✅ Easy to add multiple plans

### 5. Start the Application

```bash
# Development mode with hot reload
make air

# Or build and run
go build -o bin/server cmd/server/main.go
./bin/server
```

Visit `http://localhost:3000`

---

## Deployment Guide

### 1. Environment-Specific Configuration

For each environment (dev, staging, production):

1. **Create separate `.env` files**
2. **Run `setup-products.sh`** to register environment-specific products
3. **Use different Stripe keys** for test vs live mode

### 2. Multiple Projects on Same Stripe Account

If you're deploying multiple SaaS products:

```bash
# Project 1: TodoApp
cd todoapp
./scripts/setup-products.sh
# Enter project name: "todoapp"
# Creates: prod_TodoApp123, price_TodoAppMonthly456, price_TodoAppYearly789

# Project 2: NotesApp
cd notesapp
./scripts/setup-products.sh
# Enter project name: "notesapp"
# Creates: prod_NotesApp123, price_NotesAppMonthly456, price_NotesAppYearly789
```

Each project gets its own Stripe products for clean revenue tracking.

### 3. Container Deployment

```dockerfile
# Dockerfile
FROM golang:1.21-alpine AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN go build -o server cmd/server/main.go

FROM alpine:latest
RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY --from=builder /app/server .
COPY --from=builder /app/templates ./templates
EXPOSE 3000
CMD ["./server"]
```

**Environment variables via secrets:**
```bash
kubectl create secret generic app-secrets \
  --from-env-file=.env.production
```

---

## Payment MS Integration

### Required Payment MS Endpoint

For automated product setup, Payment MS needs this endpoint:

**`POST /admin/products/register`**

**Request:**
```json
{
  "project_name": "my-awesome-saas",
  "plans": [{
    "name": "Pro Plan",
    "features": ["unlimited_projects", "api_access"],
    "pricing": {
      "monthly": 2900,
      "yearly": 29000
    }
  }]
}
```

**Response:**
```json
{
  "success": true,
  "project_id": "my-awesome-saas",
  "products": [{
    "plan_name": "Pro Plan",
    "stripe_product_id": "prod_ABC123",
    "prices": {
      "monthly": {
        "stripe_price_id": "price_XYZ789",
        "amount": 2900,
        "interval": "month"
      },
      "yearly": {
        "stripe_price_id": "price_DEF456",
        "amount": 29000,
        "interval": "year"
      }
    }
  }]
}
```

**Implementation Guide:** See [PAYMENT_MS_SETUP.md](./PAYMENT_MS_SETUP.md)

---

## Troubleshooting

### Products Not Creating

**Error:** `Payment MS returned status 404`

**Solution:**
- Ensure Payment MS has `/admin/products/register` endpoint
- Check `PAYMENT_MS_URL` and `PAYMENT_MS_API_KEY` in `.env`
- Verify Payment MS is running

### Checkout Failing

**Error:** `Invalid price ID`

**Solution:**
- Run `./scripts/setup-products.sh` again
- Verify `.env` has correct `STRIPE_PRICE_*` values
- Check Stripe Dashboard for product existence

### User Sync Not Working

**Error:** Users not appearing in local DB after login

**Solution:**
- Check database connection: `DB_URL`
- Verify migrations ran: `001_initial_schema.sql`
- Check Auth MS returns `user_id`, `name`, `email`

---

## Configuration Reference

### All Environment Variables

| Variable | Description | Example |
|---|---|---|
| `AUTH_SERVICE_URL` | Auth microservice endpoint | `http://localhost:8080` |
| `REDIRECT_URL` | OAuth callback URL | `http://localhost:3000` |
| `DB_URL` | PostgreSQL connection string | `postgresql://...` |
| `PAYMENT_MS_URL` | Payment microservice endpoint | `http://localhost:9000` |
| `PAYMENT_MS_API_KEY` | API key for Payment MS | `your-api-key` |
| `STRIPE_PRODUCT_PRO` | Stripe product ID | `prod_ABC123` |
| `STRIPE_PRICE_MONTHLY` | Monthly price ID | `price_XYZ789` |
| `STRIPE_PRICE_YEARLY` | Yearly price ID | `price_DEF456` |
| `PORT` | Server port | `3000` |
| `SESSION_SECRET` | Session encryption key | Random string |

---

## Next Steps

- Read [Architecture Overview](./ARCHITECTURE.md)
- See [Feature Development Guide](./FEATURES.md)
- Review [Deployment Best Practices](./DEPLOYMENT.md)
