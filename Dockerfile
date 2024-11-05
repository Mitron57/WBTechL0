FROM rust:slim-bullseye

WORKDIR /root/wb_tech_l0

COPY ./src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY migrations migrations
COPY startup.sh startup.sh

ARG DATABASE_URL
ARG RUST_LOG
ENV DATABASE_URL=${DATABASE_URL}
ENV RUST_LOG=${RUST_LOG}

RUN cargo install refinery_cli
RUN cargo build --release

RUN chmod +x startup.sh

CMD ["./startup.sh"]
