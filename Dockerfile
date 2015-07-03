FROM debian
COPY ./target/release/gunpowder-memhog /gunpowder-memhog
ENTRYPOINT ["/gunpowder-memhog"]
