use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident, Token};

struct HeadArgs {
    title: Option<Expr>,
    description: Option<Expr>,
    image: Option<Expr>,
    url: Option<Expr>,
    type_: Option<Expr>,
}

impl Parse for HeadArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = HeadArgs {
            title: None,
            description: None,
            image: None,
            url: None,
            type_: None,
        };

        while !input.is_empty() {
            let key_str = if input.peek(Token![type]) {
                input.parse::<Token![type]>()?;
                "type".to_string()
            } else {
                input.parse::<Ident>()?.to_string()
            };

            input.parse::<Token![:]>()?;
            let value: Expr = input.parse()?;

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }

            match key_str.as_str() {
                "title" => args.title = Some(value),
                "description" => args.description = Some(value),
                "image" => args.image = Some(value),
                "url" => args.url = Some(value),
                "type" => args.type_ = Some(value),
                _ => {
                    return Err(syn::Error::new(
                        input.span(),
                        "Unknown key. Supported keys: title, description, image, url, type",
                    ))
                }
            }
        }

        if args.title.is_none() {
            return Err(input.error("Missing required field: title"));
        }
        if args.description.is_none() {
            return Err(input.error("Missing required field: description"));
        }

        Ok(args)
    }
}

pub fn expand_head(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as HeadArgs);

    let title = args.title.unwrap();
    let description = args.description.unwrap();

    let image_meta = if let Some(img) = args.image {
        quote! {
            format!(
                "<meta property=\"og:image\" content=\"{}\">\n<meta name=\"twitter:image\" content=\"{}\">\n<meta name=\"twitter:card\" content=\"summary_large_image\">",
                #img, #img
            )
        }
    } else {
        quote! {
            "<meta name=\"twitter:card\" content=\"summary\">".to_string()
        }
    };

    let url_meta = if let Some(url) = args.url {
        quote! {
            format!("<meta property=\"og:url\" content=\"{}\">", #url)
        }
    } else {
        quote! { String::new() }
    };

    let type_meta = if let Some(t) = args.type_ {
        quote! {
            format!("<meta property=\"og:type\" content=\"{}\">", #t)
        }
    } else {
        quote! {
            "<meta property=\"og:type\" content=\"website\">".to_string()
        }
    };

    let expanded = quote! {
        {
            let title = #title;
            let description = #description;
            format!(
                "<title>{}</title>\n\
                <meta name=\"description\" content=\"{}\">\n\
                <meta property=\"og:title\" content=\"{}\">\n\
                <meta property=\"og:description\" content=\"{}\">\n\
                {}\n\
                {}\n\
                {}\n\
                <meta name=\"twitter:title\" content=\"{}\">\n\
                <meta name=\"twitter:description\" content=\"{}\">",
                title, description,
                title, description,
                #type_meta,
                #url_meta,
                #image_meta,
                title, description
            )
        }
    };

    TokenStream::from(expanded)
}
