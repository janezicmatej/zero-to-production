FROM rust:1.68

RUN apt-get clean && apt-get update && \
    apt-get install -y --no-install-recommends \
           postgresql-client-common \
           postgresql-client \
           netcat-traditional

# install deps outside of mounted directory
ENV CARGO_TARGET_DIR=/target
# use crates.io for registry
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# install necessary cli tools
RUN cargo install cargo-watch
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

WORKDIR /app

COPY . /app

EXPOSE 8000

CMD ["/app/docker/local/entrypoint.sh"]
