# StackOverflow Api Clone
Clone of StackOverflow API created in Rust. This follows the API project from the Rust Developer Bootcamp

This project is a demo project for building an API using Rust. The project is a Stackoverflow style application with the following features
- Question CRUD
- Answer CRUD

Built With
- Rust
- Rocket
- PostgreSQL
- Docker (for hosting PostgreSql db)

<br><hr><br>

## Development
Development occurs in three stages

### Stage 1 -> API Endpoints & Models
- Configuring environment
    - cargo install required crates
- Building API models and endpoints
- stub endpoints built

### Stage 2 -> Persistance & DAO
- install/build Postgres db
- connect to db via Rust
    - db connector built to accept various dbs
- DAOs built for questions & answers

### Stage 3 -> Endpoints to DAOs
- connect DAOs to the api endpoints

<br><hr><br>

## Running Service
1. Clone the repository
1. If necessary, install
    - [Rust](https://www.rust-lang.org/tools/install)
    - [Docker](https://docs.docker.com/engine/install/)
    - [PostgreSQL](https://www.postgresql.org/download/)
        - Not necessary to run tool as Docker container will host PostgreSql database
1. Install [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) for interacting with the database
1. Start Docker container by running the following in the terminal
    ```bash
    read -p "Enter name for postgres db" POSTGRES_DB_NAME
    read -p "Enter password for postgres db" POSTGRES_PASSWORD
    docker run --name $POSTGRES_DB_NAME -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD -p 5432:5432 -d postgres
    ```
1. Create a `.env` file in the root of the directory and add the following KEY=VALUE pair
    - DATABASE_URL=postgres://postgres:POSTGRESPASSWORD@localhost:POSTGRESPORT
1. Run the following `sqlx` command to create the tables in the database. Note you must be in the root project directory.
    ```bash
    sqlx migrate run
    ```
1. Run the Rust service
    1. Running in terminal
        - Run the following command in the terminal
            ```bash
            cargo run
            ```
    1. Running as binary
        - Run the following command in the terminal to build an optimized version of the tool
            ```bash
            cargo build --release
            ```
        - Navigate to the `target->release->stackoverflowapi` executable and run it
1. Once the service is running, you can use the browser, cURL, or any other API testing tool to interact with the service
    - Ensure that the Docker container is running prior to launching the service