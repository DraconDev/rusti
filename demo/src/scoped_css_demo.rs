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

            <!-- Component 1: Red Buttons -->
            <div class="component">
                <style>
                    .button {
                        background-color: #ff0000;
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

            <!-- Component 2: Blue Buttons (same classes, different styles) -->
            <div class="component">
                <style>
                    .button {
                        background-color: #0000ff;
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

            <!-- Component 3: Global styles with :global() -->
            <div class="component">
                <style>
                    .button {
                        background-color: #00ff00;
                        color: black;
                        padding: 10px 20px;
                        border: none;
                        border-radius: 5px;
                        cursor: pointer;
                    }

                    :global(.global-text) {
                        color: purple;
                        font-weight: bold;
                    }
                </style>
                <h2>Component 3 - Green Theme + Global</h2>
                <button class="button">Green Button</button>
                <p class="global-text">This uses :global() so it applies everywhere</p>
            </div>

            <hr />

            <!-- Test that global styles reach here -->
            <p class="global-text">This should be purple and bold (from :global() above)</p>
            <button class="button">This button should have NO scoped styles</button>
        </body>
        </html>
    }
}
