# Use the official Debian base image
FROM debian:latest

# Set up the environment
ENV DEBIAN_FRONTEND noninteractive


# Install necessary tools and dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    file \
    && rm -rf /var/lib/apt/lists/*
# Install sqlite3

RUN apt-get update && \
    apt-get install -y build-essential && \
    apt-get install -y libsqlite3-dev && \
    apt-get install -y sqlite3 && \
    rm -rf /var/lib/apt/lists/*


# Build the project
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


# Set the working directory inside the container
WORKDIR /backend

# Copy the source files into the container
COPY ./backend/ /backend/
COPY ./backend/Cargo.toml /backend/Cargo.toml


# Instal diesel cli
RUN cargo install diesel_cli --no-default-features --features sqlite
# Run migrations
RUN diesel migration run

# Install node 16
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs

RUN node -v
RUN npm -v

# Run seed script (in /backend/seed-script directory)

# Change working directory to seed script
WORKDIR /backend/seed-scripts
RUN npm install
RUN node create_boards.js


# # The compiled executable will be available at /app/target/release/
WORKDIR /backend
RUN cargo build --release


# # Dockerfile cleanup (optional)
RUN rm -rf /root/.cargo
RUN rm -rf /backend/target/debug
RUN rm -rf /backend/target/.rustc_info.json
RUN rm -rf /backend/target/release/.fingerprint/backend-*
RUN rm -rf /backend/target/release/build
RUN rm -rf /backend/target/release/deps/backend-*
RUN rm -rf /backend/target/release/.fingerprint/backend-*

# Remove the source files
RUN rm -rf /backend/src
RUN rm -rf /backend/Cargo.toml
RUN rm -rf migrations
RUN rm -rf /backend/.vscode

EXPOSE 3008

CMD ["/backend/target/release/backend"]
