FROM mcr.microsoft.com/devcontainers/rust

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked mdbook@0.5.2
RUN cargo install --locked wasm-pack@0.14.0
RUN cargo install --locked watchexec-cli@2.3.3

RUN chown -R vscode:vscode /usr/local/cargo

USER vscode
