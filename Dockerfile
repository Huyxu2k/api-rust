#build
FROM rust:1.71-buster as builder

WORKDIR /app

#arg
ARG DATABASE_URL
ARG APP_ADDRESS
ARG APP_PORT

#env database
ENV DATABASE_URL=$DATABASE_URL
ENV APP_ADDRESS=$APP_ADDRESS
ENV APP_PORT=$APP_PORT

COPY . .

RUN cargo build --release

#enviroment run
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/api-rust .

CMD [ "./api-rust" ]