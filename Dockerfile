FROM rust:slim-buster

# install cargo
RUN apt update 
RUN apt install -y cargo

WORKDIR /opt/app

# add dummy common lib
ADD ./common/Cargo.toml ./common/Cargo.toml
RUN mkdir common/src && echo "// dummy file" > common/src/lib.rs

# add dummy backend lib
ADD ./backend/Cargo.toml ./backend/Cargo.toml
RUN mkdir backend/src && echo "// dummy file" > backend/src/lib.rs

# add dummy frontend lib
ADD ./frontend/Cargo.toml ./frontend/Cargo.toml
RUN mkdir frontend/src && echo "// dummy file" > frontend/src/lib.rs

# build core dependencies
ADD Cargo.toml ./Cargo.toml
RUN cargo build

# run backend server
# CMD ["cargo", "run"]

# run frontend webserver
CMD [ "trunk", "serve", "--release", "--watch", "tailwind.css" ]