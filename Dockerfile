FROM rust:latest

WORKDIR /hair-booking
COPY . .
RUN cargo install --path .
RUN cargo clean

EXPOSE 8080

CMD ["hair-booking"]

