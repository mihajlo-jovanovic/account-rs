# Rust Axum Demo
Demo of RESTFull app in Rust using [Axum](https://github.com/tokio-rs/axum)

## To run (locally):
First, start Postgres database, for example:
```
postgres -D /usr/local/var/postgresql@14
```

Then, set env var with connection info:

```
export DATABASE_URL=postgres://localhost/diesel_demo
```

Finally, run code:
```
cargo run
```
You can see the results here: http://localhost:4000/account/list

## To Do
[] Organise code: move models, routes out of main

[] http-tower component test