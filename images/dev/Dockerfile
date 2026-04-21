ARG PG_MAJOR=18
ARG PG_MINOR=1
ARG PGRX=0.18.0

FROM postgres:${PG_MAJOR}.${PG_MINOR}-trixie AS builder

ARG PG_MAJOR
ARG PGRX

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl gnupg \
    clang gcc git libssl-dev make pkg-config && \
    install -d /usr/share/postgresql-common/pgdg && \
    curl -o /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc --fail https://www.postgresql.org/media/keys/ACCC4CF8.asc && \
    . /etc/os-release && \
    sh -c "echo 'deb [signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $VERSION_CODENAME-pgdg main' > /etc/apt/sources.list.d/pgdg.list" && \
    apt-get update && \
    apt-get install -y --no-install-recommends postgresql-server-dev-${PG_MAJOR}

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default stable

# Install pgrx with cache
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    cargo install cargo-pgrx --version=${PGRX} --locked

RUN cargo pgrx init --pg${PG_MAJOR} $(which pg_config)

WORKDIR /snowid
COPY Cargo.toml Cargo.lock pg_snowid.control ./

# Pre-build dependencies
RUN mkdir src && echo 'fn main() {}' > src/lib.rs && \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/snowid/target \
    cargo build --release

COPY src ./src

# Build extension
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/snowid/target \
    cargo pgrx install --pg-config=$(which pg_config) --release

FROM postgres:${PG_MAJOR}.${PG_MINOR}-trixie

ARG PG_MAJOR

COPY --from=builder /usr/share/postgresql/${PG_MAJOR}/extension /usr/share/postgresql/${PG_MAJOR}/extension
COPY --from=builder /usr/lib/postgresql/${PG_MAJOR}/lib /usr/lib/postgresql/${PG_MAJOR}/lib

COPY images/pg/postgresql.conf /usr/share/postgresql/${PG_MAJOR}/postgresql.conf.sample

USER postgres
CMD ["postgres"]