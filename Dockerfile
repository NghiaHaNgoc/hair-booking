FROM rust:latest

WORKDIR /hair-booking
COPY . .
RUN cargo install --path .

EXPOSE 8080

CMD ["hair-booking"]

