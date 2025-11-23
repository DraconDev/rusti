# Rusti Documentation Guide

This guide helps you navigate all the documentation files.

## 📚 Documentation Files

### For Users (Start Here!)

1. **[README.md](README.md)** - Main documentation
   - Installation
   - Syntax rules (quotes, emojis, CSS, JS)
   - 5 ways to style
   - Quick translation guide
   - Best practices
   
2. **[EXAMPLES.md](EXAMPLES.md)** - Copy-paste examples
   - Emoji handling
   - CSS strategies (5 options)
   - JavaScript patterns
   - Component patterns
   - HTMX & Datastar integration
   - Quick reference (what works / what doesn't)

### For Comparison & Analysis

3. **[COMPETITIVE_ANALYSIS.md](COMPETITIVE_ANALYSIS.md)** - Detailed comparison
   - Rusti vs Maud, Askama, Leptos, Dioxus, Tera
   - Feature matrix
   - Code comparisons
   - Use case recommendations

### Optional Deep Dives

4. **[EMOJI_AND_LIFETIMES.md](EMOJI_AND_LIFETIMES.md)** - Technical deep dive
   - Why emojis behave as they do
   - Lifetime management patterns
   - Advanced techniques

5. **[UPDATES_SUMMARY.md](UPDATES_SUMMARY.md)** - Recent changes
   - Documentation updates
   - Bug fixes
   - New features

---

## Quick Start Path

1. Read **[README.md](README.md)** sections:
   - Installation
   - Golden Rules
   - Emoji section
   
2. Copy examples from **[EXAMPLES.md](EXAMPLES.md)** for your use case

3. Check **[COMPETITIVE_ANALYSIS.md](COMPETITIVE_ANALYSIS.md)** if migrating from another framework

---

## Common Questions

### "How do I use emojis?"
➜ See [README.md - Emoji & Unicode](README.md#2-emoji--unicode-in-text)  
**TL;DR:** Use variables: `let text = "Hello ✅"; rusti! { <p>{text}</p> }`

### "What CSS approach should I use?"
➜ See [README.md - 5 Ways to Style](README.md#4-the-law-of-css-5-ways-to-style) or [EXAMPLES.md - CSS Styling](EXAMPLES.md#css-styling)  
**TL;DR:** Use Tailwind for easiest experience

### "How do I handle JavaScript?"
➜ See [README.md - Inline JavaScript](README.md#-special-case-inline-javascript) or [EXAMPLES.md - JavaScript](EXAMPLES.md#javascript)  
**TL;DR:** Use raw strings: `<script>{r#"..."#}</script>`

### "Why lifetimes?"
➜ See [EMOJI_AND_LIFETIMES.md - Lifetime Patterns](EMOJI_AND_LIFETIMES.md#question-2-can-we-avoid-lifetimes)  
**TL;DR:** Use `+ '_` for borrowed data, or `String` for owned

### "How does Rusti compare to Maud/Leptos?"
➜ See [COMPETITIVE_ANALYSIS.md](COMPETITIVE_ANALYSIS.md)

---

## File Hierarchy

```
rusti/
├── README.md                      ★ START HERE - Main docs
├── EXAMPLES.md                    ★ Copy-paste patterns
├── COMPETITIVE_ANALYSIS.md        Framework comparisons
├── todo.md                        Known issues & roadmap
├── EMOJI_AND_LIFETIMES.md        Technical deep dive
└── UPDATES_SUMMARY.md            Recent changes
```

---

## Contributing

Found a bug or want to add a feature?

1. Check [todo.md](todo.md) to see if it's a known issue
2. Read [COMPETITIVE_ANALYSIS.md](COMPETITIVE_ANALYSIS.md) to understand design decisions
3. Add examples to [EXAMPLES.md](EXAMPLES.md)
4. Update [README.md](README.md) if changing syntax

---

Made with ❤️ using Rusti (via Rust variables, of course! 😄)
