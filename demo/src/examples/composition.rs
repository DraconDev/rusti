// Example showing component composition
use rusti::rusti;

pub fn header(title: &str) -> impl rusti::Component + '_ {
    rusti! {
        <header class="bg-blue-600 text-white p-4">
            <h1 class="text-2xl font-bold">{ title }</h1>
        </header>
    }
}

pub fn footer() -> impl rusti::Component {
    rusti! {
        <footer class="bg-gray-800 text-white p-4 text-center">
            <p>© 2025 - Built with Rusti</p>
        </footer>
    }
}

pub fn page(title: &str, content: &str) -> impl rusti::Component + '__ {
    rusti! {
        <div class="min-h-screen flex flex-col">
            @header(title)
            <main class="flex-grow p-8">
                <p>{ content }</p>
            </main>
            @footer()
        </div>
    }
}
