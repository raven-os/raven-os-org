FROM rust

EXPOSE 80

# Install build tools
RUN rustup default nightly
RUN cargo install diesel_cli --no-default-features --features mysql

# Copy app and set working directory
COPY . /app
WORKDIR /app

# Compile
RUN cargo build --release

# Setup environnement and run
#
# You may want to edit these values, these are only default ones
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT=80
ENV DATABASE_URL="mysql://root:root@127.0.0.1/raven-os-org"
ENV RAVEN_ADMIN_TOKEN=admin_token
CMD diesel setup && diesel migration run && cargo run --release
