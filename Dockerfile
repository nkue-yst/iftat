FROM alpine:3.15.4 AS builder

WORKDIR /home/iftat

COPY . .

RUN  apk add --no-cache gcc musl-dev rust cargo \
  && cargo build --release

FROM alpine:3.15.4

RUN  apk add --no-cache libgcc musl-dev \
  && adduser -D iftat

COPY --from=builder /home/iftat/target/release/iftat /opt/iftat/iftat

WORKDIR /home/iftat
USER iftat

ENTRYPOINT [ "/opt/iftat/iftat" ]