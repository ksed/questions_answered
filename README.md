# Questions Answered

## To Start PostgreSQL:

1. Copy `/assets/postgres-setup/` files somewhere outside the project directory.
2. Run `docker-compose up`.
3. Wait for the postgres server to initialize, and which will create `/data` folder.
4. In another terminal run `cat setup_tables.sh | docker exec -i postgresql bash` (optional with migrations).
5. If the `setup_tables.sh` command is successful you should see two `CREATE TABLE` statements.
6. Press Ctrl-C to exit the docker-exec context.
 
Postgres will now be populated with both the questions and answers tables (blank).

## To quit PostgreSQL:

1. In the terminal with the PostgreSQL logs, press Ctrl-C to exit.
2. Run `docker-compose down` to remove the container.
3. You can also delete the `/data` folder to purge the database history.

## Run Questions Answered

1. In Rust project folder `cargo run`.
2. Open http://localhost:3030/questions to see the results.
