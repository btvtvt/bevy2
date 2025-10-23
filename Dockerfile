#FROM rust:1.90.0-bookworm
FROM debian:13-slim

#RUN apt update
#
#RUN apt install -y g++ pkg-config libx11-dev libasound2-dev \
#    libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev \
#    lld clang

WORKDIR /usr/src/myapp
#COPY . .

#RUN cargo install --path .

#RUN #cargo build --bin server

#CMD ["myapp"]
ENV RUST_LOG=debug

COPY target/debug/server target/debug/server

CMD ["target/debug/server"]
