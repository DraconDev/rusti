# Payment Microservice - Product Registration Endpoint

This document provides implementation guidance for adding the `/admin/products/register` endpoint to your Payment Microservice.

## Overview

The product registration endpoint allows BFF applications to programmatically create Stripe products and prices without storing Stripe credentials.

**Benefits:**
- 🔒 **Centralized Stripe management** - only Payment MS has Stripe keys
- 🚀 **Automated setup** - new deployments self-configure
- 📊 **Clean revenue tracking** - separate products per project
- 🎯 **Consistent naming** - standardized product structure

---

## API Specification

### Endpoint

```
POST /admin/products/register
```

### Authentication

```
X-API-Key: your-payment-ms-api-key
```

**For single-owner deployments:** Use the same API key as regular operations.

**For multi-tenant/production:** Consider using a separate admin API key with elevated permissions and stricter rate limiting.

### Request Body

```json
{
  "project_name": "my-saas-app",
  "plans": [
    {
      "name": "Pro Plan",
      "description": "Professional features for power users",
      "features": [
        "unlimited_projects",
        "advanced_analytics",
        "priority_support",
        "api_access"
      ],
      "pricing": {
        "monthly": 2900,
        "yearly": 29000
      }
    }
  ]
}
```

**Field Descriptions:**
- `project_name` (string, required): Unique identifier for the project
- `plans` (array, required): List of subscription plans to create
  - `name` (string, required): Display name (e.g., "Pro Plan")
  - `description` (string, optional): Plan description
  - `features` (array, optional): List of feature identifiers
  - `pricing` (object, required): Price points
    - `monthly` (int, required): Monthly price in cents (e.g., 2900 = $29.00)
    - `yearly` (int, optional): Yearly price in cents

### Response

**Success (201 Created):**
```json
{
  "success": true,
  "project_id": "my-saas-app",
  "products": [
    {
      "plan_name": "Pro Plan",
      "stripe_product_id": "prod_ABC123XYZ",
      "prices": {
        "monthly": {
          "stripe_price_id": "price_monthly_XYZ789",
          "amount": 2900,
          "interval": "month",
          "currency": "usd"
        },
        "yearly": {
          "stripe_price_id": "price_yearly_DEF456",
          "amount": 29000,
          "interval": "year",
          "currency": "usd"
        }
      },
      "created_at": "2025-11-22T06:00:00Z"
    }
  ]
}
```

**Error Responses:**

```json
// 400 Bad Request - Invalid input
{
  "success": false,
  "error": "validation_error",
  "message": "project_name is required"
}

// 401 Unauthorized - Invalid API key
{
  "success": false,
  "error": "unauthorized",
  "message": "Invalid admin API key"
}

// 409 Conflict - Product already exists
{
  "success": false,
  "error": "already_exists",
  "message": "Product for project 'my-saas-app' already exists",
  "existing_product_id": "prod_EXISTING123"
}

// 500 Internal Server Error - Stripe API failure
{
  "success": false,
  "error": "stripe_error",
  "message": "Failed to create product in Stripe",
  "details": "Stripe error message"
}
```

---

## Implementation Guide

### 1. Handler Structure (Go Example)

```go
package handlers

import (
    "encoding/json"
    "net/http"
    "github.com/stripe/stripe-go/v76"
    "github.com/stripe/stripe-go/v76/product"
    "github.com/stripe/stripe-go/v76/price"
)

type ProductRegistrationRequest struct {
    ProjectName string `json:"project_name"`
    Plans       []Plan `json:"plans"`
}

type Plan struct {
    Name        string            `json:"name"`
    Description string            `json:"description,omitempty"`
    Features    []string          `json:"features,omitempty"`
    Pricing     Pricing           `json:"pricing"`
}

type Pricing struct {
    Monthly int64 `json:"monthly"` // in cents
    Yearly  int64 `json:"yearly,omitempty"`
}

func (h *AdminHandler) RegisterProducts(w http.ResponseWriter, r *http.Request) {
    // 1. Validate API key (same key used for regular operations)
    apiKey := r.Header.Get("X-API-Key")
    if !h.validateAPIKey(apiKey) {
        respondError(w, 401, "unauthorized", "Invalid API key")
        return
    }

    // 2. Parse request
    var req ProductRegistrationRequest
    if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
        respondError(w, 400, "invalid_json", err.Error())
        return
    }

    // 3. Validate request
    if err := validateRequest(&req); err != nil {
        respondError(w, 400, "validation_error", err.Error())
        return
    }

    // 4. Check if product already exists
    if exists, productID := h.productExists(req.ProjectName); exists {
        respondError(w, 409, "already_exists", 
            fmt.Sprintf("Product for project '%s' already exists", req.ProjectName),
            map[string]string{"existing_product_id": productID})
        return
    }

    // 5. Create products in Stripe
    products, err := h.createStripeProducts(req)
    if err != nil {
        respondError(w, 500, "stripe_error", "Failed to create products", 
            map[string]string{"details": err.Error()})
        return
    }

    // 6. Store in database
    if err := h.storeProducts(req.ProjectName, products); err != nil {
        // Rollback Stripe products if DB save fails
        h.rollbackStripeProducts(products)
        respondError(w, 500, "database_error", err.Error())
        return
    }

    // 7. Return success
    respondSuccess(w, 201, products)
}
```

