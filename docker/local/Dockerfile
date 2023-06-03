FROM rust:1.68

RUN apt-get clean && apt-get update && \
    apt-get install -y --no-install-recommends \
           postgresql-client-common \
           postgresql-client \
           netcat-traditional

ENV CARGO_TARGET_DIR=/target
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN cargo install cargo-watch
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

WORKDIR /app

COPY . /app

EXPOSE 8000

CMD ["/app/docker/local/entrypoint.sh"]
