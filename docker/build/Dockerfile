FROM amazonlinux:2023

# gcc is required to build ring, one of aws-config dependencies
# shadow-utils has useradd
RUN yum update -y \
    && yum install -y gcc shadow-utils \
    && yum clean all

# Create a new user named 'user' and set up the working directory
RUN useradd -m user

# Switch to non-root user
USER user

# https://rust-lang.github.io/rustup/concepts/profiles.html
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | /bin/sh -s -- --profile minimal -y

WORKDIR /volume
