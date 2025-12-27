import * as k8s from "@pulumi/kubernetes";
import { Release } from "@pulumi/kubernetes/helm/v3";
import * as pulumi from "@pulumi/pulumi";

export function setupCluster(provider?: k8s.Provider): Release {
    const config = new pulumi.Config();
    
    const skipCrds = config.getBoolean("cloudnativePgSkipCrds") ?? false;
    const adoptExisting = config.getBoolean("cloudnativePgAdoptExisting") ?? false;

    // Setup a namespace for Cloud Native Pg https://github.com/cloudnative-pg/cloudnative-pg
    const databaseNameSpace = new k8s.core.v1.Namespace(
        "cloud-native-pg",
        {
            metadata: {
                name: "cloud-native-pg",
            },
        },
        provider ? { provider } : undefined,
    );

    // Install the Postgres operator from a helm chart
    const releaseName = "cloudnative-pg";
    const releaseNamespace = "cloud-native-pg";

    const helmOwnershipMetadata = {
        labels: {
            "app.kubernetes.io/managed-by": "Helm",
        },
        annotations: {
            "meta.helm.sh/release-name": releaseName,
            "meta.helm.sh/release-namespace": releaseNamespace,
        },
    };

    const adoptionPatches: pulumi.Resource[] = [];

    if (adoptExisting) {
        adoptionPatches.push(
            new k8s.admissionregistration.v1.MutatingWebhookConfigurationPatch(
                "cnpg-mutating-webhook-configuration-adopt",
                {
                    metadata: {
                        name: "cnpg-mutating-webhook-configuration",
                        ...helmOwnershipMetadata,
                    },
                },
            ),
            new k8s.admissionregistration.v1.ValidatingWebhookConfigurationPatch(
                "cnpg-validating-webhook-configuration-adopt",
                {
                    metadata: {
                        name: "cnpg-validating-webhook-configuration",
                        ...helmOwnershipMetadata,
                    },
                },
            ),
            new k8s.rbac.v1.ClusterRolePatch("cnpg-manager-clusterrole-adopt", {
                metadata: {
                    name: "cnpg-manager",
                    ...helmOwnershipMetadata,
                },
            }),
            new k8s.rbac.v1.ClusterRoleBindingPatch("cnpg-manager-rolebinding-adopt", {
                metadata: {
                    name: "cnpg-manager-rolebinding",
                    ...helmOwnershipMetadata,
                },
            }),
        );
    }

    const cloudnativePg = new k8s.helm.v3.Release(
        "cloudnative-pg",
        {
        name: "cloudnative-pg",
        chart: "cloudnative-pg",
        namespace: databaseNameSpace.metadata.name,
        skipCrds,
        values: {
            crds: {
                create: !skipCrds,
            },
        },
        repositoryOpts: {
            repo: "https://cloudnative-pg.github.io/charts",
        },
        },
        {
            provider,
            dependsOn: adoptionPatches,
        },
    );

    return cloudnativePg;
}