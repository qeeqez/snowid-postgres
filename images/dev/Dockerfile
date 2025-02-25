FROM postgres:17-bookworm AS builder

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

# install pgrx
ARG PGRX_VER=0.12.9
RUN $HOME/.cargo/bin/cargo install cargo-pgrx --version=$PGRX_VER --locked
RUN $HOME/.cargo/bin/cargo pgrx init --pg17 $(which pg_config)

WORKDIR /snowid
COPY Cargo.toml Cargo.lock pg_snowid.control ./
COPY images/dev/dummy.rs ./src/lib.rs
COPY src/bin/pgrx_embed.rs ./src/bin/pgrx_embed.rs
RUN $HOME/.cargo/bin/cargo build

COPY src ./src

# install snowid
RUN $HOME/.cargo/bin/cargo pgrx install --pg-config=$(which pg_config)

FROM ghcr.io/cloudnative-pg/postgresql:17.4-bookworm

USER root

COPY --from=builder /usr/share/postgresql/17/extension /usr/share/postgresql/17/extension
COPY --from=builder /usr/lib/postgresql/17/lib /usr/lib/postgresql/17/lib

#COPY images/pg/postgresql.conf /usr/share/postgresql/17/postgresql.conf.sample

RUN usermod -u 26 postgres
USER 26