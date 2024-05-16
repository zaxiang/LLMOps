# Use an official Rust image
FROM rust:1.77 as builder

RUN apt-get update
RUN apt-get install -y build-essential cmake curl openssl libssl-dev

ARG LIBTORCH_URL=https://download.pytorch.org/libtorch/cu121/libtorch-cxx11-abi-shared-with-deps-2.1.0%2Bcu121.zip

RUN curl -L ${LIBTORCH_URL} -o libtorch.zip && \
    unzip libtorch.zip -d / && \
    rm libtorch.zip

ENV LIBTORCH=/libtorch
ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

# Create a new empty shell project
RUN USER=root cargo new artix_backend
WORKDIR /artix_backend

# Copy the manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This is a dummy build to get the dependencies cached
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependencies are built, copy your source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/artix_backend*
RUN cargo build --release

# test
# RUN cargo test -v

# Final stage
FROM debian:bookworm-slim
RUN apt-get update
RUN apt-get install -y build-essential cmake curl openssl libssl-dev

# Copy the release binary
COPY --from=builder /artix_backend/target/release/artix_backend .

# Copy model directories
# COPY ./model/distilbert-sst2 /model/distilbert-sst2
# COPY ./model/gpt2-medium /model/gpt2-medium

# Copy and configure libtorch
COPY --from=builder /libtorch/ /libtorch/
ENV LIBTORCH=/libtorch
ENV LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH

# Configure AWS environment variables
ENV AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID
ENV AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY
ENV AWS_DEFAULT_REGION=us-east-2

# Configure environment variables
ENV RUSTBERT_CACHE=/tmp
# ENV RUSTBERT_CACHE=/model
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["./artix_backend"]
