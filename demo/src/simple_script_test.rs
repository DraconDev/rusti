use rusti::rusti;

pub fn simple_script_test() -> impl rusti::Component {
    let my_number = 42;
    // let my_string = "Hello";

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
                const name = @{ my_string };

                document.getElementById("output").innerHTML =
                    "Number: " + num + "<br>String: " + name;

                console.log("num =", num);
                console.log("name =", name);
            </script>
        </body>
        </html>
    }
}
