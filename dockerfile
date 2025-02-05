FROM rust:latest

COPY ./webserver /usr/webserver

WORKDIR /usr/webserver

RUN cargo install --path .

CMD [ "cargo","run" ]

