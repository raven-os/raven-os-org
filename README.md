# Website for Raven-OS

This is the website of raven-os, used to communicate with customers.
It's back office uses rocket.

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

Set the **DATABASE_URL** environment variable, the **ADMIN_TOKEN** and
the file extension whitelist

```
$ echo DATABASE_URL=mysql://username:password@localhost/db_name > .env
$ echo ADMIN_TOKEN=secret_token_for_admin_access >> .env
$ echo WHITELIST=html/css/js/eot/ttf/woff/woff2/svg/jpeg/jpg/png/gif/bmp/ico/mp3/mp4/avi/mkv/wmv >> .env


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

### Configuration

Copy the `Rocket.toml.example` file and modify it as you wish

```
$ cp Rocket.toml.example Rocket.toml`
```

For the `secret_key` you can get a 256-bit base64 encoded string using openssl.

```
$ openssl rand -base64 32
```

### Run

```
$ cargo run
```

Rocket run with the development configuration by default.
If you want to change it, use the `ROCKET_ENV` environment variable.

```
$ ROCKET_ENV=stage cargo run
$ ROCKET_ENV=prod cargo run
```

You'll likely need `sudo` if you listen on port `80`.

### Documentation

```
$ apidoc -i src -o docs -f ".*\\.rs$"
$ firefox docs/index.html
```

### Template

Template root folder is `front/`
Every path in `href` should be absolute if you are in a subfolder.
You can include template with `{{> template_name}}`
Example for *front/subfolder/index.html.hbs*:
```
<!DOCTYPE html>
<html lang="en">
<head>
    <link href="/img/RavenIco.ico" rel="icon">
</head>
<body>
    {{> header}}
    Hello
    {{> footer}}
</body>
</html>

```
