# 🚀 Azumi Feature Suggestions

> Refined recommendations based on codebase analysis and review

## ✅ Already Working (Verified)

| Feature              | Status   | Example                                         |
| -------------------- | -------- | ----------------------------------------------- |
| `@keyframes`         | ✅ Works | `layout.rs` has `fadeInUp` animation            |
| `@media` queries     | ✅ Works | `layout.rs` uses `@media (max-width: "1200px")` |
| `@container` queries | ✅ Works | `layout.rs` has `@container (width > 700px)`    |
| Form `bind`          | ✅ Works | Compile-time form binding exists                |
| CSS error messages   | ✅ Works | `style.rs` has validation with `lightningcss`   |

---

## 🎯 Remaining Suggestions

### 1. `@loading` / `@error` States

**Impact: 4** | **Effort: 3** | **Worth considering**

For async operations where optimistic updates don't apply (database queries, external APIs).

````rust
@async(state.fetch_users()) {
    @loading { <Spinner /> }
    @error(e) { <p>"Error: " {e}</p> }

**Impact: 4** | **Effort: 4** | **Priority: 1.0**

Auto-generate responsive images, lazy loading, blur placeholders.

```rust
@Image(src="/photo.jpg", alt="Photo", width=400)
// Generates: srcset, WebP variant, lazy loading
````

**Why:** Major DX improvement but requires build-time tooling.

---

### 6. Named Component Slots

**Impact: 3** | **Effort: 3** | **Priority: 1.0**

Currently only `children: impl Component`. Named slots would help:

```rust
@Card {
    @slot(header) { <h1>"Title"</h1> }
    <p>"Content"</p>
}
```

**Why:** Useful for complex layouts like modals, dialogs.

---

### 7. Form Validation DSL

**Impact: 4** | **Effort: 3** | **Priority: 1.3**

Server-side validation with error message rendering.

```rust
#[azumi::form_action]
async fn login(form: LoginForm) -> Result<Redirect, ValidationErrors> {
    validate!(form.email, email);
    validate!(form.password, min_length: 8);
}
```

**Why:** Forms are everywhere. Built-in validation would be huge.

---

### 8. Better Error Messages in CSS Validation

**Impact: 3** | **Effort: 2** | **Priority: 1.5**

`style.rs` validates CSS but could suggest fixes:

```
Error: Unknown CSS property 'colro'
       Did you mean 'color'?
```

**Why:** Improves DX. Levenshtein distance for suggestions.

---

## 🎨 Lower Priority (Nice to Have)

### 9. Partial Hydration / Islands

**Impact: 4** | **Effort: 5** | **Priority: 0.8**

Only hydrate interactive components, not the whole page.

```rust
#[azumi::island]
fn InteractiveWidget() { ... }  // Only this ships JS
```

**Why:** Performance optimization. Complex architecture change.

---

### 10. Auto-Routing from File System

**Impact: 3** | **Effort: 4** | **Priority: 0.75**

Scan `pages/` directory like Next.js.

```rust
.merge(azumi::auto_routes!("src/pages"))
```

**Why:** Convention over configuration. Nice but not critical.

---

### 11. Toast/Notification System

**Impact: 2** | **Effort: 2** | **Priority: 1.0**

Built-in toast notifications for action feedback.

```rust
state.add_item();
azumi::toast::success("Item added!");
```

**Why:** Common pattern but apps can implement their own.

---

### 12. CSS Container Queries

**Impact: 2** | **Effort: 2** | **Priority: 1.0**

Modern CSS feature for component-based responsive design.

```rust
<style>
    @container (min-width: "400px") {
        .card { flex-direction: "row"; }
    }
</style>
```

**Why:** Cutting-edge CSS. Lower priority than media queries.

Thoughts

1. we already have animation, layout is example
2. media works too
3. but we are very different so this is when we can't do optimistic update? i could see it, worth considering
4. we are trying to limit complication that is why we have azumli live like lesson 9, but would it work with us?
5. seems like an easy win
6. how? Cna't just the header heave a title h1?, this seems less clean
7. what about the bind we have ? for forms
8. we already have this
9. Not sure if we want the complexity, currently we cna use the live struct, but this is useless in very rare cases, albeit might be more confusion than worth
10. Is this better? I think non file system routing allows for deeper logic, you can alreayd maek subfolders and router based on them, you are not forced to
11. i suppose otherwise we would need js, this is pretty low prio, i am not super scared of shipping some js in rare cases,
12. nice if we dont already have it, at least its not erroring we would have to check the macro

---

## 📊 Summary Table

| Feature            | Impact | Effort | Priority | Recommendation |
| ------------------ | ------ | ------ | -------- | -------------- |
| `@media` queries   | 5      | 2      | **2.5**  | ✅ Do now      |
| `@keyframes`       | 4      | 2      | **2.0**  | ✅ Do now      |
| Client `set`       | 3      | 1      | **3.0**  | ✅ Quick win   |
| `@loading/@error`  | 5      | 3      | **1.7**  | ✅ High value  |
| Better errors      | 3      | 2      | **1.5**  | 👍 Worth it    |
| Form validation    | 4      | 3      | **1.3**  | 👍 Worth it    |
| Image optimization | 4      | 4      | **1.0**  | 🤔 Consider    |
| Named slots        | 3      | 3      | **1.0**  | 🤔 Consider    |
| Toasts             | 2      | 2      | **1.0**  | 🤷 Optional    |
| Container queries  | 2      | 2      | **1.0**  | 🤷 Optional    |
| Islands            | 4      | 5      | **0.8**  | ⏳ Later       |
| Auto-routing       | 3      | 4      | **0.75** | ⏳ Later       |

---

## 🎯 Recommended Roadmap

1. **Phase 1 (Quick Wins):**

    - Client `set` implementation
    - Verify `@media` and `@keyframes` work
    - Better CSS error messages

2. **Phase 2 (Core Features):**

    - `@loading/@error` patterns
    - Form validation DSL

3. **Phase 3 (Polish):**

    - Image optimization
    - Named slots
    - Toasts

4. **Phase 4 (Advanced):**
    - Islands architecture
    - Auto-routing
