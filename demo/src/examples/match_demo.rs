// Example showing match expression rendering
use rusti::rusti;

pub fn role_badge(role: &str) -> impl rusti::Component + '_ {
    let (color, description) = match role {
        "admin" => ("red", "Full system access"),
        "moderator" => ("orange", "Content moderation"),
        "user" => ("green", "Standard access"),
        _ => ("gray", "Unknown role"),
    };

    rusti! {
        <div class="p-4 border-l-4" style={format!("border-color: {}", color)}>
            <h3 class="font-bold text-lg">{ role }</h3>
            <p class="text-gray-600">{ description }</p>
        </div>
    }
}

pub fn score_grade(score: i32) -> impl rusti::Component {
    let grade = match score {
        90..=100 => "A - Excellent",
        80..=89 => "B - Good",
        70..=79 => "C - Average",
        60..=69 => "D - Below Average",
        _ => "F - Failing",
    };

    rusti! {
        <div class="text-center p-6 bg-blue-50 rounded">
            <div class="text-4xl font-bold text-blue-600">{ score }</div>
            <div class="text-lg mt-2">{ grade }</div>
        </div>
    }
}
