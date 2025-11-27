use azumi::html;

pub fn css_variables_demo() -> impl azumi::Component {
    let percentage = "50%";
    let color = "yellow";

    html! {
        <style src="/static/css_variables.css" />

        <div class="progress-bar" --width={percentage} --bg-color={color}>
            "Progress"
        </div>

        <div --static-var="100px">
            "Static Var"
        </div>
    }
}

pub async fn css_variables_handler() -> axum::response::Html<String> {
    let component = css_variables_demo();
    axum::response::Html(azumi::render_to_string(&component))
}
