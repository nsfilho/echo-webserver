FROM ubuntu:latest

WORKDIR /app
COPY target/release/echo ./

ENTRYPOINT ["/app/echo"]
