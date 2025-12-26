import "../dist/asset-pipeline/index.js";
import init, { hydrate } from "../dist/web_csr.js";

async function main() {
  try {
    await init();
    hydrate();
  } catch (err) {
    console.error("Failed to initialize web-csr islands", err);
  }
}

main();
