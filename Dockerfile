FROM scratch
LABEL authors=":kelko:"

COPY target/aarch64-unknown-linux-musl/release/keret-service .

EXPOSE 3000

ENTRYPOINT ["./keret-service"]
