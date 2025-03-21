FROM debian:bookworm-slim

RUN apt update && apt update
RUN apt install -y g++ vim curl cmake make build-essential git
RUN apt install -y g++ pkg-config libssl-dev gdb lldb
RUN apt install -y g++ pip

# install Rust via rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# configure Rust tooling
RUN rustup default stable
RUN rustup component add clippy rustfmt

# copy the vimrc
COPY --chown=root:root .vimrc /root/.vimrc

RUN echo "export PS1='\[\e[91m\]\u\[\e[0m\]:\[\e[94m\]\w\[\e[0m\]$ '" >> /root/.bashrc
RUN echo 'export PROMPT_COMMAND="echo -n"' >> /root/.bashrc

WORKDIR /root