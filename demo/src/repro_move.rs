use azumi::prelude::*;

#[azumi::component]
pub fn reproduction(children: impl Component) -> impl Component {
    html! {
        <div>
            {children}
        </div>
    }
}

pub fn main() {}
