# Pricing Multiplier Calculator

## Formula

```
Monthly Price = Yearly Price ÷ Multiplier
Yearly Discount % = (1 - Multiplier/12) × 100
```

## Quick Reference Table

| Multiplier | Discount | Months Free | Use Case |
|------------|----------|-------------|----------|
| **6×** | 50% | 6 months | Extreme annual funnel (Grammarly-style) |
| **7×** | 42% | 5 months | Very aggressive |
| **8×** | 33% | 4 months | **Recommended** - Strong but believable |
| **9×** | 25% | 3 months | Moderate |
| **10×** | 20% | 2.4 months | Conservative |
| **11×** | 9% | 1 month | Minimal incentive |
| **12×** | 0% | 0 months | No discount (rare) |

## Examples by Yearly Price

### If Yearly = $90 (Starter tier)
```
6×:  $15/mo (extreme)
8×:  $11.25/mo (recommended)
10×: $9/mo (conservative)
```

### If Yearly = $290 (Pro tier)
```
6×:  $48/mo (extreme)
8×:  $36.25/mo (recommended)
10×: $29/mo (conservative)
```

### If Yearly = $990 (Business tier)
```
6×:  $165/mo (extreme)
8×:  $123.75/mo (recommended)
10×: $99/mo (conservative)
```

## Pricing Psychology

### **Start with Yearly Price** (Recommended Approach)
1. Decide what yearly revenue you want per customer
2. Choose your multiplier (8× is sweet spot)
3. Calculate monthly = yearly ÷ multiplier
4. Round to clean numbers ($36.25 → $37 or $35)

Example:
```
Target: $290/year per Pro customer
Multiplier: 8×
Monthly: $290 ÷ 8 = $36.25 → round to $37
Discount: 33% off yearly
```

### **Display Strategy**

On pricing page, show it like this:
```
Pro Plan
━━━━━━━━━━━━━━━━━━━━━━
Monthly:  $37/month
Yearly:   $24/month      ← Show monthly equivalent!
          (billed $290/year)
          💰 Save $154/year (33% off)
```

## Real-World Benchmarks

- **Notion**: 20% yearly discount (10× multiplier)
- **Grammarly**: 60% yearly discount (5× multiplier) 
- **Figma**: 25% yearly discount (9× multiplier)
- **Linear**: 33% yearly discount (8× multiplier) ✅
- **Vercel**: 15% yearly discount (10.6× multiplier)

## Our Recommendation

**Use 8× multiplier (33% discount)**

Why:
- ✅ Proven by successful SaaS (Linear, Superhuman)
- ✅ Strong enough to convert (4 months free!)
- ✅ Not so aggressive it looks like a scam
- ✅ Monthly still viable for cautious customers
- ✅ Creates urgency without desperation

## Implementation

In `setup-config.json`:
```json
{
  "_pricing_strategy": "8× multiplier: monthly = yearly ÷ 8",
  "plans": [{
    "name": "Pro",
    "pricing": {
      "monthly": 3625,   // $290 ÷ 8
      "yearly": 29000    // Anchor price
    }
  }]
}
```

**Note:** Stripe prices are in cents, so $36.25 = 3625 cents
