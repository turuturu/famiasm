FROM rust:1.81

# Install build dependencies for nesasm
RUN apt-get update && apt-get install -y \
    git \
    vim \
    curl \
    sudo \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Build nesasm from source
WORKDIR /tmp
RUN git clone https://github.com/camsaul/nesasm.git && \
    cd nesasm/source && \
    make && \
    make install && \
    rm -rf nesasm

# Create vscode user with proper permissions
RUN groupadd --gid 1000 vscode \
    && useradd --uid 1000 --gid 1000 -m -s /bin/bash vscode \
    && echo vscode ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/vscode \
    && chmod 0440 /etc/sudoers.d/vscode


# Verify installations
RUN rustc --version && cargo --version
RUN nesasm 2>/dev/null || echo "nesasm installed"
USER vscode

# Set working directory
#WORKDIR /
