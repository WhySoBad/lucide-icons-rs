FROM rust:alpine3.21 AS builder
WORKDIR /app

RUN apk add --no-cache musl-dev
COPY . .

RUN cargo build --release

FROM alpine:3.21 AS runner
WORKDIR /app

COPY --from=builder /app/target/release/lucide-icons /usr/bin/lucide-icons

CMD [ "lucide-icons" ]