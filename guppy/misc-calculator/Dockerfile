FROM ubuntu:20.04

RUN apt-get update && apt-get install -y build-essential socat libseccomp-dev cargo bubblewrap sudo \
    python3

RUN mkdir /tmp/index \
    && cd /tmp/index \
    && cargo init \
    && echo "serde = \"1\"" >> Cargo.toml \
    && cargo build \
    && rm -rf /tmp/index

ARG FLAG
ENV FLAG $FLAG

WORKDIR /
RUN useradd -m server; \
    useradd -m p0; \
    useradd -m p1; \
    useradd -m p2; \
    useradd -m p3; \
    usermod -a -G p0,p1,p2,p3 server;

COPY sudoers /etc/sudoers

EXPOSE 5000

WORKDIR /build
COPY webserver/. ./webserver
COPY problems/. ./problems

RUN chmod -R o-rwx /build; chown -R server:server /build

RUN cd webserver && cargo build --release && cp target/release/webserver . && strip ./webserver && cargo clean

WORKDIR /build/webserver

CMD ["su", "-l", "server", "-c", "cd /build/webserver && RUST_LOG=info ./webserver"]
