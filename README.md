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
- If you run behind an ingress that routes by `/api` prefix, the server also accepts `/api/api.Users/GetUsers`.