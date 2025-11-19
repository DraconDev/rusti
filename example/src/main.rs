use rusti::rusti;

fn header(title: &str) -> impl rusti::Component + '_ {
    rusti! {
        <header>
            <h1>{ title }</h1>
        </header>
    }
}

fn footer(year: i32) -> impl rusti::Component {
    rusti! {
        <footer>
            <p>Copyright { year }</p>
        </footer>
    }
}

fn page<'a>(title: &'a str, body: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <div>
            @header(title)
            <main>
                <p>{ body }</p>
            </main>
            @footer(2023)
        </div>
    }
}

fn main() {
    let title = String::from("My Rusti Page");
    let body = String::from("This is a comprehensive example.");
    let p = page(&title, &body);
    println!("{}", rusti::render_to_string(&p));
}
