# docker-rust-microservice

## Prerequisites

  * [rustup](https://www.rustup.rs/) - Rust Toolchain installer
  * [docker](https://www.docker.com/) - Docker

## Build

[Alpine Linux](https://alpinelinux.org/) uses [musl-libc](https://www.musl-libc.org/) instead of glibc, which is the default for most common distributions.

    rustup target add x86_64-unknown-linux-musl

Once that is installed we can explicitly target it when building the service,

    cargo build --target x86_64-unknown-linux-musl --release 


## Running

     docker run -it echo-rs -p 4001:4000

Troubleshooting

    docker run -it echo-rs -p 4001:4000 /bin/sh

