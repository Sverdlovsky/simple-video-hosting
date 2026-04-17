FROM docker.io/library/rust:1.94.0-alpine3.23 AS build

RUN apk add \
    openssl-dev \
    musl-dev \
    perl \
    make \
    gcc

WORKDIR /srv/api

RUN cargo init

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

RUN cargo clean -p svh

RUN rm -r src
COPY src src

RUN cargo build --release


FROM docker.io/library/alpine:3.23

COPY --from=build /srv/api/target/release/svh /usr/bin/svh

RUN addgroup -g 31541 user
RUN adduser -DH -u 31541 -G user user
USER user

EXPOSE 8080

ENTRYPOINT [ "/usr/bin/svh" ]

