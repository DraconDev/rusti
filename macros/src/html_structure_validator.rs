use crate::token_parser::Element;
use proc_macro2::TokenStream;
use quote::quote_spanned;

/// Rule 1: Tables can only contain specific children
pub fn validate_table_children(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "table" {
        return vec![];
    }

    let valid_table_children = [
        "caption", "colgroup", "thead", "tbody", "tfoot", "tr", "style", "script", "template",
    ];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if !valid_table_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <table>. Tables can only contain: caption, colgroup, thead, tbody, tfoot, tr, style, script, template. Browser will hoist this element outside the table.",
                    child_elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 2: Lists can only contain <li>, script, or template
pub fn validate_list_children(elem: &Element) -> Vec<TokenStream> {
    if elem.name != "ul" && elem.name != "ol" {
        return vec![];
    }

    let valid_list_children = ["li", "script", "template"];

    let mut errors = vec![];

    for child in &elem.children {
        if let crate::token_parser::Node::Element(child_elem) = child {
            if !valid_list_children.contains(&child_elem.name.as_str()) {
                let msg = format!(
                    "Invalid <{}> inside <{}>. Lists can only contain <li>, script, or template. This breaks accessibility - screen readers will report 0 items.",
                    child_elem.name, elem.name
                );
                errors.push(quote_spanned! { child_elem.span =>
                    compile_error!(#msg);
                });
            }
        }
    }

    errors
}

/// Rule 3: Forms cannot be nested
pub fn validate_nested_forms(elem: &Element, is_inside_form: bool) -> Vec<TokenStream> {
    let mut errors = vec![];

    if elem.name == "form" && is_inside_form {
        let msg = "Nested <form> is not allowed. Browsers will delete nested forms, breaking submit logic.";
        errors.push(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    errors
}

/// Rule 4: Buttons cannot contain interactive elements
pub fn validate_button_interactive(elem: &Element, is_inside_button: bool) -> Vec<TokenStream> {
    let mut errors = vec![];

    let interactive_elements = ["a", "button", "input", "select", "textarea", "label"];

    if is_inside_button && interactive_elements.contains(&elem.name.as_str()) {
        let msg = format!(
            "Invalid <{}> inside <button>. Buttons cannot contain interactive elements (a, button, input, select, textarea, label). This creates undefined click behavior.",
            elem.name
        );
        errors.push(quote_spanned! { elem.span =>
            compile_error!(#msg);
        });
    }

    errors
}
