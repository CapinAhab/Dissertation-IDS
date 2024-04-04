# Use the official Rust image as builder
FROM rust:1.77-bookworm as builder

# Need wget to download libtorch 2.2.0
RUN apt-get update && apt-get install -y libpcap-dev libc6 wget unzip

WORKDIR /usr/src/ids

# Move correct version of libglog to /usr/lib so compiler uses it
COPY libglog/* /usr/lib/

# Set LIBTORCH env so compiler uses correct version of libtorch
ENV LIBTORCH=/usr/src/ids/libtorch
#Set compiler to use dependencies packaged with libtorch to stop version conflicts
ENV LD_LIBRARY_PATH=/usr/src/ids/libtorch/lib:$LD_LIBRARY_PATH

COPY . .

# Download correct version of libtorch
RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.2.0%2Bcpu.zip \
    && unzip libtorch-cxx11-abi-shared-with-deps-2.2.0+cpu.zip \
    && rm libtorch-cxx11-abi-shared-with-deps-2.2.0+cpu.zip


# Build your Rust application
RUN cargo build --release

# Need debian as a base image to execute the built program
FROM debian:bookworm

RUN apt-get update && apt-get install -y libpcap-dev libc6 gcc

WORKDIR /usr/src/ids

COPY --from=builder /usr/src/ids/libtorch/lib/* /usr/lib/
COPY --from=builder /usr/src/ids/ /usr/src/ids 

EXPOSE 8000

CMD ["./target/release/ids"]
