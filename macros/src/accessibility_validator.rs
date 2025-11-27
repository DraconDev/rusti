use crate::token_parser::{Attribute, AttributeValue, Element};
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::Error;

/// Rule 1: Every <img> tag MUST have an alt attribute
pub fn validate_img_alt(elem: &Element) -> Option<TokenStream> {
    if elem.name != "img" {
        return None;
    }

    // Check if alt attribute exists
    let has_alt = elem.attrs.iter().any(|attr| attr.name == "alt");

    if !has_alt {
        Some(quote_spanned! { elem.span =>
            compile_error!("Image is missing 'alt' attribute. If decorative, use alt=\"\".");
        })
    } else {
        None
    }
}

/// Rule 2: The type attribute on <input> and <button> must be valid
pub fn validate_input_type(elem: &Element) -> Option<TokenStream> {
    // Only validate input and button elements
    if elem.name != "input" && elem.name != "button" {
        return None;
    }

    // Find type attribute
    let type_attr = elem.attrs.iter().find(|attr| attr.name == "type")?;

    // Only validate static values (not dynamic expressions)
    if let AttributeValue::Static(type_value) = &type_attr.value {
        let valid = match elem.name.as_str() {
            "input" => is_valid_input_type(type_value),
            "button" => is_valid_button_type(type_value),
            _ => return None,
        };

        if !valid {
            let suggestion = suggest_type_correction(type_value, elem.name == "input");
            let msg = format!(
                "Invalid {} type='{}'. {}",
                elem.name, type_value, suggestion
            );

            return Some(quote_spanned! { type_attr.span =>
                compile_error!(#msg);
            });
        }
    }

    None
}

/// Rule 3: ARIA role values must be valid
pub fn validate_aria_roles(elem: &Element) -> Option<TokenStream> {
    // Find role attribute
    let role_attr = elem.attrs.iter().find(|attr| attr.name == "role")?;

    // Only validate static values
    if let AttributeValue::Static(role_value) = &role_attr.value {
        if !is_valid_aria_role(role_value) {
            let msg = format!(
                "Invalid ARIA role='{}'. See https://www.w3.org/TR/wai-aria-1.2/#role_definitions for valid roles.",
                role_value
            );

            return Some(quote_spanned! { role_attr.span =>
                compile_error!(#msg);
            });
        }
    }

    None
}

// Helper: Valid input types (HTML5)
fn is_valid_input_type(type_value: &str) -> bool {
    matches!(
        type_value,
        "text"
            | "password"
            | "email"
            | "number"
            | "tel"
            | "url"
            | "search"
            | "date"
            | "time"
            | "datetime-local"
            | "month"
            | "week"
            | "color"
            | "file"
            | "hidden"
            | "checkbox"
            | "radio"
            | "submit"
            | "reset"
            | "button"
            | "range"
            | "image"
    )
}

// Helper: Valid button types
fn is_valid_button_type(type_value: &str) -> bool {
    matches!(type_value, "button" | "submit" | "reset")
}

// Helper: Suggest corrections for common typos
fn suggest_type_correction(type_value: &str, is_input: bool) -> String {
    match type_value {
        "txt" => "Did you mean 'text'?".to_string(),
        "sumbit" => "Did you mean 'submit'?".to_string(),
        "buton" => "Did you mean 'button'?".to_string(),
        "num" => "Did you mean 'number'?".to_string(),
        "mail" => "Did you mean 'email'?".to_string(),
        _ => {
            if is_input {
                "Valid types: text, password, email, number, tel, url, search, date, time, checkbox, radio, submit, reset, button, etc.".to_string()
            } else {
                "Valid types: button, submit, reset".to_string()
            }
        }
    }
}

// Helper: Valid ARIA roles (WAI-ARIA 1.2)
fn is_valid_aria_role(role_value: &str) -> bool {
    matches!(
        role_value,
        // Widget roles
        "button" | "checkbox" | "gridcell" | "link" | "menuitem" | "menuitemcheckbox"
            | "menuitemradio" | "option" | "progressbar" | "radio" | "scrollbar"
            | "searchbox" | "separator" | "slider" | "spinbutton" | "switch"
            | "tab" | "tabpanel" | "textbox" | "treeitem"
            // Composite widget roles
            | "combobox" | "grid" | "listbox" | "menu" | "menubar" | "radiogroup"
            | "tablist" | "tree" | "treegrid"
            // Document structure roles
            | "application" | "article" | "cell" | "columnheader" | "definition"
            | "directory" | "document" | "feed" | "figure" | "group" | "heading"
            | "img" | "list" | "listitem" | "math" | "none" | "note" | "presentation"
            | "row" | "rowgroup" | "rowheader" | "separator" | "table" | "term"
            | "toolbar" | "tooltip"
            // Landmark roles
            | "banner" | "complementary" | "contentinfo" | "form" | "main"
            | "navigation" | "region" | "search"
            // Live region roles
            | "alert" | "log" | "marquee" | "status" | "timer"
            // Window roles
            | "alertdialog" | "dialog"
    )
}
