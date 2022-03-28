FROM rust:slim-buster

# install cargo
RUN apt update 
RUN apt install -y cargo

# build core dependencies
ADD Cargo.toml /tmp/Cargo.toml
RUN cd /tmp
RUN mkdir src && echo "// dummy file" > src/lib.rs
RUN cargo build

# build common library
ADD ./common /tmp/common
RUN cd /tmp/common && cargo build

WORKDIR /opt/app

CMD [ "trunk", "serve", "--release", "--watch", "tailwind.css" ]