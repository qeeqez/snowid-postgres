ARG PG_MAJOR=18
ARG PG_MINOR=1

FROM ghcr.io/cloudnative-pg/postgresql:${PG_MAJOR}.${PG_MINOR}-standard-trixie

ARG TARGETARCH

USER root
COPY artifacts/${TARGETARCH}/ /

RUN usermod -u 26 postgres
USER 26