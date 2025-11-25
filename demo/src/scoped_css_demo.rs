use axum::response::{Html, IntoResponse};
use rusti::rusti;

pub fn scoped_css_demo() -> impl rusti::Component {
    rusti! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="UTF-8" />
                <title>Scoped CSS Demo</title>
            </head>
            <body>
                <h1>Scoped CSS Testing</h1>

                <div class="component">
                    <style>
                        .button {
                            background-color: #ff0000; /* Red */
                            color: white;
                            padding: 10px 20px;
                            border: none;
                            border-radius: 5px;
                            cursor: pointer;
                        }
                        h2 {
                            color: #ff0000;
                        }
                    </style>
                    <h2>Component 1 - Red Theme</h2>
                    <button class="button">Red Button</button>
                </div>

                <hr />

                <div class="component">
                    <style>
                        .button {
                            background-color: #0000ff; /* Blue */
                            color: white;
                            padding: 10px 20px;
                            border: none;
                            border-radius: 5px;
                            cursor: pointer;
                        }
                        h2 {
                            color: #0000ff;
                        }
                    </style>
                    <h2>Component 2 - Blue Theme</h2>
                    <button class="button">Blue Button</button>
                </div>

                <hr />

                <div class="component">
                    <style>
                        .button {
                            background-color: #00ff00; /* Green */
                            color: black;
                            padding: 10px 20px;
                            border: none;
                            border-radius: 5px;
                            cursor: pointer;
                        }
                        h2 {
                            color: #00ff00;
                        }
                    </style>
                    <h2>Component 3 - Green Theme</h2>
                    <button class="button">Green Button</button>
                </div>

                <hr />

                <p>This button below has class="button" but NO scoped styles apply to it:</p>
                <button class="button">Unstyled Button (proves isolation!)</button>
            </body>
        </html>
    }
}

pub async fn scoped_css_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&scoped_css_demo()))
}
