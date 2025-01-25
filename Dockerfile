FROM rust as base

LABEL maintainer="Stephane Segning <selastlambou@gmail.com>"
LABEL org.opencontainers.image.description="UI Frontend for Vymalo Projects"

ENV APP_NAME=backend

WORKDIR /app

FROM base as builder

COPY ./ ./

RUN --mount=type=cache,target=/app/target \
  --mount=type=cache,target=~/.cargo/git/db \
  --mount=type=cache,target=~/.cargo/registry \
  cargo build --release --locked \ 
  && cp ./target/release/$APP_NAME $APP_NAME

FROM debian:12 as dep

RUN rm -f /etc/apt/apt.conf.d/docker-clean; echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
  --mount=type=cache,target=/var/lib/apt,sharing=locked \
  apt-get update && apt-get install -y libpq5

# Dependencies for libpq (used by diesel)
RUN mkdir /deps && \
  cp /usr/lib/*-linux-gnu/libpq.so* /deps && \
  cp /usr/lib/*-linux-gnu/libgssapi_*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libunistring.so* /deps && \
  cp /usr/lib/*-linux-gnu/libidn*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libkeyutils.so* /deps && \
  cp /usr/lib/*-linux-gnu/libtasn1.so* /deps && \
  cp /usr/lib/*-linux-gnu/libnettle.so* /deps && \
  cp /usr/lib/*-linux-gnu/libhogweed.so* /deps && \
  cp /usr/lib/*-linux-gnu/libgmp.so* /deps && \
  cp /usr/lib/*-linux-gnu/libffi.so* /deps && \
  cp /usr/lib/*-linux-gnu/libp11-kit.so* /deps && \
  cp /usr/lib/*-linux-gnu/libkrb*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libcom_err.so* /deps && \
  cp /usr/lib/*-linux-gnu/libk5crypto.so* /deps && \
  cp /usr/lib/*-linux-gnu/libsasl2.so* /deps && \
  cp /usr/lib/*-linux-gnu/libgnutls.so* /deps && \
  cp /usr/lib/*-linux-gnu/liblber-*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libldap-*.so* /deps && \
  cp /usr/lib/*-linux-gnu/libgcc_s.so* /deps

FROM gcr.io/distroless/base-debian12:nonroot

LABEL maintainer="Stephane Segning <selastlambou@gmail.com>"
LABEL org.opencontainers.image.description="UI Frontend for Vymalo Projects"

ENV RUST_LOG=warn
ENV APP_NAME=backend

WORKDIR /app

COPY --from=builder /app/$APP_NAME /app/backend
COPY --from=dep /deps /usr/lib/

EXPOSE 3000

ENTRYPOINT ["/app/backend"]