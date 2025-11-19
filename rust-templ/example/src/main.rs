mod hello;
use templ::Component;

fn main() {
    let component = hello::hello("World");
    let mut output = String::new();
    // We need a formatter wrapper to capture output, but std::fmt::Formatter is not constructible directly.
    // We can use format! macro if we implement Display for the component wrapper?
    // Or we can just use a helper to render to string.
    
    // Let's add a helper to render to string in the runtime or just use a trick here.
    // Actually, our generated code returns `impl Component`.
    // `Component` has `render(&self, f: &mut Formatter)`.
    // We can implement Display for a wrapper struct.
    
    struct DisplayWrapper<'a, C: Component + ?Sized>(&'a C);
    impl<'a, C: Component + ?Sized> std::fmt::Display for DisplayWrapper<'a, C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.render(f)
        }
    }
    
    println!("{}", DisplayWrapper(&component));
}
