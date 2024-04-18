# Use the official Rust image as builder
FROM rust:1.77-bookworm as builder

# Need wget to download libtorch 2.2.0
#RUN apt-get update && apt-get install -y libpcap-dev libc6 wget unzip

WORKDIR /usr/src/ids

COPY . .

# Build your Rust application
RUN cargo build --release

# Need debian as a base image to execute the built program
FROM debian:bookworm

RUN apt-get update && apt-get install -y pip && pip install --user tensorflow && pip install --user scikit-learn 


WORKDIR /usr/src/ids

COPY --from=builder /usr/src/ids/ /usr/src/ids 

EXPOSE 8000

CMD ["./target/release/ids & python model.py"]
