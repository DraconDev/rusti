use rusti::rusti;

pub fn simple_script_test() -> impl rusti::Component {
    let my_number = 42;

    rusti! {
        <!DOCTYPE html>
        <html>
        <head>
            <title>Simple Script Test</title>
        </head>
        <body>
            <h1>Simple Script Injection Test</h1>
            <div id="output"></div>

            <script>
                @let my_string = "Hello";
                const num = @{ my_number };
                const name = @{ format!("{}", my_string.to_string()) };

                document.getElementById("output").innerHTML =
                    "Number: " + num + "<br>Name: " + name;

                console.log("num =", num);
                console.log("name =", name);
            </script>
        </body>
        </html>
    }
}
