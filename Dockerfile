FROM debian:jessie-slim
EXPOSE 80
RUN apt-get -qq update -y && \
  apt-get -qq install -y --no-install-recommends curl ca-certificates \
  file sudo gcc libc-dev pkg-config openssl libssl-dev
RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- \
  --channel=nightly --date=2017-04-23 --prefix=/usr
WORKDIR /build
COPY Cargo.toml Cargo.lock /build/
COPY src /build/src
RUN cargo build --release
CMD ROCKET_ENV=production cargo run --release