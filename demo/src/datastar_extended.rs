use axum::response::{Html, IntoResponse};
use rusti::rusti;

/// A more complex Datastar demo with nested state, computed values, and event handling.
pub fn datastar_extended_demo() -> impl rusti::Component {
    rusti!(
        <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>Extended Datastar Demo</title>
                <!-- Tailwind for styling -->
                <script src="https://cdn.tailwindcss.com"></script>
                <!-- Datastar library -->
                <script type="module" src="https://cdn.jsdelivr.net/npm/@sudodevnull/datastar"></script>
            </head>
            <body class="bg-gray-900 text-gray-100 min-h-screen p-8 font-sans">
                <div class="max-w-3xl mx-auto space-y-8">
                    <header class="text-center">
                        <h1 class="text-5xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-pink-500 to-violet-500">
                            Datastar Extended
                        </h1>
                        <p class="mt-4 text-gray-400">Testing complex data attributes and state management.</p>
                    </header>

                    <!-- Store definition with complex nested object -->
                    <div data-store="{
                        search: '',
                        users: [
                            { id: 1, name: 'Alice', role: 'Admin', active: true },
                            { id: 2, name: 'Bob', role: 'User', active: false },
                            { id: 3, name: 'Charlie', role: 'User', active: true },
                            { id: 4, name: 'Diana', role: 'Manager', active: true }
                        ],
                        showInactive: false,
                        newUserName: '',
                        newUserRole: 'User'
                    }">

                        <!-- Search and Filter Controls -->
                        <div class="bg-gray-800 p-6 rounded-xl shadow-lg border border-gray-700 space-y-6">
                            <div class="flex flex-col md:flex-row gap-4">
                                <div class="flex-1">
                                    <label class="block text-sm font-medium text-gray-400 mb-1">Search Users</label>
                                    <input type="text"
                                           data-model="search"
                                           placeholder="Type to filter..."
                                           class="w-full bg-gray-700 border border-gray-600 rounded-lg px-4 py-2 focus:ring-2 focus:ring-pink-500 focus:outline-none transition" />
                                </div>
                                <div class="flex items-end">
                                    <label class="flex items-center space-x-2 cursor-pointer bg-gray-700 px-4 py-2 rounded-lg hover:bg-gray-600 transition">
                                        <input type="checkbox" data-model="showInactive" class="form-checkbox h-5 w-5 text-pink-500 rounded focus:ring-offset-gray-800" />
                                        <span>Show Inactive</span>
                                    </label>
                                </div>
                            </div>

                            <!-- Computed Results Summary -->
                            <div class="p-4 bg-gray-700/50 rounded-lg border border-gray-600/50">
                                <p class="text-sm text-gray-300">
                                    Searching for: <span data-text="$search" class="font-bold text-pink-400"></span>
                                </p>
                                <p class="text-xs text-gray-500 mt-1">
                                    (This demonstrates reactive text binding)
                                </p>
                            </div>
                        </div>

                        <!-- User List Table -->
                        <div class="bg-gray-800 rounded-xl shadow-lg border border-gray-700 overflow-hidden mt-8">
                            <table class="w-full text-left">
                                <thead class="bg-gray-700/50 text-gray-400 uppercase text-xs tracking-wider">
                                    <tr>
                                        <th class="px-6 py-4">Status</th>
                                        <th class="px-6 py-4">Name</th>
                                        <th class="px-6 py-4">Role</th>
                                        <th class="px-6 py-4 text-right">Actions</th>
                                    </tr>
                                </thead>
                                <tbody class="divide-y divide-gray-700">
                                    <!--
                                        Note: In a real Datastar app, we might use a backend loop or a client-side repeater.
                                        Since Rusti is server-side, we will render the initial list, but for this demo
                                        we are testing the parsers ability to handle these complex data attributes.
                                        We will simulate client-side visibility toggling.
                                    -->
                                    <tr class="hover:bg-gray-700/50 transition"
                                        data-show="$showInactive || true"> <!-- Hardcoded true for demo visibility, normally logic -->
                                        <td class="px-6 py-4">
                                            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800">
                                                Active
                                            </span>
                                        </td>
                                        <td class="px-6 py-4 font-medium text-white">Alice</td>
                                        <td class="px-6 py-4 text-gray-400">Admin</td>
                                        <td class="px-6 py-4 text-right">
                                            <button class="text-gray-400 hover:text-white transition" data-on-click="alert('Edit Alice')">
                                                Edit
                                            </button>
                                        </td>
                                    </tr>
                                    <tr class="hover:bg-gray-700/50 transition"
                                        data-show="$showInactive"> <!-- This row toggles based on checkbox -->
                                        <td class="px-6 py-4">
                                            <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800">
                                                Inactive
                                            </span>
                                        </td>
                                        <td class="px-6 py-4 font-medium text-white">Bob</td>
                                        <td class="px-6 py-4 text-gray-400">User</td>
                                        <td class="px-6 py-4 text-right">
                                            <button class="text-gray-400 hover:text-white transition" data-on-click="alert('Edit Bob')">
                                                Edit
                                            </button>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>

                        <!-- Add User Form (Testing Event Modifiers) -->
                        <div class="mt-8 bg-gray-800 p-6 rounded-xl shadow-lg border border-gray-700">
                            <h3 class="text-xl font-bold mb-4 text-white">Add New User</h3>
                            <form data-on-submit__prevent="alert('Added user: ' + $newUserName + ' as ' + $newUserRole); $newUserName = '';">
                                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                    <input type="text"
                                           data-model="newUserName"
                                           placeholder="Username"
                                           class="bg-gray-700 border border-gray-600 rounded-lg px-4 py-2 focus:ring-2 focus:ring-violet-500 focus:outline-none text-white"
                                           required />

                                    <select data-model="newUserRole"
                                            class="bg-gray-700 border border-gray-600 rounded-lg px-4 py-2 focus:ring-2 focus:ring-violet-500 focus:outline-none text-white">
                                        <option value="User">User</option>
                                        <option value="Manager">Manager</option>
                                        <option value="Admin">Admin</option>
                                    </select>

                                    <button type="submit"
                                            class="bg-gradient-to-r from-pink-600 to-violet-600 hover:from-pink-500 hover:to-violet-500 text-white font-bold py-2 px-4 rounded-lg shadow-lg transform transition hover:-translate-y-0.5">
                                        Add User
                                    </button>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </body>
        </html>
    )
}

pub async fn datastar_extended_handler() -> impl IntoResponse {
    Html(rusti::render_to_string(&datastar_extended_demo()))
}
