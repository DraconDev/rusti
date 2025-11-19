use rusti::rusti;

rusti! {
    fn hello(name: &str) {
        <div>
            <h1>Hello, { name }!</h1>
            <p>Welcome to Rusti Macros.</p>
        </div>
    }
}

fn main() {
    let component = hello("World");
    
    struct DisplayWrapper<'a, C: rusti::Component + ?Sized>(&'a C);
    impl<'a, C: rusti::Component + ?Sized> std::fmt::Display for DisplayWrapper<'a, C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.render(f)
        }
    }
    
    println!("{}", DisplayWrapper(&component));
}
