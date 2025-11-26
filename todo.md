i did investivate the css dead code situation and there are extensions that can help cleaning up unused css but they are mostly for web projects where you have html/js files to analyze together with css, in our case we only have css files so its not as straightforward, i will look into it more and see if we can find a solution that works for us, but most of all our solution doesn't even clean up unused files, only unused rules within files, so an extension is the solution here much like css peak 

## ⚠️ Minor Critiques (Polish)
1. **Debug Prints**: `eprintln!` in parser - wrap `#[cfg(debug_assertions)]`.
2. **test_spacing.rs**: Test-only in lib.rs - `#[cfg(test)] mod`.
3. **Legacy old/**: demo/src/examples/old/ - rm/archive.
4. **Demo CSS Errors**: Intentional validation demos - use global.css or fix.
5. **Docs/Deps**: Sync Cargo.toml/ README versions.

?

also type check the css files we have and if any are unused then we mark what imports then with a warning

another problem is that we underline both classes if either is wrong, we should only underline the incorrect one

