docker run --name newsletter_db  -e POSTGRES_USER=postgres  -e POSTGRES_PASSWORD=ass  -e POSTGRES_DB=newsletter_db  -p "127.0.0.1:5432:5432"  -d postgres
$env:PGPASSWORD = "ass"
psql -h "localhost" -U postgres -p 5432 -d "postgres" -c "\q"
$env:DATABASE_URL = "postgres://postgres:ass@localhost:5432/newsletter_db"
sqlx database create
sqlx migrate add create_subscription_table
Write-Host "Postgres has been migrated, ready to go!"
# SKIP_DOCKER=true ./scripts/init_db.ps1