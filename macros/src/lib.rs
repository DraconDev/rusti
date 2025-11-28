// Force rebuild 3
mod component;

mod accessibility_validator;
mod css;
mod css_validator;
mod head;
mod html_structure_validator;
#[cfg(feature = "schema")]
mod schema;
mod test_spacing;
mod token_parser;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

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

    // 1. Collect CSS dependencies for hot-reloading
    let css_deps = collect_css_dependencies(&nodes);

    // 2. Generate HTML string construction code
    let html_construction = generate_nodes(&nodes);

    // 3. Generate bind validation checks
    let mut validation_checks = Vec::new();
    collect_bind_checks(&nodes, &mut validation_checks);

    let expanded = quote! {
        {
            // CSS dependency tracking (forces recompile when CSS changes)
            #(#css_deps)*

            // Validation block (compile-time only)
            const _: () = {
                #(#validation_checks)*
            };

            // Runtime HTML generation
            #html_construction
        }
    };

    TokenStream::from(expanded)
}

fn collect_bind_checks(nodes: &[token_parser::Node], checks: &mut Vec<proc_macro2::TokenStream>) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                // If this element has bind={Struct}, generate a check function
                if let Some(struct_path) = &elem.bind_struct {
                    let mut field_accesses = Vec::new();
                    collect_input_names(&elem.children, &mut field_accesses);

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

fn collect_input_names(nodes: &[token_parser::Node], accesses: &mut Vec<proc_macro2::TokenStream>) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                if elem.name == "input" || elem.name == "textarea" || elem.name == "select" {
                    // Find "name" attribute
                    for attr in &elem.attrs {
                        if attr.name == "name" {
                            if let token_parser::AttributeValue::Static(name_str) = &attr.value {
                                // Generate: let _ = &data.field;
                                // Handle nested paths: user.email -> &data.user.email
                                let parts: Vec<&str> = name_str.split('.').collect();
                                let fields: Vec<proc_macro2::Ident> = parts
                                    .iter()
                                    .map(|s| quote::format_ident!("{}", s))
                                    .collect();

                                accesses.push(quote! {
                                    let _ = &data.#(#fields).*;
                                });
                            }
                        }
                    }
                }
                // Recurse (e.g. input inside label or div)
                collect_input_names(&elem.children, accesses);
            }
            token_parser::Node::Fragment(frag) => {
                collect_input_names(&frag.children, accesses);
            }
            _ => {}
        }
    }
}

/// Collect CSS dependencies for hot-reloading
/// Generates `const _: &[u8] = include_bytes!(...)` for each CSS file
fn collect_css_dependencies(nodes: &[token_parser::Node]) -> Vec<proc_macro2::TokenStream> {
    let mut deps = Vec::new();
    let mut dep_counter = 0;
    collect_css_deps_recursive(nodes, &mut deps, &mut dep_counter);
    deps
}

fn collect_css_deps_recursive(
    nodes: &[token_parser::Node],
    deps: &mut Vec<proc_macro2::TokenStream>,
    counter: &mut usize,
) {
    for node in nodes {
        match node {
            token_parser::Node::Element(elem) => {
                // Look for <style src="..."> tags
                if elem.name == "style" {
                    if let Some(src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        if let token_parser::AttributeValue::Static(css_path) = &src_attr.value {
                            // Generate a unique constant name
                            let const_name = quote::format_ident!("_AZUMI_CSS_DEP_{}", counter);
                            *counter += 1;

                            // Resolve the CSS path for include_bytes!
                            // For now, use the existing CSS resolution from css_validator
                            let resolved_path =
                                crate::css_validator::resolve_css_file_path(css_path);

                            // Calculate relative path from source file to CSS file
                            // This is crucial for include_bytes! to work correctly
                            let relative_path = calculate_relative_path_for_include(&resolved_path);

                            deps.push(quote! {
                                #[allow(dead_code)]
                                const #const_name: &[u8] = include_bytes!(#relative_path);
                            });
                        }
                    }
                }
                // Recurse into children
                collect_css_deps_recursive(&elem.children, deps, counter);
            }
            token_parser::Node::Fragment(frag) => {
                collect_css_deps_recursive(&frag.children, deps, counter);
            }
            token_parser::Node::Block(block) => match block {
                token_parser::Block::If(if_block) => {
                    collect_css_deps_recursive(&if_block.then_branch, deps, counter);
                    if let Some(else_branch) = &if_block.else_branch {
                        collect_css_deps_recursive(else_branch, deps, counter);
                    }
                }
                token_parser::Block::For(for_block) => {
                    collect_css_deps_recursive(&for_block.body, deps, counter);
                }
                token_parser::Block::Match(match_block) => {
                    for arm in &match_block.arms {
                        collect_css_deps_recursive(&arm.body, deps, counter);
                    }
                }
                token_parser::Block::Call(call_block) => {
                    collect_css_deps_recursive(&call_block.children, deps, counter);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Calculate the relative path from the calling .rs file to the CSS file
/// This is needed for include_bytes! to work correctly
fn calculate_relative_path_for_include(absolute_css_path: &str) -> String {
    // For now, we'll use the absolute path directly
    // In a more sophisticated implementation, we would:
    // 1. Get the calling file location using Span::source_file()
    // 2. Calculate the relative path from that file to the CSS file
    // 3. Return the properly formatted relative path

    // Since we're in a proc macro, we need to be careful about paths
    // The absolute path from resolve_css_file_path should work with include_bytes!
    absolute_css_path.to_string()
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
                    if let Some(src_attr) = elem.attrs.iter().find(|a| a.name == "src") {
                        if let token_parser::AttributeValue::Static(path) = &src_attr.value {
                            // Use enhanced path resolution from validator
                            let file_path = crate::css_validator::resolve_css_file_path(path);
                            if let Ok(content) = std::fs::read_to_string(&file_path) {
                                // Check if this is a global.css file
                                if path.ends_with("global.css") {
                                    global_css.push_str(&content);
                                    global_css.push('\n');
                                } else {
                                    scoped_css.push_str(&content);
                                    scoped_css.push('\n');
                                }
                            }
                        }
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
    let style_validation_errors = validate_nodes(nodes, &valid_classes, &valid_ids);
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
) -> proc_macro2::TokenStream {
    use quote::quote_spanned;
    let mut errors = vec![];

    fn collect_errors_recursive(
        nodes: &[token_parser::Node],
        valid_classes: &std::collections::HashSet<String>,
        valid_ids: &std::collections::HashSet<String>,
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
                                        let msg = format!(
                                            "Class '{}' is used in HTML but not defined in CSS.",
                                            class_name
                                        );
                                        errors.push(quote_spanned! { attr.span =>
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
                                    errors.push(quote_spanned! { attr.span =>
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
                        if !exprs.is_empty()
                            && !exprs.iter().all(|e| matches!(e, syn::Expr::Assign(_)))
                        {
                            // Positional arguments detected - generate clear compile error
                            let name_str = quote!(#name).to_string();
                            let error_msg = format!(
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
                                );
                            return syn::Error::new_spanned(&call_block.name, error_msg)
                                .to_compile_error();
                        }

                        if !exprs.is_empty() {
                            // All are named arguments - good!
                            Some(exprs)
                        } else {
                            // Empty args: check if name starts with Uppercase (Component convention)
                            // If so, treat as named args (Builder pattern) to support defaults
                            let name_str = quote!(#name).to_string();
                            let last_segment = name_str.split("::").last().unwrap_or("");
                            if last_segment
                                .chars()
                                .next()
                                .is_some_and(|c| c.is_uppercase())
                            {
                                Some(exprs)
                            } else {
                                None
                            }
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
