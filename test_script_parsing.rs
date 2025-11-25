use rusti::rusti;

fn main() {
    let s = "test";
    let component = rusti! {
        <script>
            const x = @{ s };
        </script>
    };

    let output = rusti::render_to_string(&component);
    println!("{}", output);
}
