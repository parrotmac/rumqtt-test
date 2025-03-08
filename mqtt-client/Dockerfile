FROM rust:alpine AS builder

#RUN apk add --no-cache linux-headers openssl g++
RUN apk add --no-cache g++

WORKDIR /build

COPY . .

RUN cargo build --release

FROM alpine

COPY --from=builder /build/target/release/mqtt-client /opt/mqtt-client

CMD ["/opt/mqtt-client"]

