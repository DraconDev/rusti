//! Lesson 20: full_application_example.rs
//!
//! Complete CRUD application with all Azumi features
use azumi::html;

/// Blog post data model
#[derive(Clone)]
pub struct BlogPost {
    pub id: Option<u32>,
    pub title: String,
    pub content: String,
    pub author: String,
    pub published: bool,
    pub created_at: String,
}

/// Blog post form component
pub struct BlogFormProps {
    pub post: Option<BlogPost>,
    pub mode: String, // "new" or "edit"
}

pub fn blog_form(props: BlogFormProps) -> impl azumi::Component {
    let (title_value, content_value, author_value, submit_text) = match &props.post {
        Some(post) => (
            post.title.clone(),
            post.content.clone(),
            post.author.clone(),
            "Update Post",
        ),
        None => (
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "Create Post",
        ),
    };

    html! {
        <style src="/static/pages/lesson20.css" />
        <div class="blog-form-container">
            <h2>{if props.mode == "edit" { "Edit Blog Post" } else { "Create New Blog Post" }}</h2>
            
            <form hx-post={format!("/blog/{}", if props.mode == "edit" { "update" } else { "save" })} 
                  hx-swap="none" 
                  class="blog-form">
                
                @if let Some(post) = &props.post {
                    <input type="hidden" name="id" value={post.id.unwrap_or(0).to_string()} />
                }

                <div class="form-group">
                    <label for="title">"Title"</label>
                    <input 
                        type="text" 
                        id="title"
                        name="title" 
                        value={title_value}
                        required 
                        placeholder="Enter blog post title..." />
                </div>

                <div class="form-group">
                    <label for="author">"Author"</label>
                    <input 
                        type="text" 
                        id="author"
                        name="author" 
                        value={author_value}
                        required 
                        placeholder="Author name..." />
                </div>

                <div class="form-group">
                    <label for="content">"Content"</label>
                    <textarea 
                        id="content"
                        name="content" 
                        rows="10"
                        required 
                        placeholder="Write your blog post content here...">{content_value}</textarea>
                </div>

                <div class="form-actions">
                    <button type="submit" class="btn btn-primary">{submit_text}</button>
                    <a href="/blog" class="btn btn-secondary">"Cancel"</a>
                </div>
            </form>
        </div>
    }
}

/// Blog post list component
pub fn blog_list(posts: &[BlogPost]) -> impl azumi::Component {
    html! {
        <div class="blog-list-container">
            <div class="blog-header">
                <h1>"Blog Posts"</h1>
                <a href="/blog/new" class="btn btn-primary">"New Post"</a>
            </div>

            @if posts.is_empty() {
                <div class="empty-state">
                    <p>"No blog posts yet. Create your first post!"</p>
                    <a href="/blog/new" class="btn btn-primary">"Create Post"</a>
                </div>
            } else {
                <div class="posts-grid">
                    @for post in posts {
                        <article class="blog-post-card">
                            <header class="post-header">
                                <h2 class="post-title">{&post.title}</h2>
                                <div class="post-meta">
                                    <span>"By " {&post.author}</span>
                                    <span>"| Created: " {&post.created_at}</span>
                                    @if post.published {
                                        <span class="status-published">"Published"</span>
                                    } else {
                                        <span class="status-draft">"Draft"</span>
                                    }
                                </div>
                            </header>
                            
                            <div class="post-content">
                                <p>{&post.content}</p>
                            </div>
                            
                            <footer class="post-actions">
                                <a href={format!("/blog/{}", post.id.unwrap_or(0))} class="btn btn-small">"Read More"</a>
                                <a href={format!("/blog/edit/{}", post.id.unwrap_or(0))} class="btn btn-small">"Edit"</a>
                                <button 
                                    hx-delete={format!("/blog/delete/{}", post.id.unwrap_or(0))}
                                    hx-swap="outerHTML"
                                    class="btn btn-small btn-danger">"Delete"</button>
                            </footer>
                        </article>
                    }
                </div>
            }
        </div>
    }
}

/// Blog post detail component
pub fn blog_detail(post: &BlogPost) -> impl azumi::Component {
    html! {
        <div class="blog-detail-container">
            <article class="blog-post-detail">
                <header class="post-header">
                    <h1 class="post-title">{&post.title}</h1>
                    <div class="post-meta">
                        <span>"By " {&post.author}</span>
                        <span>"| " {&post.created_at}</span>
                        @if post.published {
                            <span class="status-published">"Published"</span>
                        } else {
                            <span class="status-draft">"Draft"</span>
                        }
                    </div>
                </header>
                
                <div class="post-content">
                    <p>{&post.content}</p>
                </div>
                
                <footer class="post-actions">
                    <a href="/blog" class="btn btn-secondary">"← Back to Posts"</a>
                    <a href={format!("/blog/edit/{}", post.id.unwrap_or(0))} class="btn btn-primary">"Edit Post"</a>
                </footer>
            </article>
        </div>
    }
}

