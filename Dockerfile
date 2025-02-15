FROM rust:latest

WORKDIR /usr/src/shooting/server
COPY . ../

RUN cargo install --path .

CMD ["shooting_server"]
