FROM rust:latest
WORKDIR /usr/src/letterboxd-api
COPY . .
RUN cargo build --release
CMD ["./target/release/letterboxd-api"]