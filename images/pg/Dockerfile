FROM postgres:17.4-bookworm AS builder

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

COPY Cargo.toml Cargo.lock pg_snowid.control ./
COPY src ./src

# install snowid
RUN $HOME/.cargo/bin/cargo pgrx install --pg-config=$(which pg_config)

FROM postgres:17.4-bookworm

COPY --from=builder /usr/share/postgresql/17/extension /usr/share/postgresql/17/extension
COPY --from=builder /usr/lib/postgresql/17/lib /usr/lib/postgresql/17/lib

RUN apt-get update \
    && apt-get install -y ca-certificates

COPY images/pg/postgresql.conf /usr/share/postgresql/17/postgresql.conf.sample

USER postgres
CMD ["postgres"]