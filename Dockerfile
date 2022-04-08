FROM rust:slim-buster

ARG USER_ID
ARG GROUP_ID

RUN addgroup --gid $GROUP_ID user
RUN adduser --disabled-password --gecos '' --uid $USER_ID --gid $GROUP_ID user

RUN rustup target add wasm32-unknown-unknown

# install cargo
RUN apt update
RUN apt install -y cargo libpq-dev

# backend
RUN cargo install diesel_cli --no-default-features --features postgres

# frontend
RUN cargo install --locked trunk

RUN mkdir -p \
    /opt/app/backend/src \
    /opt/app/frontend/src \
    /opt/app/common/src

WORKDIR /opt/app

# common
ADD ./common common

# add dummy backend lib
ADD ./backend/Cargo.toml ./backend/Cargo.toml
RUN echo "// dummy file" > ./backend/src/lib.rs

# add dummy frontend lib
ADD ./frontend/Cargo.toml ./frontend/Cargo.toml
RUN echo "// dummy file" > ./frontend/src/lib.rs

# build core dependencies
ADD ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN chown user:user /opt/app -R
USER user