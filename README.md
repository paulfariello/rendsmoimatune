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

Configuration
=============

Server configuration is available in ``server/Rocket.toml``.

Run
===

```sh
cd server/; cargo run
cd client/; trunk serve --proxy-backend=http://127.0.0.1:8000/api/
```

Then open http://127.0.0.1:8080/

Dev
===

```sh
docker pull postgres
docker run --name rmmt-postgres -e POSTGRES_DB=rmmt -e POSTGRES_PASSWORD=password -p 127.0.0.1:5432:5432/tcp -d postgres
```
