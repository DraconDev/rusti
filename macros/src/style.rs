use crate::css::{extract_selectors, rename_css_selectors};
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct StyleOutput {
    pub bindings: TokenStream,
    pub css: String,
}

pub fn process_style_macro(input: TokenStream) -> StyleOutput {
    let css_content = input.to_string();

    // Remove quotes if the input was parsed as a string literal (which might happen if passed as "...")
    // But usually style! { ... } passes tokens.
    // input.to_string() on tokens will produce " . class { ... } " with spaces.
    // extract_selectors and rename_css_selectors should handle spaces.

    // Generate a unique scope ID based on content
    let mut hasher = DefaultHasher::new();
    css_content.hash(&mut hasher);
    let hash = hasher.finish();
    let scope_id = format!("s{:x}", hash); // e.g. s1234abc

    let (classes, _ids) = extract_selectors(&css_content);

    let scoped_css = rename_css_selectors(&css_content, &scope_id);

    let mut bindings = TokenStream::new();
    for class in classes {
        let snake_name = class.to_snake_case();
        let ident = format_ident!("{}", snake_name);
        let scoped_class = format!("{}-{}", class, scope_id);

        bindings.extend(quote! {
            let #ident = #scoped_class;
        });
    }

    StyleOutput {
        bindings,
        css: scoped_css,
    }
}
