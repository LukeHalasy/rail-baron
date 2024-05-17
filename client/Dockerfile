FROM rust:1.75.0

WORKDIR /usr/src/app

RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown

RUN export PATH="$HOME/.cargo/bin:$PATH"
RUN cargo install trunk --version=0.17.5

COPY client ./client
COPY server ./server
COPY store ./store

WORKDIR /usr/src/app/client

EXPOSE 8088

CMD ["trunk", "serve", "--release", "--watch", "."]