/// Admin dashboard with all blog posts
pub fn admin_dashboard(posts: &[BlogPost]) -> impl azumi::Component {
    let published_count = posts.iter().filter(|p| p.published).count();
    let draft_count = posts.iter().filter(|p| !p.published).count();

    html! {
        <div class="admin-dashboard">
            <div class="dashboard-header">
                <h1>"Blog Admin Dashboard"</h1>
                <a href="/blog/new" class="btn btn-primary">"Create New Post"</a>
            </div>

            <div class="dashboard-stats">
                <div class="stat-card">
                    <h3>"Total Posts"</h3>
                    <div class="stat-value">{posts.len().to_string()}</div>
                </div>
                <div class="stat-card">
                    <h3>"Published"</h3>
                    <div class="stat-value">{published_count.to_string()}</div>
                </div>
                <div class="stat-card">
                    <h3>"Drafts"</h3>
                    <div class="stat-value">{draft_count.to_string()}</div>
                </div>
            </div>

            <div class="dashboard-content">
                <h2>"Recent Posts"</h2>
                @blog_list(posts)
            </div>
        </div>
    }
}

/// Complete application layout
pub fn app_layout_with_blog(content: impl azumi::Component) -> impl azumi::Component {
    html! {
        <style src="/static/pages/lesson20.css" />
        <html>
            <head>
                <title>"Azumi Blog Application"</title>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
            </head>
            <body>
                <div class="app-shell">
                    <header class="app-header">
                        <div class="header-content">
                            <h1 class="app-title">"Azumi Blog"</h1>
                            <nav class="main-nav">
                                <a href="/" class="nav-link">"Home"</a>
                                <a href="/blog" class="nav-link">"Blog"</a>
                                <a href="/admin" class="nav-link">"Admin"</a>
                            </nav>
                        </div>
                    </header>

                    <main class="app-main">
                        {content}
                    </main>

                    <footer class="app-footer">
                        <p>"Built with Azumi + HTMX + Rust"</p>
                    </footer>
                </div>

                <!-- Load HTMX for interactivity -->
                <script src="https://unpkg.com/htmx.org@1.9.10"></script>
            </body>
        </html>
    }
}

/// Main application demo
pub fn full_application_demo() -> impl azumi::Component {
    let sample_posts = vec![
        BlogPost {
            id: Some(1),
            title: "Welcome to Azumi".to_string(),
            content: "This is the first post showcasing how to build a complete blog application with Azumi. Azumi provides type-safe HTML templates with compile-time validation, making your web development more robust and maintainable.".to_string(),
            author: "DraconDev".to_string(),
            published: true,
            created_at: "2024-01-15".to_string(),
        },
        BlogPost {
            id: Some(2),
            title: "Building Interactive UIs with HTMX".to_string(),
            content: "HTMX allows you to build dynamic, interactive web applications without JavaScript frameworks. Combined with Azumi's type safety, you get the best of both worlds - robust server-side rendering with modern interactivity.".to_string(),
            author: "Alice Johnson".to_string(),
            published: true,
            created_at: "2024-01-20".to_string(),
        },
        BlogPost {
            id: Some(3),
            title: "Draft: Advanced Component Patterns".to_string(),
            content: "This is a draft post about advanced component patterns in Azumi. It will cover component composition, state management, and real-world application architecture patterns.".to_string(),
            author: "Bob Smith".to_string(),
            published: false,
            created_at: "2024-01-22".to_string(),
        },
    ];

    html! {
        <div class="full-app-demo">
            <section class="demo-section">
                <h2>"Application Layout"</h2>
                @app_layout_with_blog(html! {
                    <div class="demo-content">
                        <h2>"Welcome to the Blog"</h2>
                        <p>"This demonstrates a complete application layout with header, navigation, and footer"</p>
                    </div>
                })
            </section>

            <section class="demo-section">
                <h2>"Blog List"</h2>
                @blog_list(&sample_posts)
            </section>

            <section class="demo-section">
                <h2>"Create Post Form"</h2>
                @blog_form(BlogFormProps {
                    post: None,
                    mode: "new".to_string(),
                })
            </section>

            <section class="demo-section">
                <h2>"Admin Dashboard"</h2>
                @admin_dashboard(&sample_posts)
            </section>

            <section class="demo-section">
                <h2>"Application Features Demonstrated"</h2>
                <div class="features-grid">
                    <div class="feature-card">
                        <h3>"Type Safety"</h3>
                        <p>"Compile-time validation of HTML structure and CSS classes"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"Component Architecture"</h3>
                        <p>"Reusable components with props and composition"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"HTMX Integration"</h3>
                        <p>"Dynamic interactions without JavaScript frameworks"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"Layout System"</h3>
                        <p>"Flexible layout patterns for different page types"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"Form Handling"</h3>
                        <p>"Type-safe forms with validation and error handling"</p>
                    </div>
                    <div class="feature-card">
                        <h3>"CRUD Operations"</h3>
                        <p>"Complete create, read, update, delete functionality"</p>
                    </div>
                </div>
            </section>
        </div>
    }
}

/// Handler for Axum
pub async fn lesson20_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&full_application_demo()))
}