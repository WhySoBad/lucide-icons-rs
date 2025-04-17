FROM rust:alpine3.21 AS builder
WORKDIR /app

RUN apk add --no-cache musl-dev
COPY . .

RUN cargo build --release

FROM alpine:3.21 AS runner
WORKDIR /app

COPY --from=builder /app/target/release/lucide-icons /usr/bin/lucide-icons

ENV LIB_NAME=lucide-icons
ENV ICONS_VERSION=
ENV OUT_DIR=out

SHELL [ "/bin/sh", "-c" ]
CMD lucide-icons --name $LIB_NAME --output $OUT_DIR $ICONS_VERSION