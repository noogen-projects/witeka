# Witeka

The Witeka web application.

## Development notes

### Dependencies and tools

Setup dev dependencies and tools:

```shell script
sudo apt install mariadb-server libmysqlclient20 libmysqlclient-dev libssl-dev
cargo install diesel_cli --no-default-features --features mysql
cargo install wasm-bindgen-cli
```

### Database

Create database and apply migrations:

```shell script
sudo mysql -u root < db/create_db.sql
diesel migration run
```

Create test database and apply migrations:

```shell script
sudo mysql -u root < db/create_test_db.sql
./diesel_test.sh migration run
```

### Build

Check the project:

```shell script
cargo check --all-features --all-targets
```

Build and run:

```shell script
./build_wasm_client.sh
cargo run -p witeka-server
```

### Testing

Run all unit and integration tests:

```shell script
cargo test --all-features --all-targets
```

Add user via REST API:

```shell script
curl --header "Content-Type: application/json" --request POST --data '"Vasya"' http://localhost:8080/user
```

### Developer environment

Check and perform formatting:

```shell script
cargo +nightly fmt -- --check
cargo +nightly fmt
```

Enable autoformatting for IntelliJ IDEA with the Rust plugin:

`Settings -> Languages and Frameworks -> Rust -> Rustfmt -> Run rustfmt on Save`

Run clippy:

```shell script
cargo clippy --all-targets --workspace --all-features -- -D warnings
```

Setup git hook:

```shell script
cp .git-pre-push.sh .git/hooks/pre-push
```