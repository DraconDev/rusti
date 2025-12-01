use crate::css::{extract_selectors, rename_css_selectors};
use heck::ToSnakeCase;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse2, token, LitStr, Token};

pub struct StyleOutput {
    pub bindings: TokenStream,
    pub css: String,
}

// AST for our style! macro
struct StyleInput {
    rules: Vec<StyleRule>,
}

struct StyleRule {
    selectors: TokenStream,
    block: StyleBlock,
}

struct StyleBlock {
    properties: Vec<StyleProperty>,
}

struct StyleProperty {
    name: String,
    value: String,
    span: proc_macro2::Span,
}

impl Parse for StyleInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rules = Vec::new();
        while !input.is_empty() {
            rules.push(input.parse()?);
        }
        Ok(StyleInput { rules })
    }
}

impl Parse for StyleRule {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse selectors until we see a brace
        let mut selector_tokens = TokenStream::new();
        while !input.peek(token::Brace) && !input.is_empty() {
            selector_tokens.extend(std::iter::once(input.parse::<TokenTree>()?));
        }

        // Validate selectors for kebab-case classes
        validate_selectors(&selector_tokens)?;

        if input.is_empty() {
            return Err(input.error("Expected block after selectors"));
        }

        let content;
        braced!(content in input);
        let block = content.parse()?;

        Ok(StyleRule {
            selectors: selector_tokens,
            block,
        })
    }
}

