FROM rust:slim-buster

# install cargo
RUN apt update 
RUN apt install -y cargo

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /opt/app

ADD ./common /opt/app/common

# add dummy backend lib
ADD ./backend/Cargo.toml ./backend/Cargo.toml
RUN mkdir backend/src && echo "// dummy file" > backend/src/lib.rs

# add dummy frontend lib
ADD ./frontend/Cargo.toml ./frontend/Cargo.toml
RUN mkdir frontend/src && echo "// dummy file" > frontend/src/lib.rs

# build core dependencies
ADD Cargo.toml ./Cargo.toml
RUN cargo build
