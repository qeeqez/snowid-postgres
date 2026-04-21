ARG PG_MAJOR=18
ARG PG_MINOR=1
ARG PGRX=0.18.0

FROM postgres:${PG_MAJOR}.${PG_MINOR}-trixie AS builder

ARG PG_MAJOR=18
ARG PGRX=0.18.0

RUN apt-get update && apt-get install -y ca-certificates curl && \
	install -d /usr/share/postgresql-common/pgdg && \
	curl -o /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc --fail https://www.postgresql.org/media/keys/ACCC4CF8.asc && \
	. /etc/os-release && \
	sh -c "echo 'deb [signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $VERSION_CODENAME-pgdg main' > /etc/apt/sources.list.d/pgdg.list" && \
	apt-get update

RUN apt-get install -y \
	clang \
	gcc \
	git \
	libssl-dev \
	make \
	pkg-config
RUN apt-get install -y postgresql-server-dev-${PG_MAJOR}

# Install Rust dependencies
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN $HOME/.cargo/bin/rustup default stable

RUN $HOME/.cargo/bin/cargo install cargo-pgrx --version=${PGRX} --locked
RUN $HOME/.cargo/bin/cargo pgrx init --pg${PG_MAJOR} $(which pg_config)

WORKDIR /snowid
COPY Cargo.toml Cargo.lock pg_snowid.control ./
COPY images/dev/dummy.rs ./src/lib.rs
RUN $HOME/.cargo/bin/cargo build

COPY src ./src

# install snowid
RUN $HOME/.cargo/bin/cargo pgrx install --pg-config=$(which pg_config)

FROM postgres:${PG_MAJOR}.${PG_MINOR}-trixie

ARG PG_MAJOR

COPY --from=builder /usr/share/postgresql/${PG_MAJOR}/extension /usr/share/postgresql/${PG_MAJOR}/extension
COPY --from=builder /usr/lib/postgresql/${PG_MAJOR}/lib /usr/lib/postgresql/${PG_MAJOR}/lib

COPY images/pg/postgresql.conf /usr/share/postgresql/${PG_MAJOR}/postgresql.conf.sample

USER postgres
CMD ["postgres"]