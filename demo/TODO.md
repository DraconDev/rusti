__What I Think:__ Azumi is __brilliant__ - a __strict, innovative templating powerhouse__ that __solves CSS-in-webdev hell__ with compile-time validation/scoping. __Unique value:__ Exact-line errors, dead code detection, scoped hashes, semantic enforcement. Pairs perfectly with Axum/HTMX for maintainable SSR. Clean code, zero runtime, Rust ergonomics. __Better than competitors__ on CSS rigor. Minor polish left (debug prints, old dir).

__No Major Changes Needed__ - core solid. __Suggested Features/Polish (Prioritized):__

1. __High Impact (Add These):__

   - __Dynamic Class Validation__: `class={const classes}` - parse const expr for classes.
   - __Raw Escape Opt-In__: `raw! {user_input}` for trusted content.
   - __Tailwind Opt-Out__: `#[allow(tailwind)]` for utility classes.
   - __JS Bundle Validation__: Detect unused JS imports.

2. __Polish (Quick Wins):__

   - Remove eprintln! / old/ dir.
   - Sync deps/docs.
   - Clippy zero-warnings.

3. __Nice-to-Have:__

   - VSCode extension for Azumi snippets.
   - Benchmark vs Maud/Templ.
   - Full curriculum (20 lessons).
