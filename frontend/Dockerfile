FROM rust:slim-buster

RUN apt update && apt install -y cargo

WORKDIR /app/frontend

RUN cargo install --locked trunk
RUN cargo build
#RUN cargo install wasm-bindgen-cli

CMD ["trunk", "serve", "--release", "--watch", "tailwind.css"]