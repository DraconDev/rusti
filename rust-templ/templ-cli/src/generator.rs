use crate::parser::{Node, TemplDefinition};
use std::fmt::Write;

pub fn generate(templ: &TemplDefinition) -> Result<String, std::fmt::Error> {
    let mut output = String::new();

    // Generate struct to hold arguments
    // For MVP, we'll just generate a function that writes directly to a formatter
    // wrapped in a struct that implements Display.
    
    // Actually, let's generate a struct for the component
    let struct_name = format!("{}Component", templ.name);
    
    // We need to parse args to generate struct fields, but for now let's just 
    // assume args is a string like "name: &str, age: i32" and we can't easily 
    // parse that without a full Rust parser.
    // 
    // Alternative: Generate a function that returns an impl Display, capturing args in a closure?
    // Closures can't easily implement Display if they capture references without move, 
    // and we want to support references.
    //
    // Let's try the closure approach first as it's simpler for the generator.
    // pub fn hello(name: &str) -> impl std::fmt::Display + '_ {
    //    templ::RenderFn::new(move |f| { ... })
    // }
    // We need a helper in the runtime for this.

    writeln!(output, "pub fn {}({}) -> impl templ::Component + '_ {{", templ.name, templ.args)?;
    writeln!(output, "    templ::from_fn(move |f| {{")?;
    
    for child in &templ.children {
        generate_node(&mut output, child)?;
    }
    
    writeln!(output, "        Ok(())")?;
    writeln!(output, "    }})")?;
    writeln!(output, "}}")?;

    Ok(output)
}

fn generate_node(output: &mut String, node: &Node) -> Result<(), std::fmt::Error> {
    match node {
        Node::Element { name, attrs: _, children } => {
            writeln!(output, "        write!(f, \"<{}>\")?;", name)?;
            for child in children {
                generate_node(output, child)?;
            }
            writeln!(output, "        write!(f, \"</{}>\")?;", name)?;
        }
        Node::Text(text) => {
            writeln!(output, "        write!(f, \"{}\")?;", text.trim())?;
        }
        Node::Expression(expr) => {
            // Escape? For now, assume user handles it or we just write it.
            // In real templ, we'd escape HTML.
            writeln!(output, "        write!(f, \"{{}}\", {})?;", expr)?;
        }
        Node::Call { name, args, children } => {
            // @Component(args) { children }
            // This is tricky. The component function returns something implementing Component.
            // We need to call .render(f) on it.
            // And if it has children, we need to pass them?
            // MVP: Ignore children for calls for now, or assume the component takes a `children` arg?
            // Go templ passes children as a trailing closure.
            // Let's assume the component function takes args.
            writeln!(output, "        {}({}).render(f)?;", name, args)?;
            
            if !children.is_empty() {
                 // TODO: Handle children blocks in calls
            }
        }
    }
    Ok(())
}
