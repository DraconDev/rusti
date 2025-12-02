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
                .posts_list { display: "grid"; gap: "1.5rem"; }
            </style>
            <div class={blog_container}>
                <header class={blog_header}>
                    <h1 class={blog_title}>"Azumi Blog"</h1>
                    <p>"A showcase of Azumi's capabilities"</p>
                </header>
                <main class={posts_list}>
                    @for post in &posts {
                        @post_card(post=post)
                    }
                </main>
            </div>
        }
    }
}

/// Main lesson demonstration component
#[azumi::component]
pub fn lesson14() -> impl azumi::Component {
    html! {
        <style>
            .container { padding: "20px"; }
            .header { text-align: "center"; margin-bottom: "30px"; }
            .main_title { font-size: "32px"; color: "#333"; }
            .subtitle { font-size: "18px"; color: "#666"; }
            .key_points { background: "#f9f9f9"; padding: "20px"; border-radius: "8px"; margin-bottom: "30px"; }
            .section_title { font-size: "20px"; margin-bottom: "15px"; }
            .points_list { list-style: "none"; padding: "0"; }
            .point { margin-bottom: "10px"; }
            .examples { display: "grid"; gap: "20px"; }
            .example_card { border: "1px solid #ddd"; padding: "20px"; border-radius: "8px"; }
        </style>
        <div class={container}>
            <header class={header}>
                <h1 class={main_title}>"Lesson 14: Complete Blog Application"</h1>
                <p class={subtitle}>"Full-stack blog with all Azumi features"</p>
            </header>

            <section class={key_points}>
                <h2 class={section_title}>"Key Concepts"</h2>
                <ul class={points_list}>
                    <li class={point}>"✅ Full application structure"</li>
                    <li class={point}>"✅ Component composition"</li>
                    <li class={point}>"✅ Data modeling"</li>
                    <li class={point}>"✅ List rendering"</li>
                    <li class={point}>"✅ Complete feature integration"</li>
                </ul>
            </section>

            <section class={examples}>
                <div class={example_card}>
                    @blog::blog_homepage()
                </div>
            </section>
        </div>
    }
}

// Handler for Axum
pub async fn lesson14_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&lesson14()))
}
