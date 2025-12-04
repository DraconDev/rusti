use azumi::prelude::*;
use serde::{Deserialize, Serialize};

#[azumi::live]
pub struct UnifiedCounter {
    pub count: i32,
    pub active: bool,
}

#[azumi::live_impl]
impl UnifiedCounter {
    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
}

// Unified component macro auto-detects live state!
#[azumi::component]
pub fn unified_counter_view(state: &UnifiedCounter) -> impl Component + '_ {
    html! {
        <div class="counter_box">
            <h3>"Unified Component Demo"</h3>
            <p>"Count: " {state.count}</p>
            <p>"Active: " {state.active}</p>

            // New syntax: on:click={state.method}
            // Auto-generates az-on and data-predict!
            <button on:click={state.increment}>
                "Increment (+1)"
            </button>

            <button on:click={state.toggle}>
                "Toggle Active"
            </button>
            <style>
                .counter_box {
                    border: "1px solid #ccc";
                    padding: "1em";
                    border-radius: "8px";
                }
            </style>
        </div>
    }
}

pub async fn unified_demo_handler() -> axum::response::Html<String> {
    let state = UnifiedCounter {
        count: 0,
        active: true,
    };

    axum::response::Html(azumi::render_to_string(
        &unified_counter_view_component::render(
            unified_counter_view_component::Props::builder()
                .state(&state)
                .build()
                .expect("props"),
        ),
    ))
}

azumi::inventory::submit! {
    azumi::action::ActionEntry {
        path: "/unified_demo",
        handler: unified_demo_router,
    }
}

#[allow(non_snake_case)]
pub fn unified_demo_router() -> axum::routing::MethodRouter<()> {
    axum::routing::get(unified_demo_handler)
}
