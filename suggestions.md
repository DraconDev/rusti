# 🚀 Azumi Feature Suggestions

> Refined based on codebase analysis and review

## ✅ Already Working (Verified)

| Feature              | Status   | Example                                         |
| -------------------- | -------- | ----------------------------------------------- |
| `@keyframes`         | ✅ Works | `layout.rs` has `fadeInUp` animation            |
| `@media` queries     | ✅ Works | `layout.rs` uses `@media (max-width: "1200px")` |
| `@container` queries | ✅ Works | `layout.rs` has `@container (width > 700px)`    |
| Form `bind`          | ✅ Works | Compile-time form binding exists                |
| CSS error messages   | ✅ Works | `style.rs` validates with `lightningcss`        |

---

## 🎯 Remaining Suggestions

### 1. `@loading` / `@error` States

**Impact: 4** | **Effort: 3** | **Worth considering**

For async operations where optimistic updates don't apply.

```rust
@async(state.fetch_users()) {
    @loading { <Spinner /> }
    @error(e) { <p>"Error: " {e}</p> }
    |users| { @for user in users { ... } }
}
```

**Verdict:** Complements Azumi Live for cases like database queries, external APIs.

---

### 2. Client-Side `set` Implementation

**Impact: 3** | **Effort: 1** | **Easy win**

Stub exists in `azumi.js:377`. For purely client-side UI state:

-   Accordion open/close
-   Tab switching
-   Modal visibility

**Verdict:** Would work well with Azumi Live—client for ephemeral UI, server for real state.

---

### 3. Image Optimization (Future)

**Impact: 3** | **Effort: 4**

Auto-generate srcset, WebP, lazy loading. Nice to have later.

---

## ❌ Removed from List

| Feature             | Reason                                 |
| ------------------- | -------------------------------------- |
| Named Slots         | Simpler to just use `<h1>` in header   |
| Auto-routing        | Explicit routing allows deeper logic   |
| Islands             | Rare use case, adds complexity         |
| Toasts              | Can ship JS for rare cases             |
| Form Validation DSL | Already have `bind`                    |
| Better CSS errors   | Already have `lightningcss` validation |

---

## 📊 Final Priority

| Feature            | Priority | Action                            |
| ------------------ | -------- | --------------------------------- |
| Client `set`       | ⭐⭐⭐   | Implement soon (1 hour)           |
| `@loading/@error`  | ⭐⭐     | Design carefully, fits philosophy |
| Image optimization | ⭐       | Future consideration              |
