# Raven-OS's Landing page

This is Raven-OS's landing page, used as an easy way to present our project, ideas and philosophy to the community.

It is based on Rocket.

## Getting Started

These instructions will get your own copy Raven-OS's landing page up and running on your local machine for development and testing purposes.

See deployment for notes on how to deploy the project on a live system.

### Prerequisites

Use a nightly version of rust
```bash
$ rustup default nightly
```

Install `diesel_cli` a tool that handle database operations
```bash
$ cargo install diesel_cli --no-default-features --features mysql
```

For Raven-OS's landing page to work, you will need to set some environment variables. You can add them by hand or put them in a `.env` file. The easiest way to do so is by copying the given example:

```bash
$ cp .env.example .env # You can edit .env as you wish
```

### Installing

Setup the database

```bash
$ diesel setup
$ diesel migration run
```

### Configuration

You can have a look at [Rocket's documentation](https://rocket.rs/guide/configuration/#rockettoml) to see how to configure a `Rocket.toml`. Default settings should be find for debugging purposes tho'

### Run

```bash
$ cargo run
```

You can tweak the default ip/port with some environment variables (More informations [here](https://rocket.rs/guide/configuration/#rockettoml):

```bash
$ ROCKET_ADDRESS=127.0.0.1 ROCKET_PORT=80 cargo run
```

You may need `sudo` if you want to listen on port `80`.

### Documentation

To generate the backend's API documentation, run the following command:

```bash
$ apidoc -i src -o docs -f ".*\\.rs$"
```

The documentation will be placed in `docs/`

### Template

Front-end static files are placed in `static/`. Templates are placed in `templates/`.
Every path of static files should be absolute from the static folder. That is, for `static/img/raven.png`, you should use the `/img/raven.png` path.

Example for `templates/subfolder/index.html.hbs`:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <link href="/img/raven_ico.ico" rel="icon">
</head>
<body>
    {{> header}}
    Hello
    {{> footer}}
</body>
</html>

```
