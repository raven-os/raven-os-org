# Website's API

This is the API for Raven Website. It handles things like newsletter

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

Use a nightly version of rust
```
$ rustup default nightly
```

Install `diesel_cli` a tool that handle database operations
```
& cargo install diesel_cli --no-default-features --features mysql
```

### Installing

Set the **DATABASE_URL** environment variable

```
$ echo DATABASE_URL=mysql://username:password@localhost/db_name > .env
```

Setup the database

```
$ diesel setup
```

Apply migration

```
$ diesel migration run
```

### Migration

Generate a migration

```
$ diesel migration generate <name>
```

### Run

```
$ cargo run
```

#### Endpoints

**GET** `localhost:8000/email/<email>`

**GET** `localhost:8000/email/delete/<email>/<token>`
