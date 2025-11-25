use rusti::rusti;

pub fn working_scripts_demo() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
        <body>
            <h1>Debug Script Injection</h1>
            <script>
                @let test_string = "Hello, Rusti!";
                @let test_number = 42;

                const str = @{ test_string };
                console.log("String:", str);

                const num = @{ test_number };
                console.log("Number:", num);
            </script>
        </body>
        </html>
    }
}
