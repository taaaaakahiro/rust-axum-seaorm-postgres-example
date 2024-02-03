# rust-axum-seaorm-postgres

## setup
```shell
$ cargo install sea-orm-cli
$ sea-orm-cli migrate init

# migrate
$ cd ./migration
$ sea-orm-cli migrate generate <migration_file_name>
$ cargo run -- refresh

# create model
$ cd ./ #project root
$ rust-axum-seaorm-postgres % sea-orm-cli generate entity \                   
    -u postgresql://postgres:postgres@localhost:5432/example \
    -o ./src/domain/model
```

## reference
- https://zenn.dev/collabostyle/articles/0641d73f776d80