FROM rust:1-slim-bookworm

WORKDIR /usr/delayserver

COPY . .

RUN cargo build

CMD ["cargo", "run", "delayserver"]
