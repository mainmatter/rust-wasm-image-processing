FROM mcr.microsoft.com/devcontainers/rust

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked mdbook
RUN cargo install --locked wasm-pack
RUN cargo install --locked watchexec-cli

RUN chown -R vscode:vscode /usr/local/cargo

USER vscode
