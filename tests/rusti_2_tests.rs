// // Rusti 2.0 Parser Tests

// use azumi::html;

// #[test]
// fn test_quoted_text_required() {
//     // This should compile - quoted text
//     let _ = html! {
//         <h1>"Hello World"</h1>
//     };
// }

// #[test]
// fn test_quoted_attributes_required() {
//     // This should compile - quoted attributes
//     let _ = html! {
//         <div class="container" id="app">"Content"</div>
//     };
// }

// #[test]
// fn test_boolean_attributes_allowed() {
//     // This should compile - boolean attributes don't need values
//     let _ = html! {
//         <input disabled />
//         <button checked>"Button"</button>
//     };
// }

// #[test]
// fn test_dynamic_expressions_allowed() {
//     // This should compile - dynamic expressions work
//     let name = "Test";
//     let _ = html! {
//         <div class={name}>{name}</div>
//     };
// }

// #[test]
// fn test_external_style_allowed() {
//     // This should compile - external CSS via src
//     let _ = html! {
//         <div>
//             <style src="styles.css" />
//             <p>"Content"</p>a
//         </div>
//     };
// }

// #[test]
// fn test_external_script_allowed() {
//     // This should compile - external JS via src
//     let _ = html! {
//         <div>
//             <script src="/static/app.js" />
//             <p>"Content"</p>
//         </div>
//     };
// }

// #[test]
// fn test_json_script_allowed() {
//     // This should compile - JSON data script
//     let data = r#"{"key": "value"}"#;
//     let _ = html! {
//         <script type="application/json">{data}</script>
//     };
// }
