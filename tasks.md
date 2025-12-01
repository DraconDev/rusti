### Tasks

## TODO

- what about --var that we added before? do we still need it?

- how to handle global?
  - we would also need a global tag for the style 

- make the lessons 

what about this 

#[azumi::component]
pub fn basic_template() -> impl azumi::Component {
    html! {
        <style> 
            .basic_template {
                padding: "20px";
                background-color: "#f0f0f0";
            }
            .basic_h1 { color: "#333"; }
            .basic_h2 { color: "#666"; }
            .basic_p { font-size: "14px"; }
        </style>
        <div class={basic_template} >
            <h1 class={basic_h1}>"Hello, World!"</h1>
            <h2 class={basic_h2}>"Welcome to Azumi"</h2>
            <p class={basic_p}>"This is a simple styled template"</p>
        </div>
    }
}

vs

#[azumi::component]
pub fn basic_template() -> impl azumi::Component {
    html! {
        <style> 
            .basic_template {
                padding: "20px";
                background-color: "#f0f0f0";
            }
            .basic_h1 { color: "#333"; }
            .basic_h2 { color: "#666"; }
            .basic_p { font-size: "14px"; }
        </style>
        <div basic_template>
            <h1 basic_h1>"Hello, World!"</h1>
            <h2 basic_h2>"Welcome to Azumi"</h2>
            <p basic_p>"This is a simple styled template"</p>
        </div>
    }
}


## Done

- snake case for css?
- list-style should be list_style