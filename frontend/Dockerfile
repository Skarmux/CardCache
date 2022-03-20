FROM rust:alpine3.15

RUN apk update && apk add \
    cargo \
    pkgconfig

WORKDIR /app/frontend

RUN cargo install --locked trunk
RUN cargo install wasm-bindgen-cli

# from Makefile:
# tailwindcss -c tailwind.config.js -i tailwind.in.css -o tailwind.css --minify
CMD ["cargo", "run", "trunk", "serve", "--release", "--watch", "tailwind.css"]