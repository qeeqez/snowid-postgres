ARG PG_MAJOR=18
ARG PG_MINOR=1

FROM postgres:${PG_MAJOR}.${PG_MINOR}-trixie

ARG PG_MAJOR
ARG TARGETARCH

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \

COPY artifacts/${TARGETARCH}/ /

COPY images/pg/postgresql.conf /usr/share/postgresql/${PG_MAJOR}/postgresql.conf.sample

USER postgres
CMD ["postgres"]