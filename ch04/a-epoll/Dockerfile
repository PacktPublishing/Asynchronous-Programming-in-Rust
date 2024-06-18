FROM rust:1-slim-bookworm

WORKDIR /usr/a-epoll

COPY . .

RUN cargo build

CMD ["cargo", "run", "delayserver"]
