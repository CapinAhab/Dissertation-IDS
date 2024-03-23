# Use the official Rust image as builder
FROM rust:1.77-bookworm as builder

# Need wget to download libtorch 2.2.0
RUN apt-get update && apt-get install -y libpcap-dev libc6 

WORKDIR /usr/src/ids

# Move correct version of libglog to /usr/lib so compiler uses it
COPY libglog/* /usr/lib/

COPY . .

# Build your Rust application
RUN cargo build --release

# Need debian as a base image to execute the built program
FROM debian:bookworm

RUN apt-get update && apt-get install -y libpcap-dev libc6

WORKDIR /usr/src/ids

COPY --from=builder /usr/src/ids/ /usr/src/ids 
COPY --from=builder /usr/src/ids/target/release/ids .

EXPOSE 8000

CMD ["./ids"]
