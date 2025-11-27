use azumi::html;

pub fn css_variables_demo() -> impl azumi::Component {
    let percentage = "50%";
    let color = "red";

    html! {
        <div class="progress-bar" --width={percentage} --bg-color={color}>
            "Progress"
        </div>

        <div --static-var="100px">
            "Static Var"
        </div>
    }
}
