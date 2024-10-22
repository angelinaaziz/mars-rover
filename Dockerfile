# Use the official Rust image with Debian as the base
FROM rust:1.56-buster as build

# Set the working directory inside the container
WORKDIR /usr/src/mars-rover

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the Rust project
RUN cargo build --release

# Use a smaller base image for the final stage with the correct glibc
FROM debian:buster-slim

# Copy the built binary from the builder stage
COPY --from=build /usr/src/mars-rover/target/release/mars-rover /usr/local/bin/mars-rover

# Set the entry point to the Mars Rover program
ENTRYPOINT ["mars-rover"]