### 2. Stripe Product Creation

```go
func (h *AdminHandler) createStripeProducts(req ProductRegistrationRequest) ([]ProductResponse, error) {
    var results []ProductResponse

    for _, plan := range req.Plans {
        // Create Stripe Product
        productParams := &stripe.ProductParams{
            Name:        stripe.String(fmt.Sprintf("%s - %s", req.ProjectName, plan.Name)),
            Description: stripe.String(plan.Description),
            Metadata: map[string]string{
                "project_name": req.ProjectName,
                "plan_name":    plan.Name,
                "features":     strings.Join(plan.Features, ","),
            },
        }

        stripeProduct, err := product.New(productParams)
        if err != nil {
            return nil, fmt.Errorf("failed to create product: %w", err)
        }

        // Create prices
        prices := PriceResponse{}
        
        // Monthly price
        if plan.Pricing.Monthly > 0 {
            monthlyPrice, err := price.New(&stripe.PriceParams{
                Product:    stripe.String(stripeProduct.ID),
                UnitAmount: stripe.Int64(plan.Pricing.Monthly),
                Currency:   stripe.String("usd"),
                Recurring: &stripe.PriceRecurringParams{
                    Interval: stripe.String("month"),
                },
                Metadata: map[string]string{
                    "project_name": req.ProjectName,
                    "interval":     "monthly",
                },
            })
            if err != nil {
                return nil, fmt.Errorf("failed to create monthly price: %w", err)
            }
            prices.Monthly = &PriceDetails{
                StripePriceID: monthlyPrice.ID,
                Amount:        monthlyPrice.UnitAmount,
                Interval:      "month",
                Currency:      "usd",
            }
        }

        // Yearly price (if specified)
        if plan.Pricing.Yearly > 0 {
            yearlyPrice, err := price.New(&stripe.PriceParams{
                Product:    stripe.String(stripeProduct.ID),
                UnitAmount: stripe.Int64(plan.Pricing.Yearly),
                Currency:   stripe.String("usd"),
                Recurring: &stripe.PriceRecurringParams{
                    Interval: stripe.String("year"),
                },
                Metadata: map[string]string{
                    "project_name": req.ProjectName,
                    "interval":     "yearly",
                },
            })
            if err != nil {
                return nil, fmt.Errorf("failed to create yearly price: %w", err)
            }
            prices.Yearly = &PriceDetails{
                StripePriceID: yearlyPrice.ID,
                Amount:        yearlyPrice.UnitAmount,
                Interval:      "year",
                Currency:      "usd",
            }
        }

        results = append(results, ProductResponse{
            PlanName:         plan.Name,
            StripeProductID:  stripeProduct.ID,
            Prices:           prices,
            CreatedAt:        time.Now(),
        })
    }

    return results, nil
}
```

### 3. Database Storage

```sql
CREATE TABLE IF NOT EXISTS registered_products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_name VARCHAR(255) NOT NULL,
    plan_name VARCHAR(255) NOT NULL,
    stripe_product_id VARCHAR(255) NOT NULL UNIQUE,
    stripe_price_monthly VARCHAR(255),
    stripe_price_yearly VARCHAR(255),
    features JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(project_name, plan_name)
);

CREATE INDEX idx_registered_products_project ON registered_products(project_name);
```

