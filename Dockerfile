FROM rust:latest

RUN mkdir app

WORKDIR /app

COPY . /app/

RUN cargo install cargo-watch

#EXPOSE 7777