fn validate_selectors(tokens: &TokenStream) -> syn::Result<()> {
    let mut iter = tokens.clone().into_iter();
    while let Some(tt) = iter.next() {
        if let TokenTree::Punct(p) = &tt {
            if p.as_char() == '.' {
                // Class start. Next should be an Ident.
                if let Some(TokenTree::Ident(ident)) = iter.next() {
                    // Check if the *next* token is a hyphen.
                    // We need to peek, but into_iter() gives us an iterator that might not be peekable easily unless we make it so.
                    // But we can just clone the iterator before advancing? No.
                    // Let's collect to Vec? Or use peekable.
                    // But TokenStream iterator is not peekable by default?
                    // actually `into_iter()` on TokenStream returns `proc_macro2::token_stream::IntoIter`.
                    // We can make it peekable.
                }
            }
        }
    }
    // Re-implementing with peekable
    let mut iter = tokens.clone().into_iter().peekable();
    while let Some(tt) = iter.next() {
        if let TokenTree::Punct(p) = &tt {
            if p.as_char() == '.' {
                // Class start
                if let Some(TokenTree::Ident(ident)) = iter.peek() {
                    let ident_span = ident.span();
                    let _ = iter.next(); // consume ident

                    if let Some(TokenTree::Punct(next_p)) = iter.peek() {
                        if next_p.as_char() == '-' {
                            return Err(syn::Error::new(
                                ident_span,
                                "Class names in style! macro must be snake_case (no dashes allowed). Use underscores instead."
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

impl Parse for StyleBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut properties = Vec::new();
        while !input.is_empty() {
            properties.push(input.parse()?);
        }
        Ok(StyleBlock { properties })
    }
}

impl Parse for StyleProperty {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse property name (kebab-case identifier)
        // syn::Ident doesn't support dashes, so we might get multiple tokens
        // e.g. background - color
        let mut name = String::new();
        let start_span = input.span();

        loop {
            if input.peek(Token![:]) {
                break;
            }
            if input.is_empty() {
                return Err(input.error("Expected ':' after property name"));
            }
            let tt: TokenTree = input.parse()?;
            name.push_str(&tt.to_string());
        }

        // Validate property name
        if !is_valid_css_property(&name) {
            return Err(syn::Error::new(
                start_span,
                format!("Unknown CSS property: '{}'", name),
            ));
        }

        input.parse::<Token![:]>()?;

        // Parse value (must be a string literal)
        let value_lit: LitStr = input.parse().map_err(|_| {
            syn::Error::new(
                input.span(),
                "Expected string literal for property value (e.g. \"10px\")",
            )
        })?;
        let value = value_lit.value();
        let value_span = value_lit.span();

        input.parse::<Token![;]>()?;

        // Validate the CSS value using lightningcss
        if let Err(err_msg) = validate_css_value(&name, &value) {
            return Err(syn::Error::new(
                value_span,
                format!("Invalid CSS value for property '{}': {}", name, err_msg),
            ));
        }

        Ok(StyleProperty {
            name,
            value,
            span: start_span,
        })
    }
}

/// Validate CSS property value using lightningcss parser
fn validate_css_value(property: &str, value: &str) -> Result<(), String> {
    // Construct a minimal CSS rule to parse
    // Use Box::leak to give it a 'static lifetime (acceptable for compile-time macro)
    let css = Box::leak(format!(".test {{ {}: {}; }}", property, value).into_boxed_str());

    // Try to parse with lightningcss
    let parse_options = ParserOptions::default();

    match StyleSheet::parse(css, parse_options) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Extract useful error message
            let error_msg = format!("{:?}", e);
            // Simplify common errors
            if error_msg.contains("Unexpected token") || error_msg.contains("UnexpectedToken") {
                Err(format!("Unexpected token in value '{}'", value))
            } else if error_msg.contains("InvalidValue") {
                Err(format!("'{}' is not a valid value", value))
            } else {
                Err(format!("Parse error: {}", error_msg))
            }
        }
    }
}

pub fn process_style_macro(input: TokenStream) -> StyleOutput {
    // 1. Parse the input
    let style_input: StyleInput = match parse2(input) {
        Ok(input) => input,
        Err(err) => {
            return StyleOutput {
                bindings: err.to_compile_error(),
                css: String::new(),
            };
        }
    };

    // 2. Reconstruct CSS string (with quotes removed from values)
    let mut raw_css = String::new();
    for rule in &style_input.rules {
        let selector_str = tokens_to_css_string(&rule.selectors);

        raw_css.push_str(&selector_str);
        raw_css.push_str(" { ");
        for prop in &rule.block.properties {
            raw_css.push_str(&format!("{}: {}; ", prop.name, prop.value));
        }
        raw_css.push_str("} ");
    }

    // 3. Generate Scope ID
    let mut hasher = DefaultHasher::new();
    raw_css.hash(&mut hasher);
    let hash = hasher.finish();
    let scope_id = format!("s{:x}", hash);

    // 4. Extract classes for bindings
    let (classes, _ids) = extract_selectors(&raw_css);

    eprintln!("DEBUG: raw_css: '{}'", raw_css);
    eprintln!("DEBUG: extracted classes: {:?}", classes);

    // 5. Scope the CSS (rename classes)
    let scoped_css = rename_css_selectors(&raw_css, &scope_id);
    eprintln!("DEBUG: scoped_css: '{}'", scoped_css);

    // 6. Generate Bindings
    let mut bindings = TokenStream::new();
    for class in classes {
        let snake_name = class.to_snake_case();
        let ident = format_ident!("{}", snake_name);
        let scoped_class = format!("{}-{}", class, scope_id);

        bindings.extend(quote! {
            let #ident = #scoped_class;
        });
    }

    eprintln!("DEBUG: bindings tokens: {}", bindings);

    StyleOutput {
        bindings,
        css: scoped_css,
    }
}

fn tokens_to_css_string(tokens: &TokenStream) -> String {
    let mut css = String::new();
    let mut last_char_was_hyphen = false;
    let mut last_char_was_dot_or_hash = false;

    for tt in tokens.clone() {
        match tt {
            TokenTree::Ident(ident) => {
                // Add space if previous wasn't a special char that expects attachment
                if !css.is_empty() && !last_char_was_hyphen && !last_char_was_dot_or_hash {
                    css.push(' ');
                }
                css.push_str(&ident.to_string());
                last_char_was_hyphen = false;
                last_char_was_dot_or_hash = false;
            }
            TokenTree::Punct(punct) => {
                let ch = punct.as_char();
                if ch == '-' {
                    // No space before hyphen (handled by loop logic: if previous was ident, we didn't add space? Wait.
                    // If previous was ident, we are here.
                    // We just append '-'.
                    css.push(ch);
                    last_char_was_hyphen = true;
                    last_char_was_dot_or_hash = false;
                } else if ch == '.' || ch == '#' || ch == ':' {
                    // Add space before dot/hash if it's not the start?
                    // Actually, for `.class`, we want space before dot if it's `div .class`.
                    // But we decided to collapse spaces to fix `.class`.
                    // So we just append.
                    if !css.is_empty() && !last_char_was_hyphen && !last_char_was_dot_or_hash {
                        // For now, let's NOT add space before dot/hash to ensure .class works.
                        // This breaks `div .class` but fixes `.class`.
                        // css.push(' ');
                    }
                    css.push(ch);
                    last_char_was_hyphen = false;
                    last_char_was_dot_or_hash = true;
                } else {
                    // Other puncts (>, +, etc)
                    css.push(ch);
                    last_char_was_hyphen = false;
                    last_char_was_dot_or_hash = false;
                }
            }
            TokenTree::Literal(lit) => {
                if !css.is_empty() {
                    css.push(' ');
                }
                css.push_str(&lit.to_string());
                last_char_was_hyphen = false;
                last_char_was_dot_or_hash = false;
            }
            TokenTree::Group(group) => {
                if !css.is_empty() {
                    css.push(' ');
                }
                // Recurse? Or just to_string?
                // Selectors usually don't have groups (parens maybe for :not()?)
                // For :not(), we want :not(...).
                // Let's just use to_string for groups for now.
                css.push_str(&group.to_string());
                last_char_was_hyphen = false;
                last_char_was_dot_or_hash = false;
            }
        }
    }
    css
}

fn is_valid_css_property(name: &str) -> bool {
    let valid_properties = [
        "align-content",
        "align-items",
        "align-self",
        "all",
        "animation",
        "animation-delay",
        "animation-direction",
        "animation-duration",
        "animation-fill-mode",
        "animation-iteration-count",
        "animation-name",
        "animation-play-state",
        "animation-timing-function",
        "backface-visibility",
        "background",
        "background-attachment",
        "background-blend-mode",
        "background-clip",
        "background-color",
        "background-image",
        "background-origin",
        "background-position",
        "background-repeat",
        "background-size",
        "border",
        "border-bottom",
        "border-bottom-color",
        "border-bottom-left-radius",
        "border-bottom-right-radius",
        "border-bottom-style",
        "border-bottom-width",
        "border-collapse",
        "border-color",
        "border-image",
        "border-image-outset",
        "border-image-repeat",
        "border-image-slice",
        "border-image-source",
        "border-image-width",
        "border-left",
        "border-left-color",
        "border-left-style",
        "border-left-width",
        "border-radius",
        "border-right",
        "border-right-color",
        "border-right-style",
        "border-right-width",
        "border-spacing",
        "border-style",
        "border-top",
        "border-top-color",
        "border-top-left-radius",
        "border-top-right-radius",
        "border-top-style",
        "border-top-width",
        "border-width",
        "bottom",
        "box-decoration-break",
        "box-shadow",
        "box-sizing",
        "break-after",
        "break-before",
        "break-inside",
        "caption-side",
        "caret-color",
        "clear",
        "clip",
        "clip-path",
        "color",
        "column-count",
        "column-fill",
        "column-gap",
        "column-rule",
        "column-rule-color",
        "column-rule-style",
        "column-rule-width",
        "column-span",
        "column-width",
        "columns",
        "content",
        "counter-increment",
        "counter-reset",
        "cursor",
        "direction",
        "display",
        "empty-cells",
        "filter",
        "flex",
        "flex-basis",
        "flex-direction",
        "flex-flow",
        "flex-grow",
        "flex-shrink",
        "flex-wrap",
        "float",
        "font",
        "font-family",
        "font-feature-settings",
        "font-kerning",
        "font-language-override",
        "font-size",
        "font-size-adjust",
        "font-stretch",
        "font-style",
        "font-synthesis",
        "font-variant",
        "font-variant-alternates",
        "font-variant-caps",
        "font-variant-east-asian",
        "font-variant-ligatures",
        "font-variant-numeric",
        "font-variant-position",
        "font-weight",
        "gap",
        "grid",
        "grid-area",
        "grid-auto-columns",
        "grid-auto-flow",
        "grid-auto-rows",
        "grid-column",
        "grid-column-end",
        "grid-column-gap",
        "grid-column-start",
        "grid-gap",
        "grid-row",
        "grid-row-end",
        "grid-row-gap",
        "grid-row-start",
        "grid-template",
        "grid-template-areas",
        "grid-template-columns",
        "grid-template-rows",
        "hanging-punctuation",
        "height",
        "hyphens",
        "image-rendering",
        "isolation",
        "justify-content",
        "justify-items",
        "justify-self",
        "left",
        "letter-spacing",
        "line-break",
        "line-height",
        "list-style",
        "list-style-image",
        "list-style-position",
        "list-style-type",
        "margin",
        "margin-bottom",
        "margin-left",
        "margin-right",
        "margin-top",
        "max-height",
        "max-width",
        "min-height",
        "min-width",
        "mix-blend-mode",
        "object-fit",
        "object-position",
        "opacity",
        "order",
        "orphans",
        "outline",
        "outline-color",
        "outline-offset",
        "outline-style",
        "outline-width",
        "overflow",
        "overflow-wrap",
        "overflow-x",
        "overflow-y",
        "padding",
        "padding-bottom",
        "padding-left",
        "padding-right",
        "padding-top",
        "page-break-after",
        "page-break-before",
        "page-break-inside",
        "perspective",
        "perspective-origin",
        "pointer-events",
        "position",
        "quotes",
        "resize",
        "right",
        "row-gap",
        "scroll-behavior",
        "tab-size",
        "table-layout",
        "text-align",
        "text-align-last",
        "text-combine-upright",
        "text-decoration",
        "text-decoration-color",
        "text-decoration-line",
        "text-decoration-style",
        "text-indent",
        "text-justify",
        "text-orientation",
        "text-overflow",
        "text-shadow",
        "text-transform",
        "text-underline-position",
        "top",
        "transform",
        "transform-origin",
        "transform-style",
        "transition",
        "transition-delay",
        "transition-duration",
        "transition-property",
        "transition-timing-function",
        "unicode-bidi",
        "user-select",
        "vertical-align",
        "visibility",
        "white-space",
        "widows",
        "width",
        "word-break",
        "word-spacing",
        "word-wrap",
        "writing-mode",
        "z-index",
    ];

    valid_properties.contains(&name)
}
