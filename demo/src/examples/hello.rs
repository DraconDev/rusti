// Simple "Hello World" example showing basic Rusti usage
use rusti::rusti;

pub fn greeting(name: &str) -> impl rusti::Component + '_ {
    rusti! {
        <div class="card">
            <h1>Hello, { name }!</h1>
            <p>Welcome to Rusti - a type-safe HTML templating library for Rust.</p>
        </div>
    }
}
