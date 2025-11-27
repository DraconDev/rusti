use azumi::html;

pub fn css_variables_demo() -> impl azumi::Component {
    let percentage = "50%";
    let color = "blue";

    html! {
        <style src="examples/css_variables.css" />

        <div class="progress-bar" --width={percentage} --bg-color={color}>
            "Progress"
        </div>

        <div --static-var="100px">
            "Static Var"
        </div>
    }
}
