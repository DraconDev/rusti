use crate::style::process_style_macro;
use quote::quote;
use syn::{parse_macro_input, FnArg, Item, ItemFn, Pat, PatType, Stmt};

pub fn expand_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let mut fn_block = input.block; // Mutable to modify statements
    let fn_output = &input.sig.output;

    // Process style! macro invocations
    let mut component_css = String::new();
    let mut new_stmts = Vec::new();

    for stmt in fn_block.stmts {
        if let Stmt::Macro(stmt_macro) = &stmt {
            if stmt_macro.mac.path.is_ident("style") {
                // Found style! macro
                let output = process_style_macro(stmt_macro.mac.tokens.clone());
                component_css.push_str(&output.css);

                // Replace with bindings
                // We need to parse the bindings TokenStream back into Stmts
                // This is a bit tricky, but we can wrap it in a block or just parse it
                let bindings_tokens = output.bindings;
                let bindings: syn::Block = syn::parse2(quote! { { #bindings_tokens } })
                    .expect("Failed to parse style bindings");
                // Extract statements from the block
                new_stmts.extend(bindings.stmts);
                continue;
            }
        }
        new_stmts.push(stmt);
    }
    fn_block.stmts = new_stmts;

    // Parse arguments into props

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

    let fn_generics = &input.sig.generics;
    let (impl_generics, ty_generics, where_clause) = fn_generics.split_for_impl();

    // Generate the output
    let render_fn = if has_children {
        let children_ty = children_type.as_ref().unwrap();
        quote! {
            pub fn render #impl_generics (props: Props #ty_generics, children: #children_ty) #fn_output #where_clause {
                #(#props_init)*
                #fn_block
            }
        }
    } else {
        quote! {
            pub fn render #impl_generics (props: Props #ty_generics) #fn_output #where_clause {
                #(#props_init)*

                // Inject styles if any
                let __azumi_styles = if !#component_css.is_empty() {
                    azumi::html! {
                        <style data-azumi-internal="true">
                            #component_css
                        </style>
                    }
                } else {
                    azumi::html! {}
                };

                // We need to inject the styles into the output
                // Assuming the block returns an html! macro result or similar
                // We can wrap the result in a fragment with the styles

                let __azumi_content = { #fn_block };

                azumi::html! {
                    <>
                        {__azumi_styles}
                        {__azumi_content}
                    </>
                }
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
        let wrapper = if has_children || !props_fields.is_empty() {
            // Don't generate wrapper if props/children are required
            quote! {}
        } else {
            quote! {
                #fn_vis fn #fn_name #fn_generics () -> impl azumi::Component #where_clause {
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

            // #[derive(Debug)] // Removed to allow non-Debug props
            pub struct Props #fn_generics #where_clause {
                #(#props_fields),*
            }

            pub struct PropsBuilder #fn_generics #where_clause {
                #(#builder_fields),*
            }

            impl #impl_generics Props #ty_generics #where_clause {
                pub fn builder() -> PropsBuilder #ty_generics {
                    PropsBuilder {
                        #(#builder_init),*
                    }
                }
            }

            impl #impl_generics PropsBuilder #ty_generics #where_clause {
                #(#builder_setters)*

                pub fn build(self) -> Result<Props #ty_generics, &'static str> {
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
