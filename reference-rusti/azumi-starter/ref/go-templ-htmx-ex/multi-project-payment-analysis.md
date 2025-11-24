# Multi-Project Payment System Analysis

**Date:** November 21, 2025  
**Context:** Designing payment system for potential multi-project/multi-tenant usage  
**Question:** How to handle product IDs, user subscriptions, and billing across multiple projects?

---

## 🏗️ **CURRENT SINGLE-PROJECT DESIGN**

### **Current Implementation**
```go
// Project-specific product IDs
const products = {
    premium: {
        productId: 'prod_premium_123',      // Unique to this project
        priceId: 'price_premium_monthly_123', // Stripe price for this project
    }
};

// User context per project
userInfo = {
    email: "user@example.com",
    // No project context
}
```

### **Current Limitations**
- ✅ Works perfectly for single project
- ❌ No multi-project user tracking
- ❌ No billing separation between projects
- ❌ Product IDs are project-specific but hardcoded

---

## 🎯 **MULTI-PROJECT SCENARIOS**

### **Scenario 1: User Subscribes to Two Projects**
**User Journey:**
1. User subscribes to Project A (Premium Plan)
2. Same user subscribes to Project B (Premium Plan)
3. Question: How do we track/access separate subscriptions?

### **Scenario 2: Reusable Payment System**
**Architecture Goal:**
1. Single payment microservice handles multiple frontend apps
2. One user can have subscriptions to different projects
3. Billing should be project-separated
4. Products might be shared or project-specific

---

## 📋 **MULTI-PROJECT PAYMENT ARCHITECTURES**

### **Option 1: Project-Specific Product IDs**
**Implementation:** Each project gets its own Stripe products/prices

```go
// Project A (Startup Platform)
const projectAProducts = {
    premium: {
        projectId: 'project_a',
        productId: 'prod_a_premium_456',
        priceId: 'price_a_premium_monthly_456',
    }
};

// Project B (Different Product)
const projectBProducts = {
    premium: {
        projectId: 'project_b', 
        productId: 'prod_b_premium_789',
        priceId: 'price_b_premium_monthly_789',
    }
};
```

**Pros:**
- ✅ Complete isolation between projects
- ✅ Each project can have different pricing
- ✅ No conflicts if same user subscribes to multiple
- ✅ Easy to migrate users between projects
- ✅ Stripe dashboard shows separate revenue per project

**Cons:**
- ❌ Need to manage separate Stripe products per project
- ❌ More complex configuration management
- ❌ Product IDs become very long: `proj_a_basic_monthly_v2`
- ❌ Need to sync product changes across projects

---

### **Option 2: Shared Product Pool + Project Context**
**Implementation:** Same products across projects, track project context in user data

```go
// Shared products across all projects
const sharedProducts = {
    premium: {
        sharedProductId: 'prod_shared_premium',
        sharedPriceId: 'price_shared_premium_monthly',
    }
};

// Enhanced user context with project tracking
userInfo = {
    email: "user@example.com",
    projectId: "startup_platform",
    subscriptions: [
        {projectId: "startup_platform", plan: "premium", status: "active"},
        {projectId: "analytics_dashboard", plan: "basic", status: "active"}
    ]
};
```

**Pros:**
- ✅ Simple product management (one set of products)
- ✅ User can track all their subscriptions in one place
- ✅ Shared Stripe products reduce complexity
- ✅ Easy to add new projects using same products

