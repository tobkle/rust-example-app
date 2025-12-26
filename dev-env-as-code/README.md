## Build

Multi-platform build (alternative method for older Docker):

```sh
cd /workspace/dev-env-as-code

# Build both platforms
docker build --build-arg TARGETARCH=amd64 --build-arg BUILDPLATFORM=linux/amd64 -t ghcr.io/tobkle/toby-dev-env:amd64 .
docker build --build-arg TARGETARCH=arm64 --build-arg BUILDPLATFORM=linux/arm64 -t ghcr.io/tobkle/toby-dev-env:arm64 .

# Push both images
docker push ghcr.io/tobkle/toby-dev-env:amd64
docker push ghcr.io/tobkle/toby-dev-env:arm64

# Create and push manifest (requires experimental features)
docker manifest create ghcr.io/tobkle/toby-dev-env:latest \
  ghcr.io/tobkle/toby-dev-env:amd64 \
  ghcr.io/tobkle/toby-dev-env:arm64

docker manifest push ghcr.io/tobkle/toby-dev-env:latest
```

Multi-platform build with Buildx (if available):

```sh
# If `docker buildx` is missing in your environment (common in some devcontainers),
# install the plugin first, then retry:
#
#   sudo mkdir -p /usr/local/lib/docker/cli-plugins
#   sudo curl -fsSL \
#     https://github.com/docker/buildx/releases/download/v0.30.1/buildx-v0.30.1.linux-arm64 \
#     -o /usr/local/lib/docker/cli-plugins/docker-buildx
#   sudo chmod +x /usr/local/lib/docker/cli-plugins/docker-buildx

docker buildx create --use
docker buildx build --platform linux/amd64,linux/arm64 -t ghcr.io/tobkle/toby-dev-env:latest --push .
```

Note: Tailwind CSS is handled via `@tailwindcss/cli` (v4+) in `crates/web-assets/package.json`.

```sh
cd /workspace/dev-env-as-code

# Build an amd64 dev-env image
docker build \
	--build-arg TARGETARCH=amd64 \
	--build-arg BUILDPLATFORM=linux/amd64 \
	-t tobkle/toby-dev-env:amd64 \
	-f Dockerfile \
	.

# Build an arm64 dev-env image
docker build \
	--build-arg TARGETARCH=arm64 \
	--build-arg BUILDPLATFORM=linux/arm64 \
	-t tobkle/toby-dev-env:arm64 \
	-f Dockerfile \
	.
```

## Run the Arm image

```sh
docker run -it --platform linux/arm64 tobkle/toby-dev-env:arm64 sh
docker login
docker tag tobkle/toby-dev-env:arm64 tobkle/toby-dev-env:latest
docker push tobkle/toby-dev-env:latest
```

## Use with Earthly

Earthly uses `DEV_ENV_IMAGE` as the base image in the `Earthfile`. You can override it via an arg file.

### Local (arm64)

```sh
cat > /tmp/earthly.arg <<'EOF'
DEV_ENV_IMAGE=tobkle/toby-dev-env:arm64
RUST_TARGET=aarch64-unknown-linux-musl
DBMATE_ARCH=arm64
EOF

earthly --allow-privileged --arg-file-path /tmp/earthly.arg +app-container
```

### Local (amd64)

```sh
cat > /tmp/earthly.arg <<'EOF'
DEV_ENV_IMAGE=tobkle/toby-dev-env:amd64
RUST_TARGET=x86_64-unknown-linux-musl
DBMATE_ARCH=amd64
EOF

earthly --allow-privileged --arg-file-path /tmp/earthly.arg +app-container
```