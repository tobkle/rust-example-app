import * as fs from "fs";
// import * as k8s from "@pulumi/kubernetes";
// import * as kx from "@pulumi/kubernetesx";
import * as pulumi from "@pulumi/pulumi";

// import { HetznerServer } from "./hetzner-server";
// import { setupCluster } from "./cluster-setup";
// import { setupDatabase } from "./database";

// const config = new pulumi.Config();

// export const hetznerServerName = HetznerServer.name;
// export const hetznerServerIpv4 = HetznerServer.ipv4Address;

// function readKubeconfig(): pulumi.Input<string> | undefined {
//     const kubeconfigFromConfig = config.getSecret("kubeconfig");
//     const kubeconfigPathConfig = config.get("kubeconfigPath");
//     const kubeconfigPathEnv = process.env.KUBECONFIG;
//     const kubeconfigPath = kubeconfigPathConfig ?? kubeconfigPathEnv;
//     if (kubeconfigPath && kubeconfigPath.trim().length > 0) {
//         // KUBECONFIG can be a list of paths; take the first for simplicity.
//         const firstPath = kubeconfigPath.split(":")[0];
//         return fs.readFileSync(firstPath, "utf8");
//     }
//     return kubeconfigFromConfig ?? undefined;
// }

// const kubeconfig = readKubeconfig();
// const kubernetesProvider = kubeconfig
//     ? new k8s.Provider("k8s", { kubeconfig })
//     : undefined;

// Add a postgres operator and anything else applications need
// const cloudnativePg = setupCluster(kubernetesProvider);

// Setup a namespace for our application
// const applicationNameSpace = new k8s.core.v1.Namespace(
//     "rust-on-nails",
//     {
//     metadata: {
//         name: 'rust-on-nails'
//     },
//     },
//     kubernetesProvider ? { provider: kubernetesProvider } : undefined,
// );



// setupDatabase(applicationNameSpace, cloudnativePg)

// const applicationPods = new kx.PodBuilder({
//     containers: [{
//         name: "application",
//         image: `ghcr.io/tobkle/rust-example-app:latest`,
//         imagePullPolicy: 'IfNotPresent',
//         ports: { http: 3000 },
//         env: [
//             {
//                 name: 'APP_DATABASE_URL', valueFrom: {
//                     secretKeyRef: {
//                         name: 'database-urls',
//                         key: 'application-url'
//                     }
//                 }
//             },
//         ]
//     }],
//     initContainers: [{
//         // This runs the migrations when the pod starts.
//         name: "application-migrations",
//         image: `ghcr.io/tobkle/rust-example-app-migrations:latest`,
//         imagePullPolicy: 'IfNotPresent',
//         env: [
//             {
//                 name: 'DATABASE_URL', valueFrom: {
//                     secretKeyRef: {
//                         name: 'database-urls',
//                         key: 'migrations-url'
//                     }
//                 }
//             },
//         ]
//     }]
// })

// new kx.Deployment("application", {
//     metadata: {
//         name: "application",
//         namespace: applicationNameSpace.metadata.name
//     },
//     spec: applicationPods.asDeploymentSpec({ replicas: 1 }) 
// })
