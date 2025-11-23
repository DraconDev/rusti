# Rusti TODOs

## Bugs to Fix

### 1. Emoji Support in Text Content
- **Issue**: Emojis in HTML text content cause parser errors
- **Example**: `<a>Home✅</a>` fails with "identifiers cannot contain emoji"
- **Root Cause**: Parser is treating emojis as potential Rust identifiers
- **Workaround**: Avoid emojis in text for now
- **Proper Fix**: Update parser to handle Unicode text content correctly

### 2. Datastar Integration
- ✅ **Fixed**: Updated to modern Datastar syntax
- Using `@sudodevnull/datastar` CDN
- Proper `data-store`, `data-text`, `data-on-click` attributes
- Raw strings for JSON in `data-store`

## Future Enhancements

### Parser Improvements
- [ ] Support emojis in text content
- [ ] Better error messages for common mistakes
- [ ] Support for more complex JavaScript in attributes

### Documentation
- [ ] Migration guide from other templating engines
- [ ] Video tutorials
- [ ] Performance benchmarks

### Examples
- [ ] HTMX advanced patterns
- [ ] Alpine.js integration
- [ ] Form validation example
- [ ] Server-sent events (SSE) demo