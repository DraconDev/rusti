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
    let mut struct_fields = Vec::new();
    let mut has_children = false;
    let mut children_type = None;

    for arg in &input.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, attrs, .. }) = arg {
            if let Pat::Ident(pat_ident) = &**pat {
                let ident = &pat_ident.ident;

                // Check if this is the special "children" parameter
                if ident == "children" {
                    has_children = true;
                    children_type = Some(ty.clone());
                    continue; // Don't add to Props
                }

                // Check for #[prop(default = ...)]
                let mut default_value = None;
                for attr in attrs {
                    if attr.path().is_ident("prop") {
                        let _ = attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("default") {
                                let value = meta.value()?;
                                let lit_str: syn::LitStr = value.parse()?;
                                default_value = Some(
                                    lit_str
                                        .parse::<syn::Expr>()
                                        .expect("Invalid default value expression"),
                                );
                            }
                            Ok(())
                        });
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
                        let #ident = self.#ident.ok_or(concat!("Missing required field: ", stringify!(#ident)))?;
                    });
                }

                builder_setters.push(quote! {
                    pub fn #ident(mut self, value: #ty) -> Self {
                        self.#ident = Some(value);
                        self
                    }
                });

                struct_fields.push(ident);
            }
        }
    }

    // Generate the output
    let render_fn = if has_children {
        let children_ty = children_type.as_ref().unwrap();
        quote! {
            pub fn render(props: Props, children: #children_ty) #fn_output {
                #(#props_init)*
                #fn_block
            }
        }
    } else {
        quote! {
            pub fn render(props: Props) #fn_output {
                #(#props_init)*
                #fn_block
            }
        }
    };

    // Check if function name is snake_case
    let name_str = fn_name.to_string();
    let is_snake_case = name_str.chars().next().is_some_and(|c| c.is_lowercase());

    let (mod_name, wrapper_fn) = if is_snake_case {
        let mod_ident = syn::Ident::new(&format!("{}_component", name_str), fn_name.span());

        // Generate wrapper function for direct calls (e.g. @snake_case())
        // Note: This only works for components with no required props or children
        let wrapper = if has_children {
            // For now, don't generate wrapper for components with children to avoid complexity
            quote! {}
        } else {
            quote! {
                #fn_vis fn #fn_name() -> impl azumi::Component {
                    #mod_ident::render(#mod_ident::Props::builder().build().expect("Missing required props in wrapper call"))
                }
            }
        };

        (mod_ident, wrapper)
    } else {
        (fn_name.clone(), quote! {})
    };

    let expanded = quote! {
        #[allow(non_snake_case)]
        #fn_vis mod #mod_name {
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
                        #(#struct_fields: #struct_fields),*
                    })
                }
            }

            #render_fn
        }

        #wrapper_fn
    };

    proc_macro::TokenStream::from(expanded)
}
