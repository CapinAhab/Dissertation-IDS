# Use the official Rust image as builder
FROM rust:latest as builder

# Need wget to download libtorch 2.2.0
RUN apt-get update && apt-get install -y wget unzip libpcap-dev

WORKDIR /usr/src/ids

# Move correct version of libglog to /usr/lib so compiler uses it
COPY libglog/* /usr/lib/

# Set LIBTORCH env so compiler uses correct version of libtorch
ENV LIBTORCH=/usr/src/ids/libtorch

COPY . .

# Download correct version of libtorch
RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.2.0%2Bcpu.zip \
    && unzip libtorch-cxx11-abi-shared-with-deps-2.2.0+cpu.zip \
    && rm libtorch-cxx11-abi-shared-with-deps-2.2.0+cpu.zip


# Build your Rust application
RUN cargo build --release

# Need debian as a base image to execute the built program
FROM debian:buster-slim

WORKDIR /usr/src/ids

COPY --from=builder /usr/src/ids/target/release/ids .

EXPOSE 8000

CMD ["./ids"]