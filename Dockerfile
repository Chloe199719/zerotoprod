FROM messense/rust-musl-cross:x86_64-musl as builder
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
WORKDIR /zerotoprod
# Copy over your Manifest files
COPY . .

# This build step will cache your dependencies
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /zerotoprod/target/x86_64-unknown-linux-musl/release/zerotoprod .
ENTRYPOINT [ "/zerotoprod" ]
EXPOSE 3000