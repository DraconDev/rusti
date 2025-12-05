use azumi::prelude::*;

/// Lesson 10: Live Components with Auto-Detection

#[azumi::live]
pub struct LikeButton {
    pub liked: bool,
    pub count: i32,
}

#[azumi::live_impl(component = "like_button_view")]
impl LikeButton {
    pub fn toggle(&mut self) {
        self.liked = !self.liked;
        if self.liked {
            self.count += 1;
        } else {
            self.count -= 1;
        }
    }
}

/// Like button component
#[azumi::component]
pub fn like_button_view<'a>(state: &'a LikeButton) -> impl Component + 'a {
    html! {
        <style>
            .like_container {
                display: "inline-flex";
                align-items: "center";
                gap: "0.5rem";
            }
            .like_btn {
                padding: "0.75rem 1.5rem";
                font-size: "1.5rem";
                border: "2px solid #e91e63";
                border-radius: "50px";
                background: "transparent";
                cursor: "pointer";
            }
            .like_count {
                font-size: "1.2rem";
                font-weight: "bold";
                color: "#e91e63";
            }
        </style>
        <div class={like_container}>
            <button class={like_btn} on:click={state.toggle}>
                {if state.liked { "❤️" } else { "🤍" }}
            </button>
            <span class={like_count} data-bind="count">{state.count}</span>
        </div>
    }
}

// Handler for Axum
pub async fn lesson10_handler() -> axum::response::Html<String> {
    let like_state = LikeButton {
        liked: false,
        count: 42,
    };

    use like_button_view_component::Props as LikeProps;
    let like_html = azumi::render_to_string(&like_button_view_component::render(
        LikeProps::builder()
            .state(&like_state)
            .build()
            .expect("props"),
    ));

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Lesson 10: Live Components Auto-Detection</title>
    <style>
        body {{ 
            font-family: system-ui, sans-serif; 
            margin: 0;
            padding: 2rem;
            background: #fafafa;
        }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .header {{ text-align: center; margin-bottom: 2rem; }}
        .main_title {{ font-size: 2rem; color: #333; }}
        .subtitle {{ color: #666; }}
        .demo_card {{ 
            padding: 1.5rem; 
            border: 1px solid #eee; 
            border-radius: 12px;
            background: white;
            margin: 2rem 0;
        }}
        .demo_title {{ color: #2196f3; margin-bottom: 1rem; }}
        .explanation {{
            background: #e3f2fd;
            padding: 1.5rem;
            border-radius: 8px;
            margin: 1rem 0;
        }}
        .code_block {{
            background: #1e1e2e;
            color: #cdd6f4;
            padding: 1rem;
            border-radius: 8px;
            font-family: monospace;
            font-size: 0.85rem;
            overflow-x: auto;
            margin: 1rem 0;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header class="header">
            <h1 class="main_title">Lesson 10: Auto-Detection</h1>
            <p class="subtitle">Components automatically detect live state</p>
        </header>
        
        <div class="explanation">
            <h3>🔍 Auto-Detection</h3>
            <p>When first parameter is <code>state: &amp;T</code>, component detects live mode:</p>
            <div class="code_block">
                #[azumi::component]<br/>
                fn my_view&lt;'a&gt;(state: &amp;'a MyState) -&gt; impl Component + 'a {{<br/>
                &nbsp;&nbsp;html! {{<br/>
                &nbsp;&nbsp;&nbsp;&nbsp;&lt;button on:click={{state.action}}&gt;"Click"&lt;/button&gt;<br/>
                &nbsp;&nbsp;}}<br/>
                }}
            </div>
        </div>
        
        <div class="demo_card">
            <h3 class="demo_title">❤️ Like Button</h3>
            <p>Click to toggle like state - instant UI update!</p>
            {}
        </div>
    </div>
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        like_html
    );

    axum::response::Html(html)
}
