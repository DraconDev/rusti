use rusti::rusti;

fn main() {
    let html = rusti::render_to_string(&scoped_css_test());
    println!("{}", html);
}

fn scoped_css_test() -> impl rusti::Component {
    rusti! {
        <div class="component">
            <style>
                .button { background-color: red; color: white; }
                h2 { color: red; }
            </style>
            <h2>Component 1</h2>
            <button class="button">Red Button</button>
        </div>
    }
}
