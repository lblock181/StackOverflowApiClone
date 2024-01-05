# StackOverflow Api Clone
Clone of StackOverflow API created in Rust. This follows the API project from the Rust Developer Bootcamp

This project is a demo project for building an API using Rust. The project is a Stackoverflow style application with the following features
- Question CRUD
- Answer CRUD

Built With
- Rust
- Rocket
- Postgres

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