**Cons:**
- ❌ Pricing must be the same across all projects
- ❌ Hard to have project-specific features
- ❌ Revenue reporting becomes complex (can't separate by project)
- ❌ User might not understand billing across projects

---

### **Option 3: Tenant-Based Payment Microservice**
**Implementation:** Payment microservice handles project isolation, user tracking

```go
// Payment microservice handles multi-tenancy
POST /api/v1/checkout/subscription
{
    "user_id": "user@example.com",
    "project_id": "startup_platform",        // New field
    "product_id": "premium",
    "tenant_id": "company_abc"               // For billing separation
}

// User subscription tracking
GET /api/v1/subscriptions/user@example.com
{
    "user_id": "user@example.com",
    "subscriptions": [
        {
            "project_id": "startup_platform",
            "plan": "premium",
            "status": "active",
            "tenant_id": "company_abc"
        }
    ]
}
```

**Pros:**
- ✅ Complete multi-tenant isolation
- ✅ Payment microservice handles complexity
- ✅ Each project can have different products/pricing
- ✅ User can track all their projects
- ✅ Billing separated by tenant/project

**Cons:**
- ❌ Requires extending payment microservice
- ❌ More complex API design
- ❌ Need to manage tenant/project mapping
- ❌ Requires coordination between teams

---

### **Option 4: Hybrid Approach**
**Implementation:** Use shared products for simple plans, project-specific for complex ones

```go
// Simple plans: Shared across all projects
const simplePlans = {
    basic: { sharedProductId: "prod_basic_shared", price: "$9/month" },
    premium: { sharedProductId: "prod_premium_shared", price: "$29/month" }
};

// Complex plans: Project-specific
const complexPlans = {
    enterprise: { 
        projectSpecific: true,
        products: {
            "startup_platform": { priceId: "price_enterprise_startup" },
            "analytics_dashboard": { priceId: "price_enterprise_analytics" }
        }
    }
};
```

**Pros:**
- ✅ Flexibility for different project needs
- ✅ Shared simple products for consistency
- ✅ Project-specific complex products
- ✅ Best of both worlds

**Cons:**
- ❌ More complex configuration logic
- ❌ Need to decide which products are shared vs specific
- ❌ Product management becomes more complex

---

## 🎯 **RECOMMENDATION**

### **For Your Current Situation: Option 1 (Project-Specific)**

**Why This Makes Sense:**
1. **Simplicity**: Current hardcoded approach works, just add project prefix
2. **Isolation**: Complete separation between projects
3. **Growth Ready**: Easy to add new projects with different pricing
4. **Stripe Best Practice**: Separate products per business unit/project

**Implementation:**
```go
// Move from: prod_premium_123
// To: startup_platform_premium_monthly_v1

const products = {
    premium: {
        projectId: "startup_platform",
        productId: "startup_platform_premium_v1",
        priceId: "startup_platform_premium_monthly_price",
        name: "Premium Plan",
        price: "$29/month",
        features: [...]
    }
};
```

### **Future Multi-Project Evolution**

**Phase 1: Project-Specific (Current)**
- Each project has unique product IDs
- Simple configuration management
- Works immediately

**Phase 2: Multi-Project User Tracking** 
- Add project context to user sessions
- Track user subscriptions across projects
- Payment microservice returns multi-project user data

**Phase 3: Centralized Payment Service**
- One payment microservice for all projects
- Tenant-based billing separation
- Advanced subscription management

---

## 🛠️ **IMMEDIATE ACTION PLAN**

### **This Week: Project-Specific Product IDs**

1. **Rename Current Products** (15 minutes)
   ```go
   // From: prod_premium_123 → startup_platform_premium_v1
   // Add project context to all product IDs
   ```

2. **Update Environment Configuration** (10 minutes)
   ```go
   // Add project identifier to config
   cfg.ProjectID = "startup_platform"
   cfg.ProductPrefix = "startup_platform"
   ```

3. **Future-Proof API Design** (30 minutes)
   ```go
   // Include project context in API requests
   paymentReq := map[string]interface{}{
       "user_id": userInfo.Email,
       "project_id": "startup_platform",  // Add this
       "product_id": "premium",
       // ...
   }
   ```

### **Next Month: Multi-Project User Tracking**

1. **Extended User Context**
   - Add project tracking to user sessions
   - Store current project context
   - Track subscription status per project

2. **Enhanced Payment API**
   - Include project_id in all payment requests
   - Return subscription data with project context
   - Enable cross-project subscription management

---

## 💡 **KEY INSIGHTS**

### **Product ID Strategy**
- **Unique per project**: `project_name_product_type_version`
- **Example**: `startup_platform_premium_v1`, `analytics_dashboard_basic_v2`

### **User Tracking Strategy**  
- **Current**: Single project user context
- **Future**: Multi-project user with active project context
- **Evolution**: User can switch between projects seamlessly

### **Billing Separation**
- **Project-specific**: Each project has separate Stripe products
- **User tracking**: User can subscribe to multiple projects
- **Management**: Each project manages its own billing

### **Scalability Path**
1. **Now**: Single project with unique product IDs
2. **Soon**: Multi-project user context
3. **Later**: Centralized multi-tenant payment service

---

## 🚀 **CONCLUSION**

You're absolutely right to think about multi-project scenarios! The current approach works perfectly for single projects, but planning for multi-project usage is smart.

**Recommendation**: Start with **Option 1 (Project-Specific Product IDs)** for immediate implementation, then evolve to **Option 3 (Tenant-Based)** when you have multiple projects.

This gives you:
- ✅ Immediate solution that works
- ✅ Future-proof architecture  
- ✅ Clear upgrade path
- ✅ Professional multi-tenant billing

The key insight is that **product IDs should be project-specific** but **user tracking should evolve** to handle multiple projects gracefully.