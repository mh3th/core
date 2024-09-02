# auto-reload with systemfd and cargo-watch

## Install cargo-watch systemfd
```bash
cargo install cargo-watch systemfd
```

## Running by systemfd and cargo-watch
```bash
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

# sqlx-cli

## Install sqlx-cli
```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

## Create the database
```bash
sqlx database create --database-url postgres://admin:admin@localhost/iam_service_db
```

## Migrate the database
```bash
sqlx migrate run --source .\iam_service\migrations\ --database-url postgres://postgres:password@localhost/iam_service_db
```
or in service directory
```bash
sqlx migrate run
```

## Prepare the database
```bash
sqlx prepare --database-url postgres://postgres:password@localhost/iam_service_db
```

## Create a new migration
```bash
sqlx migrate add -r created_fixme_table --source .\iam_service\migrations\
```
OR in service directory
```bash
 sqlx migrate add -r created_account_event_types_table
```