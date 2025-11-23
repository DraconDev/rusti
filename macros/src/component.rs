use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, PatType};

pub fn expand_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_block = &input.block;
    let fn_output = &input.sig.output;

    // Parse arguments into props
    let mut props_fields = Vec::new();
    let mut props_init = Vec::new();
    let mut builder_fields = Vec::new();
    let mut builder_init = Vec::new();
    let mut builder_setters = Vec::new();
    let mut build_logic = Vec::new();

    for arg in &input.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, attrs, .. }) = arg {
            if let Pat::Ident(pat_ident) = &**pat {
                let ident = &pat_ident.ident;
                
                // Check for #[prop(default = ...)]
                let mut default_value = None;
                for attr in attrs {
                    if attr.path.is_ident("prop") {
                        if let Ok(syn::Meta::List(meta_list)) = attr.parse_meta() {
                            for nested in meta_list.nested {
                                if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = nested {
                                    if nv.path.is_ident("default") {
                                        if let syn::Lit::Str(lit_str) = nv.lit {
                                            default_value = Some(lit_str.parse::<syn::Expr>().expect("Invalid default value expression"));
                                        } else {
                                            // Handle other literal types or error
                                            // For simplicity, let's assume string containing expr for now, or just raw tokens?
                                            // Actually, nv.lit is a Lit. If user writes default = "foo", it's a string.
                                            // If user writes default = 10, it's an int.
                                            // But standard attribute syntax default = expr is not valid Rust syntax for key-value unless it's a literal.
                                            // So users might have to write default = "expr".
                                            // Or we can parse `default = ...` if we use custom parsing.
                                            // Let's stick to default = "expr" string for now as it's easier with standard syn::Meta.
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                props_fields.push(quote! {
                    pub #ident: #ty
                });
                props_init.push(quote! {
                    let #ident = props.#ident;
                });

                // Builder logic
                if let Some(default) = default_value {
                    builder_fields.push(quote! {
                        #ident: Option<#ty>
                    });
                    builder_init.push(quote! {
                        #ident: None
                    });
                    build_logic.push(quote! {
                        let #ident = self.#ident.unwrap_or_else(|| #default);
                    });
                } else {
                    builder_fields.push(quote! {
                        #ident: Option<#ty>
                    });
                    builder_init.push(quote! {
                        #ident: None
                    });
                    build_logic.push(quote! {
                        let #ident = self.#ident.ok_or("Missing required field: #ident")?;
                    });
                }

                builder_setters.push(quote! {
                    pub fn #ident(mut self, value: #ty) -> Self {
                        self.#ident = Some(value);
                        self
                    }
                });
            }
        }
    }

    // Generate the output
    // We use a module to namespace the Props struct
    let expanded = quote! {
        #[allow(non_snake_case)]
        #fn_vis mod #fn_name {
            use super::*;

            #[derive(Debug)]
            pub struct Props {
                #(#props_fields),*
            }

            pub struct PropsBuilder {
                #(#builder_fields),*
            }

            impl Props {
                pub fn builder() -> PropsBuilder {
                    PropsBuilder {
                        #(#builder_init),*
                    }
                }
            }

            impl PropsBuilder {
                #(#builder_setters)*

                pub fn build(self) -> Result<Props, &'static str> {
                    #(#build_logic)*
                    Ok(Props {
                        #(#props_fields: #props_fields),* // This is wrong, need ident
                    })
                }
            }
            
            // We need to fix the build method construction
        }
    };
    
    // Wait, I need to extract idents for the final struct construction
    // Let's redo the loop to capture idents cleanly.
    
    // ... (re-implementing cleanly below)
