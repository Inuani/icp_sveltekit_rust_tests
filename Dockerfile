FROM --platform=linux/amd64 ubuntu:22.04
FROM rust:1.71.0

ENV NVM_DIR=/root/.nvm
ENV NVM_VERSION=v0.39.1
ENV NODE_VERSION=18.1.0

WORKDIR /app

# Install a basic environment needed for our build tools
RUN apt-get update && apt-get upgrade -y -f
RUN apt-get install build-essential ca-certificates curl pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake rsync libunwind-dev sudo git -y -f

# Install Rust and Cargo
RUN rustup target add wasm32-unknown-unknown
# RUN cargo install ic-wasm


# Install Node.js using nvm
ENV PATH="/root/.nvm/versions/node/v${NODE_VERSION}/bin:${PATH}"
RUN curl --fail -sSf https://raw.githubusercontent.com/creationix/nvm/${NVM_VERSION}/install.sh | bash
RUN . "${NVM_DIR}/nvm.sh" && nvm install ${NODE_VERSION}
RUN . "${NVM_DIR}/nvm.sh" && nvm use v${NODE_VERSION}
RUN . "${NVM_DIR}/nvm.sh" && nvm alias default v${NODE_VERSION}


# Remove hostname from prompt
RUN sed -i "s|@\\\h||g" ~/.bashrc

# Install dfx
RUN sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"

CMD ["dfx","--start"]