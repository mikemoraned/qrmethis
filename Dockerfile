FROM liuchong/rustup:nightly-musl as build
COPY . /root
RUN cargo build --release

FROM scratch
COPY --from=build /root/target/x86_64-unknown-linux-musl/release/qrmethis /qrmethis
CMD ["/qrmethis"]
EXPOSE 8000
