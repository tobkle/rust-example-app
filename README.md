## Rust on Nails

Built with the [Rust on Nails](https://rust-on-nails.com/) architecture for secure full stack web applications.

## Bruno gRPC (local dev)

- Server address: `localhost:3000` (no `http://`)
- Transport: plaintext / insecure (h2c, no TLS)
- Enable **Server Reflection** in Bruno, then refresh schema
- Service/method: `api.Users` → `GetUsers`

Notes:
- The canonical gRPC method path is `/api.Users/GetUsers`.
- If you run behind an ingress that routes by `/api` prefix, the server also accepts `/api/api.Users/GetUsers`.

## Asset pipeline (web-components)

- Build the JS/CSS bundle: `just asset-pipeline`
- The Rust build publishes it automatically into `crates/web-assets/dist/asset-pipeline/`
- The server serves it under `/dist/asset-pipeline/index.js`

Usage example (Dioxus/RSX): render the custom element anywhere, e.g. `hello-world {}`.

## Dev env image

See [dev-env-as-code/README.md](dev-env-as-code/README.md) for how to build the dev environment image locally for both `amd64` and `arm64` (tags `:amd64` / `:arm64`) and how to point Earthly to the right one via `DEV_ENV_IMAGE`.