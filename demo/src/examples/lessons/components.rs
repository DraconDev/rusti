// Reusable lesson components with scoped CSS - Azumi style!
use azumi::html;

pub fn LessonHeader(title: &str, subtitle: &str, lesson_num: &str) -> impl azumi::Component {
    html! {
        <header class="lesson-header">
            <a href="/" class="back-link">"← Back to Lessons"</a>
            <div class="lesson-number">{lesson_num}</div>
            <h1 class="lesson-title">{title}</h1>
            <p class="lesson-subtitle">{subtitle}</p>
        </header>
    }
}

pub fn Section(title: &str, content: impl azumi::Component) -> impl azumi::Component {
    html! {
        <section class="section">
            <h2 class="section-title">{title}</h2>
            {content}
        </section>
    }
}

pub fn HighlightBox(content: impl azumi::Component) -> impl azumi::Component {
    html! {
        <div class="highlight-box">
            {content}
        </div>
    }
}

pub fn NavigationButtons(prev: &str, next: &str) -> impl azumi::Component {
    html! {
        <nav class="nav-buttons">
            <a href={prev} class="btn btn-secondary">"← Previous"</a>
            <a href={next} class="btn">"Next →"</a>
        </nav>
    }
}
