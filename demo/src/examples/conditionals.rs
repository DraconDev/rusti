// Example showing if/else conditional rendering
use rusti::rusti;

pub fn status_badge(is_active: bool) -> impl rusti::Component {
    let (color, text) = if is_active {
        ("bg-green-500", "Active")
    } else {
        ("bg-gray-500", "Inactive")
    };

    rusti! {
        <span class={"px-3 py-1 rounded-full text-white " + color}>
            { text }
        </span>
    }
}

pub fn user_status(username: &str, item_count: i32) -> impl rusti::Component + '_ {
    let message = if item_count > 10 {
        "You have many items!"
    } else if item_count > 0 {
        "You have some items"
    } else {
        "No items yet"
    };

    rusti! {
        <div class="p-4 bg-white rounded shadow">
            <h2 class="text-xl font-bold">{ username }</h2>
            <p class="text-gray-600">Items: { item_count }</p>
            <p class="italic">{ message }</p>
            @status_badge(item_count > 0)
        </div>
    }
}
