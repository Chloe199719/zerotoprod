FROM messense/rust-musl-cross:x86_64-musl as checf
RUN cargo install cargo-chef

WORKDIR /zerotoprod

FROM checf as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM checf as builder
COPY --from=planner /zerotoprod/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .

# This build step will cache your dependencies
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /zerotoprod/target/x86_64-unknown-linux-musl/release/zerotoprod .
ENTRYPOINT [ "/zerotoprod" ]
EXPOSE 3000