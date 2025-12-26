dev-init:
    k3d cluster delete k3d-nails
    k3d cluster create k3d-nails --agents 1 -p "30000-30001:30000-30001@agent:0"
    just get-config

dev-setup:
    stack init
    stack install --manifest stack.dev.yaml

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    k3d kubeconfig write k3d-nails --kubeconfig-merge-default
    sed -i "s/127\.0\.0\.1/host.docker.internal/g; s/0\.0\.0\.0/host.docker.internal/g" "$HOME/.kube/config"
    # Disable TLS verification for local dev
    sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
    sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
    echo "✅ kubeconfig updated and TLS verification disabled"

tailwind:
    mkdir -p /workspace/crates/web-assets/dist
    cd /workspace/crates/web-assets && npx tailwindcss --config ./tailwind.config.ts -i ./input.css -o ./dist/tailwind.css --watch

islands:
    cargo watch \
      -w crates/web-csr \
      -s 'cargo build -p web-csr --target wasm32-unknown-unknown --release && \
          wasm-bindgen \
            target/wasm32-unknown-unknown/release/web_csr.wasm \
            --target web \
            --out-dir crates/web-assets/dist'

asset-pipeline:
    cd /workspace/crates/asset-pipeline && npm run release

asset-pipeline-watch:
    cd /workspace/crates/asset-pipeline && npm run start

run-docker-build:
    cd dev-env-as-code &&  \
    docker build --build-arg TARGETARCH=arm64 --build-arg BUILDPLATFORM=linux/arm64 -t tobkle/toby-dev-env:latest -f Dockerfile .  && \
    docker push tobkle/toby-dev-env:latest  &&\
    cd ..

run-earthly:
    printf '%s\n' 'IMAGE_PREFIX=ghcr.io/tobkle' 'RUST_TARGET=aarch64-unknown-linux-musl' 'DBMATE_ARCH=arm64' > /tmp/earthly.arg
    cd /workspace && earthly -P --disable-remote-registry-proxy --arg-file-path /tmp/earthly.arg +build-cache
    cd /workspace && earthly -P --disable-remote-registry-proxy --arg-file-path /tmp/earthly.arg +migration-container
    cd /workspace && earthly -P --disable-remote-registry-proxy --arg-file-path /tmp/earthly.arg +app-container

run-earthly-push:
    printf '%s\n' 'IMAGE_PREFIX=ghcr.io/tobkle' 'RUST_TARGET=aarch64-unknown-linux-musl' 'DBMATE_ARCH=arm64' > /tmp/earthly.arg
    cd /workspace && earthly -P --push --disable-remote-registry-proxy --arg-file-path /tmp/earthly.arg +migration-container
    cd /workspace && earthly -P --push --disable-remote-registry-proxy --arg-file-path /tmp/earthly.arg +app-container

# Log in to GitHub Container Registry (GHCR).
# Usage: GHCR_TOKEN=... just ghcr-login
ghcr-login:
    test -n "$GHCR_TOKEN"
    echo "$GHCR_TOKEN" | docker login ghcr.io -u tobkle --password-stdin

# Verify that the images exist on GHCR by fetching their manifests.
ghcr-check:
    docker manifest inspect ghcr.io/tobkle/app:latest >/dev/null && \
    docker manifest inspect ghcr.io/tobkle/app-migrations:latest >/dev/null && \
    echo "OK: ghcr.io/tobkle/app:latest and ghcr.io/tobkle/app-migrations:latest exist"
