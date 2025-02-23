ARG PG_VERSION=17
ARG PGRX_VERSION=0.12.9

FROM quay.io/coredb/pgrx-builder:pg${PG_VERSION}-pgrx${PGRX_VERSION}

ARG PG_VERSION=17
ARG PGRX_VERSION=0.12.9

WORKDIR /app

COPY --chown=postgres:postgres . .

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN cargo pgrx package