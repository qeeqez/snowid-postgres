ARG PG_MAJOR=18
ARG PG_MINOR=1
ARG TARGETARCH

FROM ghcr.io/cloudnative-pg/postgresql:${PG_MAJOR}.${PG_MINOR}-standard-trixie

USER root
COPY artifacts/${TARGETARCH}/pg_snowid-pg${PG_MAJOR} /

RUN usermod -u 26 postgres
USER 26