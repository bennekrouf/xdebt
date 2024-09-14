
# Use an official Rust nightly image as the base
FROM rustlang/rust:nightly

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files first (for caching dependencies)
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies (this step is cached unless Cargo.toml or Cargo.lock changes)
RUN cargo fetch

# Now copy the source code
COPY . .

# Build the release version of your Rust project
RUN cargo build --release

# Set environment variables for any .env configuration
COPY .env .env

# Use the compiled binary as the entry point
CMD ["./target/release/xdebt"]

# Expose a port if the app is a server (you can change the port if needed)
EXPOSE 9000

