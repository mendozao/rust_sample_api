# Rust Sample API

To run:

1. Clone repo
2. Install diesel_cli globally `cargo install diesel_cli` [http://diesel.rs/guides/getting-started/]
2. Create postgres database `createdb rust_sample_api`
3. Update .env file `DATABASE_URL=postgres://username:password@localhost/rust_sample_api`
4. Run migrations with `diesel migration run`
5. Manually insert data into posts table
6. Visit localhost:3000/posts and localhost:3000/posts/1