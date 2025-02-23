FROM postgres:@@PG_VERSION@@-bookworm as builder

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
	postgresql-server-dev-@@PG_VERSION@@


# Install snowid
COPY . snowid
RUN cd snowid && make && make install

FROM postgres:@@PG_VERSION@@-bookworm

COPY --from=builder /usr/share/postgresql/@@PG_VERSION@@/extension /usr/share/postgresql/@@PG_VERSION@@/extension
COPY --from=builder /usr/lib/postgresql/@@PG_VERSION@@/lib /usr/lib/postgresql/@@PG_VERSION@@/lib

RUN apt-get update \
    && apt-get install -y ca-certificates

COPY postgresql.conf /usr/share/postgresql/@@PG_VERSION@@/postgresql.conf.sample

USER postgres
CMD ["postgres"]