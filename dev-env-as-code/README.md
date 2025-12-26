## Build

```sh
cd /workspace/dev-env-as-code && \
docker build --build-arg TARGETARCH=amd64 --build-arg BUILDPLATFORM=linux/amd64 -t tobkle/toby-dev-env:latest -f Dockerfile . \
docker build --build-arg TARGETARCH=arm64 --build-arg BUILDPLATFORM=linux/arm64 -t tobkle/toby-dev-env:latest-f Dockerfile .
```

## Run the Arm image

```sh
docker run -it --platform linux/arm64 toby-dev-env:latest sh
docker login
docker tag toby-dev-env:latest tobkle/toby-dev-env:latest
docker push tobkle/toby-dev-env:latest
```