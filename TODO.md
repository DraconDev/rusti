# azumi

Roadmap & TODO

## ✅ Completed

-   [x] `<link rel="stylesheet">` scoping enforced (parser prevents local files, CDN allowed)
-   [x] `@let` block support implemented
-   [x] Component-scoped CSS with `<style src>`
-   [x] PascalCase and snake_case component support

## 🚀 Next Up

### 1. Comprehensive README

one change is that we are hating on tailwind and using a custom css parser, we are not using any css framework, only css and all the css is scoped to the component, and all css must have a match in the css file, and all css must be used in the component

-   [ ] Clear explanation of what Azumi is and is NOT
-   [ ] Design philosophy and why we made our choices

### 2. More Complex Component Examples

-   [ ] Form components with validation
-   [ ] Modal/Dialog components
-   [ ] Nested component composition patterns

### 3. Developer Experience

-   [ ] Add CSS peek extension recommendation to README
-   [ ] Include rust-analyzer workspace settings for CSS files
