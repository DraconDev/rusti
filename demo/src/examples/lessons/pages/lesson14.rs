
use azumi::html;

/// Lesson 14: Complete Blog Application
///
/// Full-stack blog with all Azumi features
mod blog {
    use super::*;

    #[derive(Debug)]
    struct Post {
        id: i32,
        title: String,
        content: String,
        author: String,
        created_at: String,
    }

    #[azumi::component]
    pub fn post_card(post: Post) -> impl azumi::Component {
        html! {
            <style>
                .post_card { border: "1px solid #eee"; padding: "1.5rem"; margin-bottom: "1rem"; border-radius: "8px"; }
                .post_title { font-size: "1.5rem"; margin-bottom: "0.5rem"; color: "#1976d2"; }
                .post_meta { display: "flex"; gap: "1rem"; color: "#666"; margin-bottom: "1rem"; font-size: "0.9rem"; }
                .post_content { line-height: "1.6"; }
                .post_author { font-weight: "bold"; }
            </style>
            <article class={post_card}>
                <h2 class={post_title}>{post.title}</h2>
                <div class={post_meta}>
                    <span class={post_author}>"By " {post.author}</span>
                    <span>"Published on " {post.created_at}</span>
                </div>
                <div class={post_content}>{post.content}</div>
            </article>
        }
    }

    #[azumi::component]
    pub fn blog_homepage() -> impl azumi::Component {
        let posts = vec![
            Post {
                id: 1,
                title: "Getting Started with Azumi",
                content: "Learn how to build your first Azumi application with this comprehensive guide.",
                author: "Azumi Team",
                created_at: "2023-01-15",
            },
            Post {
                id: 2,
                title: "Advanced CSS Techniques",
                content: "Explore advanced CSS patterns and how Azumi handles them at compile time.",
                author: "CSS Expert",
                created_at: "2023-02-20",
            },
            Post {
                id: 3,
                title: "Database Integration",
                content: "Connect your Azumi app to databases with these proven patterns and best practices.",
                author: "Backend Specialist",
                created_at: "2023-03-10",
            },
        ];

        html! {
            <style>
                .blog_container { max-width: "800px"; margin: "0 auto"; padding: "2rem"; }
                .blog_header { text-align: "center"; margin-bottom: "2rem"; }
                .blog_title { font-size: "2.5rem"; color: "#2196f3"; }
