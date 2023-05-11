FROM rust:1.68

# RUN apt-get clean && apt-get update && \
#     apt-get install -y --no-install-recommends \
#            postgresql-client-common \
#            postgresql-client \
#            netcat-traditional

ENV CARGO_TARGET_DIR=/target

RUN cargo install cargo-watch

WORKDIR /app

COPY . /app

# EXPOSE 8080

CMD ["/app/docker/local/entrypoint.sh"]
