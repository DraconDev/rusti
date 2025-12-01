// Force rebuild 3
mod component;

mod accessibility_validator;
mod css;
mod css_validator;
mod head;
mod html_structure_validator;
#[cfg(feature = "schema")]
mod schema;
mod style;
mod test_spacing;
mod token_parser;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::spanned::Spanned;

#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
    head::expand_head(input)
}

#[cfg(feature = "schema")]
#[proc_macro_derive(Schema, attributes(schema))]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    schema::derive_schema(input)
}

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    component::expand_component(item)
}

struct NodesWrapper(Vec<token_parser::Node>);

impl Parse for NodesWrapper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        token_parser::parse_nodes(input).map(NodesWrapper)
    }
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as token_parser::HtmlInput);
    let nodes = input.nodes;

    // 1. Process styles (hoist <style> tags)
    let (style_bindings, scoped_css) = process_styles(&nodes);

    // 2. CSS dependencies are no longer collected for external files
    let css_deps: Vec<proc_macro2::TokenStream> = Vec::new();

    // 3. Generate HTML string construction code
    let html_construction = generate_nodes(&nodes);

    // 4. Generate bind validation checks
    let mut validation_checks = Vec::new();
    collect_bind_checks(&nodes, &mut validation_checks);

    let expanded = quote! {
        {
            // Inject style bindings (hoisted)
            #style_bindings

            // CSS dependency tracking (forces recompile when CSS changes)
            #(#css_deps)*

            // Validation block (compile-time only)
            const _: () = {
                #(#validation_checks)*
            };

            // Runtime HTML generation
            azumi::from_fn(move |f| {
                // Inject scoped CSS if present
                if !#scoped_css.is_empty() {
                     write!(f, "<style data-azumi-internal=\"true\">{}</style>", #scoped_css)?;
                }
                #html_construction.render(f)
            })
        }
    };

    TokenStream::from(expanded)
}

