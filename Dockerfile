FROM rust:1.19 as builder
RUN mkdir /gunpowder-memhog
COPY . /gunpowder-memhog
RUN cd /gunpowder-memhog && cargo build --release

FROM debian:stretch
COPY --from=builder /gunpowder-memhog/target/release/gunpowder-memhog /gunpowder-memhog
ENTRYPOINT ["/gunpowder-memhog"]
