ARG PG_MAJOR=18
ARG PG_MINOR=1
ARG PGRX=0.18.0

FROM postgres:${PG_MAJOR}.${PG_MINOR}-trixie AS builder

ARG PG_MAJOR
ARG PGRX

# Grouped apt-get commands to reduce image layers
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl gnupg \
    clang gcc git libssl-dev make pkg-config && \
    install -d /usr/share/postgresql-common/pgdg && \
    curl -o /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc --fail https://www.postgresql.org/media/keys/ACCC4CF8.asc && \
    . /etc/os-release && \
    sh -c "echo 'deb [signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $VERSION_CODENAME-pgdg main' > /etc/apt/sources.list.d/pgdg.list" && \
    apt-get update && \
    apt-get install -y --no-install-recommends postgresql-server-dev-${PG_MAJOR}

# Install Rust dependencies
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN $HOME/.cargo/bin/rustup default stable

# Use cache mounts to massively speed up Cargo compilation
RUN --mount=type=cache,id=cloudnative-pg-cargo-registry,target=/root/.cargo/registry,sharing=locked \
    --mount=type=cache,id=cloudnative-pg-cargo-git,target=/root/.cargo/git,sharing=locked \
    $HOME/.cargo/bin/cargo install cargo-pgrx --version=${PGRX} --locked

RUN $HOME/.cargo/bin/cargo pgrx init --pg${PG_MAJOR} $(which pg_config)

WORKDIR /app
COPY Cargo.toml Cargo.lock pg_snowid.control ./
# Pre-build dependencies to cache them in a Docker layer
RUN mkdir src && echo '' > src/lib.rs && \
    mkdir sql && \
    $HOME/.cargo/bin/cargo build --release

# Copy actual source code
COPY src ./src
COPY sql ./sql

# Build the actual extension (dependencies are already cached)
RUN touch src/lib.rs && \
    $HOME/.cargo/bin/cargo pgrx install --pg-config=$(which pg_config) --release

FROM ghcr.io/cloudnative-pg/postgresql:${PG_MAJOR}.${PG_MINOR}-standard-trixie

ARG PG_MAJOR

USER root

COPY --from=builder /usr/share/postgresql/${PG_MAJOR}/extension /usr/share/postgresql/${PG_MAJOR}/extension
COPY --from=builder /usr/lib/postgresql/${PG_MAJOR}/lib /usr/lib/postgresql/${PG_MAJOR}/lib

RUN usermod -u 26 postgres
USER 26