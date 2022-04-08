FROM rust:slim-buster

# install cargo
RUN apt update 
RUN apt install -y cargo libpq-dev

# RUN groupadd 1000
# RUN useradd -m -u 1000 -g 1000 -s /bin/bash dev
# USER dev

# backend
#RUN apt install -f libpg-dev
RUN cargo install diesel_cli --no-default-features --features postgres

# frontend
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

ADD ./common /opt/app/common

# add dummy backend lib
ADD ./backend/Cargo.toml /opt/app/backend/Cargo.toml
RUN mkdir /opt/app/backend/src && echo "// dummy file" > /opt/app/backend/src/lib.rs

# add dummy frontend lib
ADD ./frontend/Cargo.toml /opt/app/frontend/Cargo.toml
RUN mkdir /opt/app/frontend/src && echo "// dummy file" > /opt/app/frontend/src/lib.rs

# build core dependencies
ADD Cargo.toml ./Cargo.toml
RUN cargo build

