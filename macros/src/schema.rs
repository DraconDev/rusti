use heck::ToLowerCamelCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Lit, Meta, NestedMeta};

pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    // Extract @type from struct-level #[schema(type = "...")] attribute or use struct name
    let schema_type = extract_schema_type(&input.attrs, name.to_string());

    // Only work with named structs
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "Schema can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(name, "Schema can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    // Process each field
    let mut field_serializations = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        // Check for #[schema(skip)]
        if should_skip_field(&field.attrs) {
            continue;
        }

        // Get JSON key name (either from #[schema(name = "...")] or camelCase conversion)
        let json_key = extract_field_name(&field.attrs)
            .unwrap_or_else(|| field_name.to_string().to_lower_camel_case());

        // Generate serialization code based on type
        let serialization = generate_field_serialization(field_name, field_type, &json_key);

        field_serializations.push(serialization);
    }

    // Generate the implementation
    let expanded = quote! {
        impl azumi::Schema for #name {
            fn to_schema_script(&self) -> String {
                let json_value = self.to_schema_json_value();
                let json_string = serde_json::to_string_pretty(&json_value)
                    .expect("Failed to serialize schema to JSON");
                format!(
                    "<script type=\"application/ld+json\">\n{}\n</script>",
                    json_string
                )
            }

            fn to_schema_json_value(&self) -> serde_json::Value {
                let mut map = serde_json::Map::new();
                map.insert(
                    "@context".to_string(),
                    serde_json::Value::String("https://schema.org".to_string())
                );
                map.insert(
                    "@type".to_string(),
                    serde_json::Value::String(#schema_type.to_string())
                );

                #(#field_serializations)*

                serde_json::Value::Object(map)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Extract schema type from #[schema(type = "...")] attribute
fn extract_schema_type(attrs: &[syn::Attribute], default: String) -> String {
    for attr in attrs {
        if !attr.path.is_ident("schema") {
            continue;
        }

        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            for nested in meta_list.nested {
                if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                    if nv.path.is_ident("type") {
                        if let Lit::Str(lit_str) = nv.lit {
                            return lit_str.value();
                        }
                    }
                }
            }
        }
    }
    default
}

/// Check if field has #[schema(skip)]
fn should_skip_field(attrs: &[syn::Attribute]) -> bool {
    for attr in attrs {
        if !attr.path.is_ident("schema") {
            continue;
        }

        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            for nested in meta_list.nested {
                if let NestedMeta::Meta(Meta::Path(path)) = nested {
                    if path.is_ident("skip") {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Extract custom field name from #[schema(name = "...")]
fn extract_field_name(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path.is_ident("schema") {
            continue;
        }

        if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
            for nested in meta_list.nested {
                if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                    if nv.path.is_ident("name") {
                        if let Lit::Str(lit_str) = nv.lit {
                            return Some(lit_str.value());
                        }
                    }
                }
            }
        }
    }
    None
}

/// Generate field serialization code based on the type
fn generate_field_serialization(
    field_name: &syn::Ident,
    field_type: &syn::Type,
    json_key: &str,
) -> proc_macro2::TokenStream {
    let type_str = quote!(#field_type).to_string();

    // Handle Option<T>
    if type_str.contains("Option") {
        return quote! {
            if let Some(ref value) = self.#field_name {
                map.insert(
                    #json_key.to_string(),
                    serialize_value(value)
                );
            }
        };
    }

    // Handle Vec<T>
    if type_str.contains("Vec") {
        return quote! {
            {
                let array: Vec<serde_json::Value> = self.#field_name
                    .iter()
                    .map(|item| serialize_value(item))
                    .collect();
                map.insert(
                    #json_key.to_string(),
                    serde_json::Value::Array(array)
                );
            }
        };
    }

    // Default: direct serialization
    quote! {
        map.insert(
            #json_key.to_string(),
            serialize_value(&self.#field_name)
        );
    }
}

/// Helper function to serialize values (will be generated inline)
/// This is a placeholder - the actual implementation needs to handle:
/// - Nested structs that implement Schema
/// - Primitive types
/// - Generic serialization fallback
#[allow(dead_code)]
fn serialize_value_placeholder() {
    // This function exists to demonstrate the concept
    // The actual code generation will inline this logic
}
