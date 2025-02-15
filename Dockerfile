FROM rust:1.79.0
WORKDIR /app
ADD . .
ENV ROCKET_PORT=8080
ENV ROCKET_ADDRESS=0.0.0.0
RUN cargo install --path .

EXPOSE 8080

CMD ["config_server", "-c", "/config/..data/"]
