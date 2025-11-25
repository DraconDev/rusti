use rusti::rusti;

pub fn working_scripts_demo() -> impl rusti::Component {
    // Test data for various scenarios
    let test_string = "Hello, Rusti!";
    let test_number = 42;
    let test_float = 3.14159;
    let test_bool = true;
    let items = vec!["Apple", "Banana", "Cherry", "Date", "Elderberry"];
    let numbers = vec![1, 2, 3, 4, 5];
    let user_data = vec![("Alice", 25), ("Bob", 30), ("Charlie", 35)];
    let status = "success";
    let debug_mode = true;
    let nested_array = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    rusti! {
        <!DOCTYPE html>
        <html>
        <body>
            <h1>Debug Script Injection</h1>
            <script>
                const str = @{ test_string };
                console.log("String:", str);

                const num = @{ test_number };
                console.log("Number:", num);
            </script>
        </body>
        </html>
    }
}
