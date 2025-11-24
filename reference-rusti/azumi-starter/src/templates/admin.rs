use rusti::{component, rusti};

#[derive(Debug, Clone, Copy)]
pub struct AdminStats {
    pub total_users: i64,
    pub users_today: i64,
    pub users_this_week: i64,
}

#[component]
pub fn admin_dashboard(
    stats: AdminStats,
    recent_users: Vec<crate::db::User>,
) -> impl rusti::Component {
    rusti! {
        <div class="max-w-6xl mx-auto">
            <h1 class="text-4xl font-bold mb-8 bg-gradient-to-r from-cyan-400 to-purple-500 bg-clip-text text-transparent">
                "Admin Dashboard"
            </h1>

            // Stats Grid
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                @stat_card("👥 Total Users", &stats.total_users.to_string(), "from-blue-500 to-blue-600")
                @stat_card("📈 Signups Today", &stats.users_today.to_string(), "from-green-500 to-green-600")
                @stat_card("📊 This Week", &stats.users_this_week.to_string(), "from-purple-500 to-purple-600")
            </div>

            // Recent Users
            <div class="glass-card rounded-2xl shadow-2xl p-8">
                <h2 class="text-2xl font-semibold mb-6 text-cyan-400">"Recent Users"</h2>
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead>
                            <tr class="border-b border-gray-700">
                                <th class="text-left py-3 px-4 text-gray-400">"Name"</th>
                                <th class="text-left py-3 px-4 text-gray-400">"Email"</th>
                                <th class="text-left py-3 px-4 text-gray-400">"Joined"</th>
                                <th class="text-left py-3 px-4 text-gray-400">"Admin"</th>
                            </tr>
                        </thead>
                        <tbody>
                            @for user in &recent_users {
                                <tr class="border-b border-gray-800 hover:bg-gray-800/50 transition-colors">
                                    <td class="py-3 px-4">
                                        <div class="flex items-center space-x-3">
                                            @if let Some(picture) = &user.picture {
                                                <img src={picture} alt="Avatar" class="w-8 h-8 rounded-full" />
                                            } else {
                                                <div class="w-8 h-8 rounded-full bg-gradient-to-br from-cyan-400 to-purple-500 flex items-center justify-center text-sm font-bold">
                                                    {user.name.chars().next().unwrap_or('U').to_string().to_uppercase()}
                                                </div>
                                            }
                                            <span class="text-white">{&user.name}</span>
                                        </div>
                                    </td>
                                    <td class="py-3 px-4 text-gray-300">{&user.email}</td>
                                    <td class="py-3 px-4 text-gray-400">{user.created_at.format("%Y-%m-%d").to_string()}</td>
                                    <td class="py-3 px-4">
                                        @if user.is_admin {
                                            <span class="bg-purple-500/20 text-purple-400 px-2 py-1 rounded text-sm">"Admin"</span>
                                        } else {
                                            <span class="bg-gray-700/50 text-gray-400 px-2 py-1 rounded text-sm">"User"</span>
                                        }
                                    </td>
                                </tr>
                            }
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}

fn stat_card<'a>(label: &'a str, value: &'a str, gradient: &'a str) -> impl rusti::Component + 'a {
    let card_class = match gradient {
        "from-blue-500 to-blue-600" => {
            "glass-card rounded-2xl shadow-2xl p-6 bg-gradient-to-br from-blue-500 to-blue-600"
        }
        "from-green-500 to-green-600" => {
            "glass-card rounded-2xl shadow-2xl p-6 bg-gradient-to-br from-green-500 to-green-600"
        }
        "from-purple-500 to-purple-600" => {
            "glass-card rounded-2xl shadow-2xl p-6 bg-gradient-to-br from-purple-500 to-purple-600"
        }
        _ => "glass-card rounded-2xl shadow-2xl p-6 bg-gradient-to-br from-gray-500 to-gray-600",
    };

    rusti! {
        <div class={card_class}>
            <p class="text-white/80 text-sm mb-2">{label}</p>
            <p class="text-4xl font-bold text-white">{value}</p>
        </div>
    }
}
