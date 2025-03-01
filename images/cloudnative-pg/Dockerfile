ARG PG_MAJOR
ARG PG_MINOR
ARG PGRX

FROM postgres:${PG_MAJOR}.${PG_MINOR}-bookworm AS builder

ARG PG_MAJOR
ARG PGRX

RUN apt-get update \
    && apt-get install -y \
	ca-certificates \
	clang \
	curl \
	gcc \
	git \
	libssl-dev \
	make \
	pkg-config \
	postgresql-server-dev-17

# Install Rust dependencies
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN $HOME/.cargo/bin/rustup default stable

RUN $HOME/.cargo/bin/cargo install cargo-pgrx --version=${PGRX} --locked
RUN $HOME/.cargo/bin/cargo pgrx init --pg${PG_MAJOR} $(which pg_config) --release

COPY Cargo.toml Cargo.lock pg_snowid.control ./
COPY src ./src

# install snowid
RUN $HOME/.cargo/bin/cargo pgrx install --pg-config=$(which pg_config) --release

FROM ghcr.io/cloudnative-pg/postgresql:${PG_MAJOR}.${PG_MINOR}-bookworm

ARG PG_MAJOR

USER root

COPY --from=builder /usr/share/postgresql/${PG_MAJOR}/extension /usr/share/postgresql/${PG_MAJOR}/extension
COPY --from=builder /usr/lib/postgresql/${PG_MAJOR}/lib /usr/lib/postgresql/${PG_MAJOR}/lib

USER 26