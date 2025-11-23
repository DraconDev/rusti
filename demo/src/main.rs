        .route("/forms", get(forms_demo))
        // Basic page demo - assuming a handler exists within basic_page module
        // If basic_page does not contain a handler function named 'basic_page_handler',
        // this line will cause a compilation error and should be removed or adjusted.
        // For example: `use basic_page::basic_page_handler;` and then `.route("/basic", get(basic_page_handler))`
