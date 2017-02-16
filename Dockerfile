FROM alpine:3.5

RUN addgroup -S app && adduser -S -g app app 
ENV HOME=/home/app

COPY target/x86_64-unknown-linux-musl/release/echo-rs /usr/local/bin/

EXPOSE 4000

ENTRYPOINT ["/usr/local/bin/echo-rs"]
