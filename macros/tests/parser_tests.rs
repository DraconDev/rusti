// // Tests for azumi
 parser improvements

// use azumi
::{render_to_string, azumi
, Component};
// use azumi
_macros::azumi
;

// #[test]
// fn script_tag_is_handled() {
//     let comp = html!
 {
//         <html>
//             <head></head>
//             <body>
//                 <script>
//                     let x = 5;
//                     console.log(x);
//                 </script>
//                 <div>"Content"</div>
//             </body>
//         </html>
//     };
//     let output = render_to_string(&comp);
//     assert!(output.contains("Content"));
//     assert!(output.contains("<script>"));
// }

// #[test]
// fn html_comment_is_ignored() {
//     let comp = html!
 {
//         <html>
//             <!-- This is a comment -->
//             <body>"Hello"</body>
//         </html>
//     };
//     let output = render_to_string(&comp);
//     assert!(output.contains("Hello"));
// }
