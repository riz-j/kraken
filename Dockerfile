# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory in the Docker container
WORKDIR /usr/src/myapp

# Copy the Rust project's source code into the Docker container
COPY . .

# Install cargo-watch
RUN cargo install cargo-watch

# Expose port 2900
EXPOSE 2900

# Command to run when the container starts
CMD ["cargo", "watch", "-q", "-c", "-x", "run"]