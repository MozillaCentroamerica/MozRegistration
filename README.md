# MozRegistration

### Setup Rust
- Set rust to nightly `rustup default nightly`
- Update cargo `rustup update && cargo update`
### Install Diesel Cli
`cargo install diesel_cli`
### Setup Database
- Set DATABASE_URL Env `export DATABASE_URL=mysql://user:pass@localhost/personas`
- Create database `diesel setup`
- Run migrations `diesel migration run`
### Run 
- `cargo run`



