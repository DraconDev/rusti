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

    for arg in &input.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            if let Pat::Ident(pat_ident) = &**pat {
                let ident = &pat_ident.ident;
                props_fields.push(quote! {
                    pub #ident: #ty
                });
                props_init.push(quote! {
                    let #ident = props.#ident;
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

            pub fn render(props: Props) #fn_output {
                #(#props_init)*
                #fn_block
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
