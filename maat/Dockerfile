FROM rust:1.38 AS Build

WORKDIR /maat/

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() { }" > src/main.rs
RUN cargo build --release

RUN rm ./target/release/deps/maat*

COPY src src

RUN cargo install --path . --offline

FROM rust:1.38 AS Runtime

COPY --from=Build /usr/local/cargo/bin/maat maat

CMD ["./maat"]
