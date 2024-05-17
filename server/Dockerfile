FROM rust:1.75.0

WORKDIR /usr/src/app

RUN rustup toolchain install nightly
RUN rustup default nightly

COPY client ./client
COPY server ./server
COPY store ./store

WORKDIR /usr/src/app/server
RUN cargo install --path .

EXPOSE 8000
CMD ["cargo", "run"]