use rusti::rusti;

pub fn simple_script_test() -> impl rusti::Component {
    let my_number = 42;
    // let my_string = "Hello";

    let my_array = vec![1, 2, 3, 4, 5];

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
                @let my_string = "Hello".to_string();
                const num = @{ my_number };
                const str = @{ format!("\"{}\"", my_string) };
                // const str = @{ my_string }; doesn't work
                // const str = @{ my_string.to_string() }; doesn't work
                const arr = @{ format!("{:?}", my_array) }; // this works too
                // const arr = @{ my_array }; doesn't work, even errors in compiler
                // const arr = new Array (@{ my_array }); doesn't work

                document.getElementById("output").innerHTML =
                    "Number: " + num + "<br>String: " + str;


                document.getElementById("output").innerHTML += "<br>Array: " + arr; // this works too, albeit just: Array: 1,2,3,4,5

                @for item in &my_array {
                    document.getElementById("output").innerHTML += "<br>" + @{ item };
                } // this works too

                @let my_string_array = vec!["Apple", "Banana", "Cherry"];

                // @for item in &my_string_array {
                //     document.getElementById("output").innerHTML += "<br>" + @{ item };
                // } // this doesn't work, simple-script-test:1 Uncaught ReferenceError: Apple is not defined

                @for item in &my_string_array {
                    document.getElementById("output").innerHTML += "<br>" + @{ format!("\"{}\"", item) };
                } // this works too



                console.log("num =", num);
                console.log("str =", str);
                console.log("arr =", arr);
            </script>
        </body>
        </html>
    }
}
