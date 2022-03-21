Run
===

```
cargo run --manifest-path server/Cargo.toml
trunk serve --proxy-backend=http://127.0.0.1:8000/api/ client/Cargo.toml
```

Then open http://127.0.0.1:8080/
