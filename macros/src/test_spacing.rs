#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use std::str::FromStr;

    #[test]
    fn test_token_stream_spacing() {
        // JS Example
        let js_source = "const app = { count: 0, increment() { this.count++; } };";
        let ts = TokenStream::from_str(js_source).unwrap();

        let mut output = String::new();
        for tt in ts {
            output.push_str(&tt.to_string());
        }
        println!("Original JS: {}", js_source);
        println!("Parsed JS:   {}", output);

        assert_eq!(
            output,
            js_source.replace(" ", ""),
            "Current behavior strips all spaces"
        );

        // CSS Example
        let css_source = ".container { margin: 0; padding: 1px solid red; }";
        let ts_css = TokenStream::from_str(css_source).unwrap();

        let mut css_output = String::new();
        for tt in ts_css {
            css_output.push_str(&tt.to_string());
        }
        println!("Original CSS: {}", css_source);
        println!("Parsed CSS:   {}", css_output);
    }
}
