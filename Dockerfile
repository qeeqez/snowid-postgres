ARG PG_MAJOR=17
ARG PGRX=0.13.0

FROM quay.io/coredb/pgrx-builder:pg${PG_MAJOR}-pgrx${PGRX}

ARG PG_MAJOR=17
ARG PGRX=0.13.0

WORKDIR /app

COPY --chown=postgres:postgres . .

ARG EXTENSION_NAME
ARG EXTENSION_VERSION

RUN cargo pgrx package