fn process_styles(nodes: &[token_parser::Node]) -> (proc_macro2::TokenStream, String) {
    let mut bindings = proc_macro2::TokenStream::new();
    let mut css_output = String::new();

    for node in nodes {
        match node {
            token_parser::Node::Block(token_parser::Block::Style(style_block)) => {
                let output = style::process_style_macro(style_block.content.clone());
                bindings.extend(output.bindings);
                css_output.push_str(&output.css);
            }
            token_parser::Node::Element(elem) => {
                let (child_bindings, child_css) = process_styles(&elem.children);
                bindings.extend(child_bindings);
                css_output.push_str(&child_css);
            }
            token_parser::Node::Fragment(frag) => {
                let (child_bindings, child_css) = process_styles(&frag.children);
                bindings.extend(child_bindings);
                css_output.push_str(&child_css);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    let (b, c) = process_styles(&if_block.then_branch);
                    bindings.extend(b);
                    css_output.push_str(&c);
                    if let Some(else_branch) = &if_block.else_branch {
                        let (b, c) = process_styles(else_branch);
                        bindings.extend(b);
                        css_output.push_str(&c);
                    }
                }
                token_parser::Block::For(for_block) => {
                    let (b, c) = process_styles(&for_block.body);
                    bindings.extend(b);
                    css_output.push_str(&c);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        let (b, c) = process_styles(&arm.body);
                        bindings.extend(b);
                        css_output.push_str(&c);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    (bindings, css_output)
}

fn collect_bind_checks(nodes: &[token_parser::Node], checks: &mut Vec<proc_macro2::TokenStream>) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                // If this element has bind={Struct}, generate a check function
                if let Some(struct_path) = &elem.bind_struct {
                    let mut field_accesses = Vec::new();
                    collect_input_names(&elem.children, struct_path, &mut field_accesses);

                    if !field_accesses.is_empty() {
                        let check_fn_name =
                            quote::format_ident!("azumi_bind_check_{}", checks.len());

                        let check_block = quote! {
                            #[allow(unused_variables, non_snake_case)]
                            fn #check_fn_name(data: &#struct_path) {
                                #(#field_accesses)*
                            }
                        };
                        checks.push(check_block);
                    }
                }
                // Recurse
                collect_bind_checks(&elem.children, checks);
            }
            token_parser::Node::Fragment(frag) => {
                collect_bind_checks(&frag.children, checks);
            }
            token_parser::Node::Block(block) => {
                // Handle blocks (if, for, match, etc.) by recursing into their bodies
                match block {
                    token_parser::Block::If(if_block) => {
                        collect_bind_checks(&if_block.then_branch, checks);
                        if let Some(else_branch) = &if_block.else_branch {
                            collect_bind_checks(else_branch, checks);
                        }
                    }
                    token_parser::Block::For(for_block) => {
                        collect_bind_checks(&for_block.body, checks);
                    }
                    token_parser::Block::Match(match_block) => {
                        for arm in &match_block.arms {
                            collect_bind_checks(&arm.body, checks);
                        }
                    }
                    token_parser::Block::Call(call_block) => {
                        collect_bind_checks(&call_block.children, checks);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn collect_input_names(
    nodes: &[token_parser::Node],
    bind_struct: &syn::Path,
    errors: &mut Vec<proc_macro2::TokenStream>,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "input" || elem.name == "textarea" || elem.name == "select" {
                    // Find "name" attribute
                    for attr in &elem.attrs {
                        if attr.name == "name" {
                            if let token_parser::AttributeValue::Static(name_str) = &attr.value {
                                // Use the span of the attribute value for better error reporting
                                let span = attr.value_span.unwrap_or(attr.span);
                                let parts: Vec<&str> = name_str.split('.').collect();

                                // Validate each part is a valid Rust identifier
                                let mut all_valid = true;
                                for part in &parts {
                                    if !is_valid_identifier(part) {
                                        all_valid = false;
                                        let error_msg = format!(
                                            "Invalid field name '{}' in form binding. Field names must be valid Rust identifiers (no spaces or special characters).",
                                            part
                                        );
                                        errors.push(quote_spanned! {span=>
                                            compile_error!(#error_msg);
                                        });
                                        break;
                                    }
                                }

                                if !all_valid {
                                    continue;
                                }

                                // Generate identifiers for the field path
                                let field_idents: Vec<proc_macro2::Ident> = parts
                                    .iter()
                                    .map(|s| proc_macro2::Ident::new(s, span))
                                    .collect();

                                // Generate validation that field exists on struct
                                errors.push(quote! {
                                    let _ = &data.#(#field_idents).*;
                                });
                            }
                        }
                    }
                }
                // Recurse (e.g. input inside label or div)
                collect_input_names(&elem.children, bind_struct, errors);
            }
            token_parser::Node::Fragment(frag) => {
                collect_input_names(&frag.children, bind_struct, errors);
            }
            _ => {}
        }
    }
}

// Helper function to check if a string is a valid Rust identifier
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let mut chars = s.chars();
    let first = chars.next().unwrap();

    // First character must be alphabetic or underscore
    if !first.is_alphabetic() && first != '_' {
        return false;
    }

    // Remaining characters must be alphanumeric or underscore
    for c in chars {
        if !c.is_alphanumeric() && c != '_' {
            return false;
        }
    }

    true
}

fn generate_nodes(nodes: &[token_parser::Node]) -> proc_macro2::TokenStream {
    let body = generate_body(nodes);
    quote! {
        azumi::from_fn(move |f| {
            #body
            Ok(())
        })
    }
}

/// Strip outer quotes from string literals for cleaner text rendering
/// If the user wants literal quotes, they should use raw strings like r#""Hello""#
fn strip_outer_quotes(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.len() >= 2
        && ((trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\'')))
    {
        return trimmed[1..trimmed.len() - 1].to_string();
    }
    s.to_string()
}

#[derive(Clone, PartialEq, Debug)]
enum Context {
    Normal,
    Script,
}

#[derive(Clone, Debug)]
struct GenerationContext {
    mode: Context,
    scope_id: Option<String>,
}

impl GenerationContext {
    fn normal() -> Self {
        Self {
            mode: Context::Normal,
            scope_id: None,
        }
    }

    fn with_scope(scope_id: String) -> Self {
        Self {
            mode: Context::Normal,
            scope_id: Some(scope_id),
        }
    }

    fn with_mode(&self, mode: Context) -> Self {
        Self {
            mode,
            scope_id: self.scope_id.clone(),
        }
    }
}

/// Collect ALL CSS content from all <style> tags in the component
/// Returns (global_css, scoped_css)
fn collect_all_styles(nodes: &[token_parser::Node]) -> (String, String) {
    let mut global_css = String::new();
    let mut scoped_css = String::new();
    collect_styles_recursive(nodes, &mut global_css, &mut scoped_css);
    (global_css, scoped_css)
}

