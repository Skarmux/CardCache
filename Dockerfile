FROM rust:slim-buster

ARG USER_ID
ARG GROUP_ID

RUN addgroup --gid $GROUP_ID user
RUN adduser --disabled-password --gecos '' --uid $USER_ID --gid $GROUP_ID user

# install cargo
RUN apt update
RUN apt install -y cargo libssl-dev pkg-config

RUN mkdir -p \
    /opt/app/backend/src \
    /opt/app/backend/entity/src \
    /opt/app/backend/migration/src \
    /opt/app/frontend/src

WORKDIR /opt/app

# add dummy backend lib
COPY ./backend/Cargo.toml ./backend/Cargo.toml
RUN echo "// dummy file" > ./backend/src/lib.rs
COPY ./backend/entity/Cargo.toml ./backend/entity/Cargo.toml
RUN echo "// dummy file" > ./backend/entity/src/lib.rs
COPY ./backend/migration/Cargo.toml ./backend/migration/Cargo.toml
RUN echo "// dummy file" > ./backend/migration/src/lib.rs

# add dummy frontend lib
ADD ./frontend/Cargo.toml ./frontend/Cargo.toml
RUN echo "// dummy file" > ./frontend/src/lib.rs

RUN rustup target add wasm32-unknown-unknown

# dependency for sea-rom-cli
RUN rustup component add rustfmt

# common
COPY ./common common

# build core dependencies
COPY ./Cargo.toml ./Cargo.toml

RUN chown user:user /opt/app -R

USER user

RUN cargo fetch

# backend
RUN cargo install sea-orm-cli

# frontend
RUN cargo install --locked trunk

RUN cargo build --release
