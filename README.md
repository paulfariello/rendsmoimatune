Dependencies
============

```sh
cargo install diesel_cli --no-default-features --features postgres
cargo install --locked trunk
```

Installation
============

```sh
echo DATABASE_URL=postgres://user:password@host/database > server/.env
diesel migration generate init
diesel migration run
```

Run
===

```sh
cargo run --manifest-path server/Cargo.toml
cd client/; trunk serve --proxy-backend=http://127.0.0.1:8000/api/
```

Then open http://127.0.0.1:8080/
