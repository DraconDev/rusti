module github.com/DraconDev/go-templ-htmx-ex

go 1.24

require (
	github.com/a-h/templ v0.3.960
	github.com/dracondev/go-templ-htmx-ex/libs/cachex v0.0.0
	github.com/dracondev/go-templ-htmx-ex/libs/configx v0.0.0
	github.com/dracondev/go-templ-htmx-ex/libs/dbx v0.0.0
	github.com/dracondev/go-templ-htmx-ex/libs/httperrx v0.0.0
	github.com/google/uuid v1.6.0
	github.com/gorilla/mux v1.8.1
	github.com/lib/pq v1.10.9
)

require (
	github.com/google/go-cmp v0.7.0 // indirect
	github.com/joho/godotenv v1.5.1 // indirect
)

replace (
	github.com/dracondev/go-templ-htmx-ex/libs/cachex => ./libs/cachex
	github.com/dracondev/go-templ-htmx-ex/libs/configx => ./libs/configx
	github.com/dracondev/go-templ-htmx-ex/libs/dbx => ./libs/dbx
	github.com/dracondev/go-templ-htmx-ex/libs/httperrx => ./libs/httperrx
)
