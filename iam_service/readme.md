# auto-reload with systemfd and cargo-watch

## Install cargo-watch systemfd
```sh
cargo install cargo-watch systemfd
```

## Running by systemfd and cargo-watch
```sh
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

# sqlx-cli

## Install sqlx-cli
```sh
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

## Create the database
```bash
cargo sqlx database create --database-url postgres://admin:admin@localhost/iam_service_db
```

## Migrate the database
```bash
cargo sqlx migrate run --source .iam_service\migrations\iam_service\ --database-url postgres://postgres:password@localhost/iam_service_db
```

## Prepare the database
```bash
cargo sqlx prepare --database-url postgres://postgres:password@localhost/iam_service_db
```

## Create a new migration
```bash
cargo sqlx migrate add -r created_fixme_table --source .iam_service\migrations\iam_service_db\
```