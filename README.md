# Test API written in Rust

To run:

1. Clone repo
2. Create postgres database named `testapi`
3. update .env file with credentials. `DATABASE_URL=postgres://username:password@localhost/testapi`
4. Run migrations with `diesel migration run`