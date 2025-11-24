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
        for tt in ts.clone() {
            output.push_str(&tt.to_string());
        }
        println!("Original JS:      {}", js_source);
        println!("Iterated JS:      {}", output);
        println!("TokenStream JS:   {}", ts.to_string());

        // CSS Example
        let css_source = ".container { margin: 0; padding: 1px solid red; }";
        let ts_css = TokenStream::from_str(css_source).unwrap();

        let mut css_output = String::new();
        for tt in ts_css.clone() {
            css_output.push_str(&tt.to_string());
        }
        println!("Original CSS:     {}", css_source);
        println!("Iterated CSS:     {}", css_output);
        println!("TokenStream CSS:  {}", ts_css.to_string());

        // Complex CSS
        let complex_css = "div.foo > div # bar { color: red; }";
        let ts_complex = TokenStream::from_str(complex_css).unwrap();
        println!("Original Complex: {}", complex_css);
        println!("TokenStream Cplx: {}", ts_complex.to_string());
    }
}
