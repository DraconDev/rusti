use crate::services::auth::UserContext;
use rusti::{component, rusti};

#[component]
pub fn profile_page(user: UserContext) -> impl rusti::Component {
    rusti! {
        <div class="max-w-4xl mx-auto">
            <h1 class="text-4xl font-bold mb-8 bg-gradient-to-r from-cyan-400 to-purple-500 bg-clip-text text-transparent">
                "Profile"
            </h1>

            <div class="glass-card rounded-2xl shadow-2xl p-8 mb-8">
                <div class="flex items-center space-x-6 mb-8">
                    @if let Some(picture) = &user.picture {
                        <img src={picture} alt="Profile" class="w-24 h-24 rounded-full border-4 border-cyan-400 glow-effect" />
                    } else {
                        <div class="w-24 h-24 rounded-full bg-gradient-to-br from-cyan-400 to-purple-500 flex items-center justify-center text-3xl font-bold">
                            {user.name.chars().next().unwrap_or('U').to_string().to_uppercase()}
                        </div>
                    }
                    <div>
                        <h2 class="text-3xl font-bold text-white">{&user.name}</h2>
                        <p class="text-gray-400">{&user.email}</p>
                        <p class="text-sm text-gray-500">"User ID: " {&user.user_id}</p>
                    </div>
                </div>

                <div class="border-t border-gray-700 pt-6">
                    <h3 class="text-xl font-semibold mb-4 text-cyan-400">"Account Information"</h3>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        @info_field("Name", &user.name)
                        @info_field("Email", &user.email)
                        @info_field("User ID", &user.user_id)
                        @info_field("Profile Picture", if user.picture.is_some() { "Set" } else { "Not set" })
                    </div>
                </div>
            </div>

            <div class="glass-card rounded-2xl shadow-2xl p-8">
                <h3 class="text-2xl font-semibold mb-4 text-cyan-400">"Quick Actions"</h3>
                <div class="flex flex-wrap gap-4">
                    <a href="/settings" class="bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-400 hover:to-blue-500 text-white px-6 py-3 rounded-lg transition-all">
                        "⚙️ Settings"
                    </a>
                    <a href="/admin" class="bg-gradient-to-r from-purple-500 to-purple-600 hover:from-purple-400 hover:to-purple-500 text-white px-6 py-3 rounded-lg transition-all">
                        "👑 Admin Dashboard"
                    </a>
                </div>
            </div>
        </div>
    }
}

fn info_field<'a>(label: &'a str, value: &'a str) -> impl rusti::Component + 'a {
    rusti! {
        <div class="bg-gray-800/50 rounded-lg p-4">
            <div class="text-sm text-gray-400 mb-1">{label}</div>
            <div class="text-white font-semibold">{value}</div>
        </div>
    }
}
