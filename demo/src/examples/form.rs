use azumi::html;

pub fn form() -> impl azumi::Component {
    html! {
        <form>
            <input type="text" name="name" />
            <input type="email" name="mail" />
            <button type="submit">"Submit"</button>
        </form>
    }
}
