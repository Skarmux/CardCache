FROM rust:slim-buster

RUN apt update && apt install -y cargo

WORKDIR /app/backend

# RUN cargo install

CMD ["cargo", "run"]