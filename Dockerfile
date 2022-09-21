FROM ubuntu:latest

EXPOSE 8000/tcp

WORKDIR /app
COPY target/release/echo /app
COPY Rocket.toml /app

ENTRYPOINT ["/app/echo"]
