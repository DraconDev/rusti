i did investivate the css dead code situation and there are extensions that can help cleaning up unused css but they are mostly for web projects where you have html/js files to analyze together with css, in our case we only have css files so its not as straightforward, i will look into it more and see if we can find a solution that works for us, but most of all our solution doesn't even clean up unused files, only unused rules within files, so an extension is the solution here much like css peak 

also type check the css files we have and if any are unused then we mark what imports then with a warning


           token_parser::Node::Expression(expr) => {
                let content = &expr.content;
                println!(
                    "Generating expression: {:?} in context: {:?}",
                    content, ctx.mode
                );
                match ctx.mode {
                    Context::Script => {
                        // In script tags, use azumi::js() to safely inject values (Debug formatting)
                        println!("  -> Script context, using azumi::js");
                        quote! { write!(f, "{}", azumi::js(&(#content)))?; }
                    }
                    Context::Style => {
                        // In style tags, use Display (raw text)
                        quote! { write!(f, "{}", #content)?; }
                    }
                    Context::Normal => {
                        // In normal HTML, use Escaped (HTML escaping)
                        quote! { write!(f, "{}", azumi::Escaped(&(#content)))?; }
                    }
                }
            }

            ? clean up this code related to script tags