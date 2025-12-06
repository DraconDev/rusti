use crate::examples::lessons::components::layout::DarkModernLayout;
use azumi::html;

#[azumi::component]
pub fn homepage() -> impl azumi::Component {
    html! {
        @DarkModernLayout() {
            @Lessons()
        }
    }
}

#[azumi::component]
pub fn Lessons() -> impl azumi::Component {
    html! {
        <style>
            .homepage_title {
                color: "purple";
                font-size: "3em";
            }
            .lesson_grid {
                display: "grid";
                grid-template-columns: "repeat(auto-fill, minmax(300px, 1fr))";
                gap: "20px";
                margin-top: "30px";
            }
            .lesson_card {
                border: "1px solid #ddd";
                padding: "20px";
                border-radius: "8px";
                background: "white";
                box-shadow: "0 2px 4px rgba(0,0,0,0.1)";
                transition: "transform 0.2s";
            }
            .lesson_card:hover {
                transform: "translateY(-5px)";
                box-shadow: "0 4px 8px rgba(0,0,0,0.15)";
            }
            .lesson_title {
                color: "#1976d2";
                font-size: "1.2rem";
                margin-bottom: "10px";
            }
            .lesson_link {
                color: "#6c5ce7";
                text-decoration: "none";
                font-weight: "bold";
                display: "inline-block";
                margin-top: "10px";
            }
            .lesson_link:hover {
                text-decoration: "underline";
            }
            .subtitle {
                text-align: "center";
                color: "#666";
                margin-bottom: "30px";
            }
        </style>
        <div>
            <h1 class={homepage_title}>
                "Azumi Learning Platform"
            </h1>
            <p class={subtitle}>
                "Explore interactive lessons to learn Azumi framework!"
            </p>

            <div class={lesson_grid}>
                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 0: Introduction"</h3>
                    <p>"Basic component structure and syntax"</p>
                    <a href="/lesson-0" class={lesson_link}>"Start Lesson 0 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 1: Components"</h3>
                    <p>"Introduction to Azumi Components"</p>
                    <a href="/lesson-1" class={lesson_link}>"Start Lesson 1 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 2: CSS Scoping"</h3>
                    <p>"CSS Scoping & Validation Fundamentals"</p>
                    <a href="/lesson-2" class={lesson_link}>"Start Lesson 2 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 3: Global vs Component CSS"</h3>
                    <p>"Understanding style scoping options"</p>
                    <a href="/lesson-3" class={lesson_link}>"Start Lesson 3 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 4: HTML Structure"</h3>
                    <p>"HTML structure validation and best practices"</p>
                    <a href="/lesson-4" class={lesson_link}>"Start Lesson 4 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 5: Accessibility"</h3>
                    <p>"Accessibility validation and best practices"</p>
                    <a href="/lesson-5" class={lesson_link}>"Start Lesson 5 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 6: Forms"</h3>
                    <p>"Form handling and validation"</p>
                    <a href="/lesson-6" class={lesson_link}>"Start Lesson 6 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 7: Events"</h3>
                    <p>"Event handling and interactivity"</p>
                    <a href="/lesson-7" class={lesson_link}>"Start Lesson 7 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 8: State Management"</h3>
                    <p>"State management patterns"</p>
                    <a href="/lesson-8" class={lesson_link}>"Start Lesson 8 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 9: Advanced Components"</h3>
                    <p>"Advanced component patterns"</p>
                    <a href="/lesson-9" class={lesson_link}>"Start Lesson 9 →"</a>
                </div>

                <div class={lesson_card}>
                    <h3 class={lesson_title}>"Lesson 10: Performance"</h3>
                    <p>"Performance optimization techniques"</p>
                    <a href="/lesson-10" class={lesson_link}>"Start Lesson 10 →"</a>
                </div>
            </div>
        </div>
    }
}

pub async fn homepage_handler() -> impl axum::response::IntoResponse {
    axum::response::Html(azumi::render_to_string(&homepage()))
}
