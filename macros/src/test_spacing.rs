#[cfg(test)]
mod tests {
    use quote::quote;

    #[test]
    fn test_token_stream_spacing() {
        let ts = quote! {
            const app = {
                count: 0,
                increment() {
                    this.count++;
                }
            };
        };
        println!("JS Output:\n{}", ts.to_string());

        let css = quote! {
            .container {
                margin: 0;
                padding: 1px solid red;
            }
        };
        println!("CSS Output:\n{}", css.to_string());
        
        let complex_css = quote! {
            div.foo > span#bar {
                color: red;
            }
        };
        println!("Complex CSS Output:\n{}", complex_css.to_string());
    }
}
