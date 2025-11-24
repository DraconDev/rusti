package admin

import (
	"fmt"
	"net/http"

	"github.com/DraconDev/go-templ-htmx-ex/internal/middleware"
	"github.com/DraconDev/go-templ-htmx-ex/templates/layouts"
	"github.com/DraconDev/go-templ-htmx-ex/templates/pages"
)

// =============================================================================
// ADMIN DASHBOARD HANDLERS
// =============================================================================
// These handlers manage the admin dashboard and its functionality:
// - Dashboard rendering and data loading
// - Admin access validation
// - Dashboard HTML generation
// =============================================================================

// AdminDashboardHandler serves the admin dashboard
func (h *AdminHandler) AdminDashboardHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Printf("📋 ADMIN: Admin dashboard requested\n")

	// Get user info from middleware context
	userInfo := middleware.GetUserFromContext(r)
	if !userInfo.LoggedIn {
		fmt.Printf("📋 ADMIN: User not logged in\n")
		http.Redirect(w, r, "/", http.StatusFound)
		return
	}

	fmt.Printf("📋 ADMIN: User logged in: %s (%s)\n", userInfo.Name, userInfo.Email)

	// Check if this user is admin using UserService
	if h.UserService != nil {
		userRecord, err := h.UserService.GetUserByEmail(r.Context(), userInfo.Email)
		if err != nil {
			fmt.Printf("📋 ACCESS DENIED: Could not fetch user from database: %v\n", err)
			http.Error(w, "Access denied: Admin privileges required", http.StatusForbidden)
			return
		}

		// Check if user is admin
		if !userRecord.IsAdmin {
			fmt.Printf("📋 ACCESS DENIED: User %s is not admin in database\n", userInfo.Email)
			http.Error(w, "Access denied: Admin privileges required", http.StatusForbidden)
			return
		}
	} else {
		// Fallback: no database connection - deny access
		fmt.Printf("📋 ACCESS DENIED: No database connection available\n")
		http.Error(w, "Access denied: Admin privileges required", http.StatusForbidden)
		return
	}

	fmt.Printf("📋 ADMIN: Access granted for admin %s\n", userInfo.Email)

	// Get dashboard data
	dashboardData := h.getDashboardData(r)

	// Render dashboard
	h.renderAdminDashboard(w, r, userInfo, dashboardData)
}

// getDashboardData loads real dashboard data from UserService
func (h *AdminHandler) getDashboardData(r *http.Request) pages.DashboardData {
	var dashboardData pages.DashboardData
	dashboardData.SystemHealth = "operational"

	if h.UserService == nil {
		// Default values when no database connection
		dashboardData.SystemHealth = "offline"
		fmt.Printf("⚠️ ADMIN: No UserService available\n")
		return dashboardData
	}

	fmt.Printf("📊 ADMIN: Loading real database data...\n")

	// Total users
	totalUsers, err := h.UserService.CountUsers(r.Context())
	if err == nil {
		dashboardData.TotalUsers = int(totalUsers)
		fmt.Printf("📊 ADMIN: Total users loaded: %d\n", dashboardData.TotalUsers)
	} else {
		fmt.Printf("❌ ADMIN: Error loading total users: %v\n", err)
	}

	// Today's signups
	signupsToday, err := h.UserService.CountUsersCreatedToday(r.Context())
	if err == nil {
		dashboardData.SignupsToday = int(signupsToday)
		fmt.Printf("📊 ADMIN: Today's signups loaded: %d\n", dashboardData.SignupsToday)
	} else {
		fmt.Printf("❌ ADMIN: Error loading today's signups: %v\n", err)
	}

	// This week's signups
	signupsThisWeek, err := h.UserService.CountUsersCreatedThisWeek(r.Context())
	if err == nil {
		dashboardData.UsersThisWeek = int(signupsThisWeek)
		fmt.Printf("📊 ADMIN: This week's signups loaded: %d\n", dashboardData.UsersThisWeek)
	} else {
		fmt.Printf("❌ ADMIN: Error loading this week's signups: %v\n", err)
	}

	// Recent users
	recentUsers, err := h.UserService.GetRecentUsers(r.Context())
	if err == nil && len(recentUsers) > 0 {
		// Show up to 5 recent users
		maxUsers := 5
		if len(recentUsers) < maxUsers {
			maxUsers = len(recentUsers)
		}
		for i, user := range recentUsers[:maxUsers] {
			dashboardData.RecentUsers = append(dashboardData.RecentUsers, pages.RecentUser{
				Name:  user.Name,
				Email: user.Email,
				Date:  user.CreatedAt.Format("2006-01-02"),
			})
			fmt.Printf("📊 ADMIN: Recent user %d: %s (%s)\n", i+1, user.Name, user.Email)
		}
	} else if err != nil {
		fmt.Printf("❌ ADMIN: Error loading recent users: %v\n", err)
	} else {
		fmt.Printf("⚠️ ADMIN: No recent users found\n")
	}

	fmt.Printf("📊 ADMIN: Dashboard data ready - Total: %d, Today: %d, ThisWeek: %d, Recent: %d\n",
		dashboardData.TotalUsers, dashboardData.SignupsToday, dashboardData.UsersThisWeek, len(dashboardData.RecentUsers))

	return dashboardData
}

// renderAdminDashboard renders the admin dashboard HTML
func (h *AdminHandler) renderAdminDashboard(w http.ResponseWriter, r *http.Request, userInfo layouts.UserInfo, dashboardData pages.DashboardData) {
	w.Header().Set("Content-Type", "text/html")

	// Convert layouts.UserInfo to pages.UserInfo for the dashboard content
	pagesUserInfo := pages.UserInfo{
		LoggedIn: userInfo.LoggedIn,
		Name:     userInfo.Name,
		Email:    userInfo.Email,
		Picture:  userInfo.Picture,
	}

	component := layouts.Layout("Admin Dashboard", "Administrative dashboard with user statistics, analytics, and platform management tools.", layouts.NavigationLoggedIn(userInfo), pages.AdminDashboardContent(pagesUserInfo, dashboardData))
	if err := component.Render(r.Context(), w); err != nil {
		fmt.Printf("🚨 ADMIN: Error rendering admin dashboard: %v\n", err)
		http.Error(w, "Internal Server Error", http.StatusInternalServerError)
		return
	}
}