```go
func (h *AdminHandler) storeProducts(projectName string, products []ProductResponse) error {
    tx, err := h.db.Begin()
    if err != nil {
        return err
    }
    defer tx.Rollback()

    for _, product := range products {
        featuresJSON, _ := json.Marshal(product.Features)
        
        _, err = tx.Exec(`
            INSERT INTO registered_products 
            (project_name, plan_name, stripe_product_id, stripe_price_monthly, stripe_price_yearly, features)
            VALUES ($1, $2, $3, $4, $5, $6)
        `, 
            projectName,
            product.PlanName,
            product.StripeProductID,
            product.Prices.Monthly.StripePriceID,
            product.Prices.Yearly.StripePriceID,
            featuresJSON,
        )
        if err != nil {
            return err
        }
    }

    return tx.Commit()
}
```

### 4. Security Considerations

**API Key Validation:**
```go
// For single-owner scenarios: use same key as regular operations
func (h *AdminHandler) validateAPIKey(providedKey string) bool {
    return subtle.ConstantTimeCompare(
        []byte(providedKey),
        []byte(h.config.APIKey),
    ) == 1
}
```

**Optional: Separate Admin Key (for production):**
```go
// Store admin API key separately from regular keys
// Use environment variable or secure secret manager
PAYMENT_MS_API_KEY=your-regular-api-key
PAYMENT_MS_ADMIN_API_KEY=different-key-for-admin-ops  // Optional

// Validate with constant-time comparison
func (h *AdminHandler) validateAdminKey(providedKey string) bool {
    adminKey := h.config.AdminAPIKey
    if adminKey == "" {
        adminKey = h.config.APIKey  // Fall back to regular key
    }
    return subtle.ConstantTimeCompare(
        []byte(providedKey),
        []byte(adminKey),
    ) == 1
}
```

**Rate Limiting:**
```go
// Add rate limiting to prevent abuse
// Allow max 10 product registrations per hour per IP
rateLimiter := tollbooth.NewLimiter(10, &limiter.ExpirableOptions{
    DefaultExpirationTTL: time.Hour,
})
```

---

## Testing

### cURL Example

```bash
curl -X POST http://localhost:9000/admin/products/register \
  -H "X-API-Key: your-admin-api-key" \
  -H "Content-Type: application/json" \
  -d '{
    "project_name": "test-saas",
    "plans": [{
      "name": "Pro Plan",
      "description": "For professionals",
      "features": ["unlimited_projects", "api_access"],
      "pricing": {
        "monthly": 2900,
        "yearly": 29000
      }
    }]
  }'
```

### Expected Response

```json
{
  "success": true,
  "project_id": "test-saas",
  "products": [{
    "plan_name": "Pro Plan",
    "stripe_product_id": "prod_TEST123",
    "prices": {
      "monthly": {
        "stripe_price_id": "price_TEST456",
        "amount": 2900,
        "interval": "month",
        "currency": "usd"
      },
      "yearly": {
        "stripe_price_id": "price_TEST789",
        "amount": 29000,
        "interval": "year",
        "currency": "usd"
      }
    }
  }]
}
```

---

## Rollback Strategy

If product creation fails midway:

```go
func (h *AdminHandler) rollbackStripeProducts(products []ProductResponse) {
    for _, product := range products {
        // Archive (don't delete) to preserve audit trail
        product.Archive(&stripe.ProductParams{
            Active: stripe.Bool(false),
        })
    }
}
```

---

## Common Issues

**Issue:** Product created in Stripe but not in database

**Solution:** Implement idempotency - check Stripe first:
```go
func (h *AdminHandler) productExists(projectName string) (bool, string) {
    // Check database first
    var productID string
    err := h.db.QueryRow(`
        SELECT stripe_product_id FROM registered_products 
        WHERE project_name = $1 LIMIT 1
    `, projectName).Scan(&productID)
    
    if err == nil {
        return true, productID
    }
    return false, ""
}
```

---

## Additional Resources

- [Stripe Products API Documentation](https://stripe.com/docs/api/products)
- [Stripe Prices API Documentation](https://stripe.com/docs/api/prices)
- [Stripe Testing Best Practices](https://stripe.com/docs/testing)
