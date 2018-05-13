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
# You may want to edit these values
ENV ROCKET_ADDRESS="0.0.0.0"
ENV ROCKET_PORT=80
ENV DATABASE_URL="mysql://root:root@localhost/db_name"
ENV ADMIN_TOKEN=[enter a admin token]
RUN echo "DATABASE_URL=$DATABASE_URL" > .env
RUN echo "ADMIN_TOKEN=$ADMIN_TOKEN" >> .env
RUN echo "WHITELIST=html/css/js/eot/ttf/woff/woff2/svg/jpeg/jpg/png/gif/bmp/ico/mp3/mp4/avi/mkv/wmv" >> .env
CMD diesel setup && diesel migration run && cargo run --release