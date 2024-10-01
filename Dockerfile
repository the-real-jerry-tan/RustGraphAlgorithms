FROM rust:1.56
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release
CMD ["./target/release/graph_algorithms"]
