# Rust development environment for devcontainer
FROM rust:1.87.0-slim-bullseye

# Install packages required for development
RUN apt-get update && apt-get install -y \
    # Basic tools
    git \
    curl \
    wget \
    vim \
    nano \
    zsh \
    procps \
    lsof \
    # Build tools
    build-essential \
    pkg-config \
    libssl-dev \
    # Debugging tools
    gdb \
    lldb \
    # Network tools
    net-tools \
    && rm -rf /var/lib/apt/lists/*

# Optimize Rust settings
RUN rustup component add rustfmt clippy llvm-tools-preview rust-analyzer
RUN cargo install cargo-llvm-cov

# Oh My Zsh
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Set working directory
WORKDIR /workspace

CMD ["/bin/zsh"]
