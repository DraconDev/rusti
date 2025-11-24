# SaaS Pricing Strategy Guide

## Typical SaaS Subscription Tiers

Most successful SaaS products use **3-4 pricing tiers**:

### **Freemium or Starter** ($0-10/month)
- **Purpose**: User acquisition, conversion funnel entry
- **Limits**: Basic features, usage caps
- **Example**: 3 projects, 5GB storage, community support

### **Pro/Professional** ($20-50/month)
- **Purpose**: Individual power users, freelancers
- **Sweet spot**: Most profitable tier for many SaaS
- **Example**: Unlimited projects, API access, priority support

### **Business/Team** ($50-100/month)
- **Purpose**: Small-medium teams, businesses
- **Features**: Collaboration, SSO, advanced security
- **Example**: Team features, SAML, SLA

### **Enterprise** ($100+/month or custom)
- **Purpose**: Large organizations
- **Pricing**: Usually custom/contact sales
- **Features**: Dedicated support, custom contracts

---

## Lifetime Deals vs Subscriptions

### **Subscriptions** (Recurring Revenue - Recommended)
```json
{
  "pricing": {
    "monthly": 2900,   // $29/mo
    "yearly": 29000    // $290/yr (17% discount = 2 months free)
  }
}
```

**Benefits:**
- ✅ Predictable recurring revenue (MRR/ARR)
- ✅ Better customer LTV
- ✅ Sustainable business model

### **Lifetime Deals** (One-time Payment)
```json
{
  "pricing": {
    "one_time": 49900  // $499 one-time
  }
}
```

**When to use:**
- 🚀 Product launches (AppSumo, Product Hunt)
- 💰 Cash injection for early-stage
- 📢 Marketing/buzz generation

**Risks:**
- ⚠️ No recurring revenue
- ⚠️ Support costs forever
- ⚠️ Devalues subscription tiers

---

## Pricing Examples from Real SaaS

### **GitHub**
- Free: Public repos
- Pro: $4/mo - Private repos
- Team: $4/user/mo
- Enterprise: Custom

### **Notion**
- Free: Individual use
- Plus: $8/mo - Small teams
- Business: $15/mo - Companies
- Enterprise: Custom

### **Vercel**
- Hobby: $0
- Pro: $20/mo
- Team: $20/user/mo (min 2)
- Enterprise: Custom

---

## Recommended Starter Configuration

For a **new SaaS product**, I recommend:

### **3-Tier Model** (setup-config.example.json shows this)
```
Starter:    $9/mo  ($90/yr)  - Basic features
Pro:        $29/mo ($290/yr) - Full features  ← Primary target
Business:   $99/mo ($990/yr) - Team features
```

**Why this works:**
- ✅ **Starter** converts free trial users
- ✅ **Pro** is your revenue driver (70% of customers)
- ✅ **Business** captures high-value accounts
- ✅ Yearly pricing ~17% discount (industry standard)

### **When to Add 4th Tier**
Add "Free" or "Enterprise" when:
- **Free tier**: You need viral growth/user acquisition
- **Enterprise**: You have 5+ $1k+/mo customers asking for custom terms

---

## Lifetime Deal Strategy

**If you do offer lifetime deals**, structure it carefully:

```json
{
  "name": "Lifetime Pro",
  "description": "One-time payment, lifetime access to Pro features",
  "features": ["all_pro_features", "lifetime_updates"],
  "pricing": {
    "one_time": 49900  // ~17 months of Pro pricing
  }
}
```

**Best practices:**
1. **Limited availability** - Create urgency (100 spots)
2. **Higher than 12mo subscription** - Value your MRR
3. **Grandfather clause** - They keep current features, not future tiers
4. **Separate from main pricing** - Don't show on pricing page

---

## Your Config File

The **`setup-config.example.json`** now shows a realistic 3-tier structure:

- **Starter**: $9/mo - Entry point
- **Pro**: $29/mo - Sweet spot (your main revenue)
- **Business**: $99/mo - Enterprise-lite

Customize the:
- `project_name` to your SaaS name
- `features` array to your actual feature gates  
- `pricing` amounts to your market positioning

Then run `./scripts/setup-products.sh` to create them in Stripe!
