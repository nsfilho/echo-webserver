FROM ubuntu:latest

WORKDIR /app
COPY target/release/echo /app

ENTRYPOINT ["/app/echo"]
