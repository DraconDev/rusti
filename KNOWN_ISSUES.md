# Known Issues

## Rust-analyzer Macro Expansion Error

**Issue**: rust-analyzer shows `"Unexpected input remaining: < html > ..."` error in the IDE even though the code compiles and runs successfully.

**Root Cause**: rust-analyzer may stringify `TokenStream` differently than `rustc`. The error message shows spaces around `<` and `>` (e.g., `< html >`) but the actual compiler produces `<html>` without spaces. This is a rust-analyzer display/expansion quirk.

**Why this happens**: 
- The actual `TokenStream::to_string()` produces: `<html> <head>...` (no spaces around brackets)
- rust-analyzer may show: `< html > < head >...` (with spaces) in error messages
- Our parser works correctly with the actual format but rust-analyzer shows errors based on its own stringification

**Impact**: Cosmetic only - does not affect compilation or runtime behavior.

**Verification**: 
- ✅ `cargo build` succeeds
- ✅ `cargo test` passes with actual TokenStream format
- ✅ `cargo run` works correctly
- ✅ Web server serves HTML properly
- ✅ XSS protection works

**Workarounds**:
1. **Ignore the error** - the code works fine (recommended)
2. Try restarting rust-analyzer: `Ctrl+Shift+P` → "Rust: Restart server"
3. Verify with `cargo build` rather than relying on IDE errors

**Future**: This may be resolved as rust-analyzer improves its proc macro support and TokenStream stringification.
