FROM rust:slim-buster

ARG USER_ID
ARG GROUP_ID

RUN addgroup --gid $GROUP_ID user
RUN adduser --disabled-password --gecos '' --uid $USER_ID --gid $GROUP_ID user

# install cargo
RUN apt update
RUN apt install -y cargo libssl-dev

# backend
RUN cargo install sea-orm-cli

# frontend
RUN cargo install --locked trunk

RUN mkdir -p \
    /opt/app/backend/src \
    /opt/app/frontend/src

WORKDIR /opt/app

# common
ADD ./common common

# add dummy backend lib
ADD ./backend/Cargo.toml ./backend/Cargo.toml
RUN echo "// dummy file" > ./backend/src/lib.rs

# add dummy frontend lib
ADD ./frontend/Cargo.toml ./frontend/Cargo.toml
RUN echo "// dummy file" > ./frontend/src/lib.rs

RUN rustup target add wasm32-unknown-unknown

# build core dependencies
ADD ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN chown user:user /opt/app -R

USER user