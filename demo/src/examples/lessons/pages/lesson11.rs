use azumi::prelude::*;

/// Lesson 11: Declarative Event Binding (on:event)

#[azumi::live]
pub struct CartItem {
    pub quantity: i32,
}

#[azumi::live_impl]
impl CartItem {
    pub fn increment(&mut self) {
        self.quantity += 1;
    }

    pub fn decrement(&mut self) {
        if self.quantity > 0 {
            self.quantity -= 1;
        }
    }
}

/// Cart item component
#[azumi::component]
pub fn cart_item_view<'a>(state: &'a CartItem, name: &'a str, price: f64) -> impl Component + 'a {
    html! {
        <style>
            .cart_item {
                display: "flex";
                align-items: "center";
                gap: "1rem";
                padding: "1rem";
                border: "1px solid #eee";
                border-radius: "8px";
                background: "white";
            }
            .item_info { flex: "1"; }
            .item_name { font-weight: "bold"; color: "#333"; }
            .item_price { color: "#666"; }
            .qty_controls {
                display: "flex";
                align-items: "center";
                gap: "0.5rem";
            }
            .qty_btn {
                width: "32px";
                height: "32px";
                border: "1px solid #ddd";
                border-radius: "4px";
                background: "white";
                cursor: "pointer";
                font-size: "1.2rem";
            }
            .qty_value {
                min-width: "40px";
                text-align: "center";
                font-weight: "bold";
            }
            .item_total {
                font-weight: "bold";
                color: "#4caf50";
                min-width: "80px";
                text-align: "right";
            }
        </style>
        <div class={cart_item}>
            <div class={item_info}>
                <div class={item_name}>{name}</div>
                <div class={item_price}>"$" {format!("{:.2}", price)}</div>
            </div>

            <div class={qty_controls}>
                <button class={qty_btn} on:click={state.decrement}>"-"</button>
                <span class={qty_value} data-bind="quantity">{state.quantity}</span>
                <button class={qty_btn} on:click={state.increment}>"+"</button>
            </div>

            @let total = price * (state.quantity as f64);
            <div class={item_total}>"$" {format!("{:.2}", total)}</div>
        </div>
    }
}

// Handler for Axum
pub async fn lesson11_handler() -> axum::response::Html<String> {
    let cart_state = CartItem { quantity: 1 };

    use cart_item_view_component::Props as CartProps;
    let cart_html = azumi::render_to_string(&cart_item_view_component::render(
        CartProps::builder()
            .state(&cart_state)
            .name("Azumi Pro License")
            .price(99.00)
            .build()
            .expect("props"),
    ));

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Lesson 11: Declarative Event Binding</title>
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
        .section {{ margin: 2rem 0; }}
        .section_title {{ color: #2196f3; margin-bottom: 1rem; }}
        .comparison {{ display: grid; gap: 1rem; margin: 2rem 0; }}
        .compare_box {{ padding: 1rem; border-radius: 8px; font-family: monospace; font-size: 0.85rem; }}
        .old_syntax {{ background: #ffebee; border: 1px solid #ef9a9a; }}
        .new_syntax {{ background: #e8f5e9; border: 1px solid #a5d6a7; }}
        .compare_label {{ font-weight: bold; margin-bottom: 0.5rem; font-family: sans-serif; }}
    </style>
</head>
<body>
    <div class="container">
        <header class="header">
            <h1 class="main_title">Lesson 11: on:event Syntax</h1>
            <p class="subtitle">Declarative event binding</p>
        </header>
        
        <div class="comparison">
            <div class="compare_box old_syntax">
                <div class="compare_label">❌ Old (Manual)</div>
                <code>az-on="click call toggle" data-predict="..."</code>
            </div>
            <div class="compare_box new_syntax">
                <div class="compare_label">✅ New (Declarative)</div>
                <code>on:click={{state.toggle}}</code>
            </div>
        </div>
        
        <section class="section">
            <h2 class="section_title">🛒 Shopping Cart Demo</h2>
            {}
        </section>
    </div>
    <script src="/static/idiomorph.js"></script>
    <script src="/static/azumi.js"></script>
</body>
</html>"#,
        cart_html
    );

    axum::response::Html(html)
}