fn collect_styles_recursive(
    nodes: &[token_parser::Node],
    global_css: &mut String,
    scoped_css: &mut String,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "style" {
                    // Extract CSS from this style tag
                    if let Some(_src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        // External CSS is banned, do nothing here.
                        // Validation will catch it.
                    } else {
                        // Inline content - treat as scoped
                        for child in &elem.children {
                            if let token_parser::Node::Text(text) = child {
                                scoped_css.push_str(&text.content);
                                scoped_css.push('\n');
                            }
                        }
                    }
                } else {
                    // Recurse into children
                    collect_styles_recursive(&elem.children, global_css, scoped_css);
                }
            }
            token_parser::Node::Fragment(frag) => {
                collect_styles_recursive(&frag.children, global_css, scoped_css);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    collect_styles_recursive(&if_block.then_branch, global_css, scoped_css);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_styles_recursive(else_branch, global_css, scoped_css);
                    }
                }
                token_parser::Block::For(for_block) => {
                    collect_styles_recursive(&for_block.body, global_css, scoped_css);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_styles_recursive(&arm.body, global_css, scoped_css);
                    }
                }
                token_parser::Block::Call(call_block) => {
                    collect_styles_recursive(&call_block.children, global_css, scoped_css);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn generate_body(nodes: &[token_parser::Node]) -> proc_macro2::TokenStream {
    // Pass 0: CSS Validation - Revolutionary compile-time CSS type checking!
    let css_validation_errors = css_validator::validate_component_css(nodes);

    // If there are compile errors from CSS validation, emit them
    if !css_validation_errors.is_empty() {
        return css_validation_errors;
    }

    // Collect all CSS content first (needed for validation and scoping)
    let (global_css, scoped_css) = collect_all_styles(nodes);

    // Pass 0.5: Strict CSS Validation (New Feature)
    // Extract valid selectors from the SCOPED CSS only (global CSS is opt-out)
    let (valid_classes, valid_ids) = crate::css::extract_selectors(&scoped_css);

    // Validate nodes against strict rules - COMPILE ERRORS
    let style_validation_errors =
        validate_nodes(nodes, &valid_classes, &valid_ids, !scoped_css.is_empty());
    if !style_validation_errors.is_empty() {
        return style_validation_errors;
    }

    // Pass 1: Check if component has any style tags
    let has_global = !global_css.is_empty();
    let has_scoped = !scoped_css.is_empty();

    if has_global || has_scoped {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Generate body
        let body = if has_scoped {
            // Generate scope ID from scoped CSS only
            let mut hasher = DefaultHasher::new();
            scoped_css.hash(&mut hasher);
            let hash = hasher.finish();
            let scope_id = format!("s{:x}", hash);

            // Scope the CSS
            let scoped_output = crate::css::scope_css(&scoped_css, &scope_id);

            // Generate body with scope context
            let ctx = GenerationContext::with_scope(scope_id.clone());
            let body_content = generate_body_with_context(nodes, &ctx);

            // Inject global CSS first (unscoped), then scoped CSS
            if has_global {
                quote! {
                    write!(f, "<style>{}</style>", #global_css)?;
                    write!(f, "<style>{}</style>", #scoped_output)?;
                    #body_content
                }
            } else {
                quote! {
                    write!(f, "<style>{}</style>", #scoped_output)?;
                    #body_content
                }
            }
        } else {
            // Only global CSS, no scoping needed
            let ctx = GenerationContext::normal();
            let body_content = generate_body_with_context(nodes, &ctx);

            quote! {
                write!(f, "<style>{}</style>", #global_css)?;
                #body_content
            }
        };

        body
    } else {
        generate_body_with_context(nodes, &GenerationContext::normal())
    }
}

fn validate_nodes(
    nodes: &[token_parser::Node],
    valid_classes: &std::collections::HashSet<String>,
    valid_ids: &std::collections::HashSet<String>,
    has_scoped_css: bool,
) -> proc_macro2::TokenStream {
    use quote::quote_spanned;
    let mut errors = vec![];

    fn collect_errors_recursive(
        nodes: &[token_parser::Node],
        valid_classes: &std::collections::HashSet<String>,
        valid_ids: &std::collections::HashSet<String>,
        has_scoped_css: bool,
        errors: &mut Vec<proc_macro2::TokenStream>,
        is_inside_form: bool,
        is_inside_button: bool,
        is_inside_anchor: bool,
    ) {
        for node in nodes {
            match node {
                token_parser::Node::Element(elem) => {
                    for attr in &elem.attrs {
                        let name = &attr.name;

                        // Rule 1: Ban inline styles - COMPILE ERROR
                        if name == "style" {
                            errors.push(quote_spanned! { attr.span =>
                                compile_error!("Inline styles banned. Use CSS classes instead.");
                            });
                        }

                        // Rule 2: Check class existence - COMPILE ERROR
                        if name == "class" {
                            if let token_parser::AttributeValue::Static(val) = &attr.value {
                                for class_name in val.split_whitespace() {
                                    if !valid_classes.contains(class_name) {
                                        let msg = if has_scoped_css {
                                            format!(
                                                "CSS class '{}' is not defined in any CSS file. Check for typos or add the class to your CSS.",
                                                class_name
                                            )
                                        } else {
                                            format!(
                                                "CSS class '{}' is used but no CSS styles are defined for this component. Import a CSS file with <style src=\"...\" />.",
                                                class_name
                                            )
                                        };
                                        // Use value_span to point to the attribute value, fallback to attr.span
                                        let error_span = attr.value_span.unwrap_or(attr.span);
                                        errors.push(quote_spanned! { error_span =>
                                            compile_error!(#msg);
                                        });
                                    }
                                }
                            }
                        }

                        // Rule 3: Check ID existence - COMPILE ERROR
                        if name == "id" {
                            if let token_parser::AttributeValue::Static(val) = &attr.value {
                                if !valid_ids.contains(val) {
                                    let msg = format!(
                                        "ID '{}' is used in HTML but not defined in CSS.",
                                        val
                                    );
                                    // Use value_span to point to the attribute value, fallback to attr.span
                                    let error_span = attr.value_span.unwrap_or(attr.span);
                                    errors.push(quote_spanned! { error_span =>
                                        compile_error!(#msg);
                                    });
                                }
                            }
                        }

                        // Rule 4: Validate attribute name (Strict HTML)
                        if let Some(err) = html_structure_validator::validate_attribute_name(attr) {
                            errors.push(err);
                        }
                    }

                    // Accessibility validation
                    if let Some(err) = accessibility_validator::validate_img_alt(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_input_type(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_aria_roles(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_button_content(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_anchor_target_blank(elem) {
                        errors.push(err);
                    }
                    if let Some(err) = accessibility_validator::validate_iframe_title(elem) {
                        errors.push(err);
                    }

                    // HTML Structure validation
                    if let Some(err) = html_structure_validator::validate_tag_name(elem) {
                        errors.push(err);
                    }
                    errors.extend(html_structure_validator::validate_table_children(elem));
                    errors.extend(html_structure_validator::validate_list_children(elem));
                    errors.extend(html_structure_validator::validate_nested_forms(
                        elem,
                        is_inside_form,
                    ));
                    errors.extend(html_structure_validator::validate_button_interactive(
                        elem,
                        is_inside_button,
                    ));
                    errors.extend(html_structure_validator::validate_paragraph_content(elem));
                    errors.extend(html_structure_validator::validate_anchor_nesting(
                        elem,
                        is_inside_anchor,
                    ));
                    errors.extend(html_structure_validator::validate_heading_content(elem));

                    // Update context for recursion
                    let new_is_inside_form = is_inside_form || elem.name.as_str() == "form";
                    let new_is_inside_button = is_inside_button || elem.name == "button";
                    let new_is_inside_anchor = is_inside_anchor || elem.name == "a";

                    // Recurse with updated context
                    collect_errors_recursive(
                        &elem.children,
                        valid_classes,
                        valid_ids,
                        has_scoped_css,
                        errors,
                        new_is_inside_form,
                        new_is_inside_button,
                        new_is_inside_anchor,
                    );
                }
                token_parser::Node::Fragment(frag) => {
                    collect_errors_recursive(
                        &frag.children,
                        valid_classes,
                        valid_ids,
                        has_scoped_css,
                        errors,
                        is_inside_form,
                        is_inside_button,
                        is_inside_anchor,
                    );
                }
                token_parser::Node::Block(block) => match block {
                    token_parser::Block::If(if_block) => {
                        collect_errors_recursive(
                            &if_block.then_branch,
                            valid_classes,
                            valid_ids,
                            has_scoped_css,
                            errors,
                            is_inside_form,
                            is_inside_button,
                            is_inside_anchor,
                        );
                        if let Some(else_branch) = &if_block.else_branch {
                            collect_errors_recursive(
                                else_branch,
                                valid_classes,
                                valid_ids,
                                has_scoped_css,
                                errors,
                                is_inside_form,
                                is_inside_button,
                                is_inside_anchor,
                            );
                        }
                    }
                    token_parser::Block::For(for_block) => {
                        collect_errors_recursive(
                            &for_block.body,
                            valid_classes,
                            valid_ids,
                            has_scoped_css,
                            errors,
                            is_inside_form,
                            is_inside_button,
                            is_inside_anchor,
                        );
                    }
                    token_parser::Block::Match(match_block) => {
                        for arm in &match_block.arms {
                            collect_errors_recursive(
                                &arm.body,
                                valid_classes,
                                valid_ids,
                                has_scoped_css,
                                errors,
                                is_inside_form,
                                is_inside_button,
                                is_inside_anchor,
                            );
                        }
                    }
                    token_parser::Block::Call(call_block) => {
                        collect_errors_recursive(
                            &call_block.children,
                            valid_classes,
                            valid_ids,
                            has_scoped_css,
                            errors,
                            is_inside_form,
                            is_inside_button,
                            is_inside_anchor,
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    collect_errors_recursive(
        nodes,
        valid_classes,
        valid_ids,
        has_scoped_css,
        &mut errors,
        false,
        false,
        false,
    );

    if errors.is_empty() {
        quote! {}
    } else {
        quote! { #(#errors)* }
    }
}

fn generate_body_with_context(
    nodes: &[token_parser::Node],
    ctx: &GenerationContext,
) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    for node in nodes {
        let chunk = match node {
            token_parser::Node::Element(elem) => {
                let name = &elem.name;

                // Skip <style> tags - they're already processed in generate_body
                if name == "style" {
                    continue;
                }

                // Determine context for children
                let child_context = if name == "script" {
                    ctx.with_mode(Context::Script)
                } else {
                    ctx.clone()
                };

                // Generate children
                let children_code = generate_body_with_context(&elem.children, &child_context);

                // Generate attributes
                let mut attr_code = proc_macro2::TokenStream::new();
                let mut css_vars = Vec::new();

                for attr in &elem.attrs {
                    let attr_name = &attr.name;

                    // Check for CSS variables (start with --)
                    if attr_name.starts_with("--") {
                        css_vars.push(attr);
                        continue;
                    }

                    match &attr.value {
                        token_parser::AttributeValue::Static(val) => {
                            attr_code.extend(quote! {
                                write!(f, " {}=\"{}\"", #attr_name, azumi::Escaped(#val))?;
                            });
                        }
                        token_parser::AttributeValue::Dynamic(expr) => {
                            attr_code.extend(quote! {
                                write!(f, " {}=\"{}\"", #attr_name, azumi::Escaped(&(#expr)))?;
                            });
                        }
                        token_parser::AttributeValue::None => {
                            // Boolean attribute
                            attr_code.extend(quote! {
                                write!(f, " {}", #attr_name)?;
                            });
                        }
                    }
                }

                // Generate style attribute for CSS variables if any exist
                if !css_vars.is_empty() {
                    let mut style_content = proc_macro2::TokenStream::new();
                    let mut first = true;

                    for attr in css_vars {
                        let name = &attr.name;
                        if !first {
                            style_content.extend(quote! { write!(f, "; ")?; });
                        }
                        first = false;

                        match &attr.value {
                            token_parser::AttributeValue::Static(val) => {
                                style_content.extend(quote! {
                                    write!(f, "{}: {}", #name, azumi::Escaped(#val))?;
                                });
                            }
                            token_parser::AttributeValue::Dynamic(expr) => {
                                style_content.extend(quote! {
                                    write!(f, "{}: {}", #name, azumi::Escaped(&(#expr)))?;
                                });
                            }
                            token_parser::AttributeValue::None => {
                                // Empty value for variable? Treat as empty string
                                style_content.extend(quote! {
                                    write!(f, "{}: ", #name)?;
                                });
                            }
                        }
                    }

                    attr_code.extend(quote! {
                        write!(f, " style=\"")?;
                        #style_content
                        write!(f, "\"")?;
                    });
                }

                // Generate element with potential scope attribute from context
                if let Some(ref scope_id) = ctx.scope_id {
                    quote! {
                        write!(f, "<{}", #name)?;
                        write!(f, " data-{}", #scope_id)?;
                        #attr_code
                        write!(f, ">")?;
                        #children_code
                        write!(f, "</{}>", #name)?;
                    }
                } else {
                    quote! {
                        write!(f, "<{}", #name)?;
                        #attr_code
                        write!(f, ">")?;
                        #children_code
                        write!(f, "</{}>", #name)?;
                    }
                }
            }
            token_parser::Node::Text(text) => {
                let content = &text.content;
                if content.is_empty() {
                    quote! {}
                } else {
                    // Strip outer quotes from string literals for cleaner rendering
                    let stripped = strip_outer_quotes(content);
                    quote! { write!(f, "{}", #stripped)?; }
                }
            }
            token_parser::Node::Expression(expr) => {
                let content = &expr.content;
                match ctx.mode {
                    Context::Script => {
                        // In JSON script tags, output raw (user provides JSON string via serde_json::to_string etc.)
                        quote! { write!(f, "{}", #content)?; }
                    }
                    Context::Normal => {
                        // In normal HTML, use Smart Interpolation
                        // RenderWrapper::render_azumi will pick:
                        // 1. Component::render (if it's a Component)
                        // 2. Escaped Display (if it's just Display)
                        quote! {
                            {
                                use azumi::FallbackRender; // Import trait for fallback
                                azumi::RenderWrapper(&(#content)).render_azumi(f)?;
                            }
                        }
                    }
                }
            }
            token_parser::Node::Comment(_) => {
                // Ignore comments in output
                quote! {}
            }
            token_parser::Node::Doctype(doctype) => {
                let content = &doctype.content;
                quote! { write!(f, "<!DOCTYPE {}>", #content)?; }
            }
            token_parser::Node::Fragment(frag) => generate_body_with_context(&frag.children, ctx),
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    let condition = &if_block.condition;
                    let then_code = generate_body_with_context(&if_block.then_branch, ctx);
                    let else_code = if let Some(else_branch) = &if_block.else_branch {
                        let else_body = generate_body_with_context(else_branch, ctx);
                        quote! { else { #else_body } }
                    } else {
                        quote! {}
                    };
                    quote! {
                        if #condition {
                            #then_code
                        } #else_code
                    }
                }
                token_parser::Block::For(for_block) => {
                    let pattern = &for_block.pattern;
                    let iterator = &for_block.iterator;
                    let body_code = generate_body_with_context(&for_block.body, ctx);
                    quote! {
                        for #pattern in #iterator {
                            #body_code
                        }
                    }
                }
                token_parser::Block::Match(match_block) => {
                    let expr = &match_block.expr;
                    let arms = match_block.arms.iter().map(|arm| {
                        let pattern = &arm.pattern;
                        let body = generate_body_with_context(&arm.body, ctx);
                        quote! {
                            #pattern => { #body }
                        }
                    });
                    quote! {
                        match #expr {
                            #(#arms),*
                        }
                    }
                }
                token_parser::Block::Call(call_block) => {
                    let name = &call_block.name;
                    let args = &call_block.args;
                    let has_children = !call_block.children.is_empty();
                    let children_code = if has_children {
                        let children_body = generate_body_with_context(&call_block.children, ctx);
                        quote! {
                            azumi::from_fn(|f| {
                                #children_body
                                Ok(())
                            })
                        }
                    } else {
                        quote! {}
                    };

                    // AZUMI 2.0: ENFORCE NAMED ARGUMENTS
                    // All component calls must use named arguments for clarity and maintainability
                    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::token::Comma>::parse_terminated;
                    let named_args = if let Ok(exprs) =
                        syn::parse::Parser::parse2(parser, args.clone())
                    {
                        // Check if we have positional arguments (not all are Assign expressions)
                        let has_positional = !exprs.is_empty()
                            && !exprs.iter().all(|e| matches!(e, syn::Expr::Assign(_)));

                        if has_positional {
                            // Positional arguments detected - generate clear compile error
                            // We enforce named arguments for ALL components to ensure #[component] usage
                            let name_str = quote!(#name).to_string();

                            // First, try to find the first positional argument to highlight
                            // This helps users see exactly which argument is the problem
                            let error_span = if let Some(first_positional) =
                                exprs.iter().find(|e| !matches!(e, syn::Expr::Assign(_)))
                            {
                                // Use the span of the first positional argument for precision
                                first_positional.span()
                            } else {
                                // Fallback to the entire call span
                                call_block.span
                            };

                            // Show the actual problematic call for better context
                            let positional_values: Vec<String> = exprs
                                .iter()
                                .filter(|e| !matches!(e, syn::Expr::Assign(_)))
                                .map(|e| {
                                    let s = quote!(#e).to_string();
                                    // Truncate long values
                                    if s.len() > 20 {
                                        format!("{}...", &s[..17])
                                    } else {
                                        s
                                    }
                                })
                                .collect();

                            let error_msg = if !positional_values.is_empty() {
                                format!(
                                        "Component '{}' must be called with named arguments.\n\
                                         \n\
                                         ❌ You wrote:\n\
                                         @{}({})\n\
                                         \n\
                                         ✅ Use named arguments instead:\n\
                                         @{}(param1={}, param2={}, ...)\n\
                                         \n\
                                         💡 Tip: Check the component definition to find the exact parameter names.\n\
                                         Named arguments prevent bugs when signatures change and make code self-documenting.",
                                        name_str,
                                        name_str,
                                        positional_values.join(", "),
                                        name_str,
                                        positional_values.get(0).unwrap_or(&"...".to_string()),
                                        positional_values.get(1).unwrap_or(&"...".to_string())
                                    )
                            } else {
                                format!(
                                        "Component '{}' must be called with named arguments.\n\
                                         \n\
                                         ❌ Positional arguments are not allowed:\n\
                                         @{}(\"value1\", \"value2\")  // Error!\n\
                                         \n\
                                         ✅ Use named arguments instead:\n\
                                         @{}(arg1=\"value1\", arg2=\"value2\")\n\
                                         \n\
                                         Why? Named arguments prevent bugs when component signatures change\n\
                                         and make code self-documenting.",
                                        name_str, name_str, name_str
                                    )
                            };

                            return syn::Error::new(error_span, error_msg).to_compile_error();
                        } else if !exprs.is_empty() {
                            // All are named arguments - good!
                            Some(exprs)
                        } else {
                            // Empty args: treat as named args (Builder pattern) to support defaults
                            // This enforces #[component] even for empty arg calls
                            Some(exprs)
                        }
                    } else {
                        None
                    };

                    if let Some(exprs) = named_args {
                        let setters = exprs.iter().map(|e| {
                            if let syn::Expr::Assign(assign) = e {
                                let key = &assign.left;
                                let value = &assign.right;
                                quote! { .#key(#value) }
                            } else {
                                unreachable!()
                            }
                        });

                        let children_arg = if has_children {
                            quote! { , #children_code }
                        } else {
                            quote! {}
                        };

                        // Check if name is snake_case (starts with lowercase)
                        let name_str = quote!(#name).to_string();
                        let last_segment = name_str.split("::").last().unwrap_or("");
                        let is_snake_case = last_segment
                            .chars()
                            .next()
                            .is_some_and(|c| c.is_lowercase());

                        let module_name = if is_snake_case {
                            let new_name = format!("{}_component", name_str);
                            syn::parse_str::<syn::Path>(&new_name).unwrap_or(name.clone())
                        } else {
                            name.clone()
                        };

                        quote! {
                            azumi::Component::render(&#module_name::render(#module_name::Props::builder()
                                #(#setters)*
                                .build().expect("Failed to build props")
                                #children_arg), f)?;
                        }
                    } else {
                        let args_separator = if !args.is_empty() && has_children {
                            quote! { , }
                        } else {
                            quote! {}
                        };

                        quote! {
                            azumi::Component::render(&#name(#args #args_separator #children_code), f)?;
                        }
                    }
                }
                token_parser::Block::Component(comp_block) => {
                    let name = &comp_block.name;
                    quote! {
                        azumi::Component::render(&#name, f)?;
                    }
                }
                token_parser::Block::Let(let_block) => {
                    let pattern = &let_block.pattern;
                    let value = &let_block.value;
                    quote! {
                        let #pattern = #value;
                    }
                }
            },
        };
        stream.extend(chunk);
    }
    stream
}

#[cfg(test)]
mod token_parser_tests;
