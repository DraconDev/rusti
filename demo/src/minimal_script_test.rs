use rusti::rusti;

pub fn minimal_test() -> impl rusti::Component {
    let test_var = 42;

    rusti! {
        <!DOCTYPE html>
        <html>
        <body>
            <script>
                const myNumber = @{ test_var };
                console.log("Number is:", myNumber);
            </script>
        </body>
        </html>
    }
}

pub fn minimal_test_handler() -> impl rusti::Component {
    minimal_test()
}
