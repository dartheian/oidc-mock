FROM rust:alpine as build-env
WORKDIR /usr/src/oidcm
COPY . .
RUN cargo install --path . --root /usr

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /usr/bin/oidcm /

HEALTHCHECK --interval=5m --timeout=3s CMD curl -f http://localhost/health || exit 1
CMD ["./oidcm"]
