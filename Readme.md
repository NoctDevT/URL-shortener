# URL Shortener

Using Docker compose to containerise the Rust and Db layers

## Development

Usin





Run with `cargo run --features dotenv`.

Start application with:

```shell
docker-compose up --build



docker build -t url-shortener-db:latest database
docker run --rm --env POSTGRES_PASSWORD=password url-shortener-db:latest
```
