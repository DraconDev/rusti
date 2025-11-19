// Example showing HTML attributes with Tailwind CSS classes
use rusti::rusti;

pub fn styled_card<'a>(title: &'a str, description: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <div class="max-w-md mx-auto bg-white rounded-xl shadow-md overflow-hidden md:max-w-2xl m-4">
            <div class="p-8">
                <div class="uppercase tracking-wide text-sm text-indigo-500 font-semibold">{ title }</div>
                <p class="mt-2 text-gray-500">{ description }</p>
            </div>
            <a href="#" class="block p-4 text-center text-blue-600 hover:text-blue-800 font-medium transition-colors">Read More</a>
        </div>
        
    }
}
