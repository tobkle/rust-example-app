# Setup K3S on Hetzner Cloud

```sh
brew install vitobotta/tap/hetzner_k3s
brew install helm
brew install argo
hetzner-k3s create --config cluster_config.yaml | tee create.log
export KUBECONFIG=./kubeconfig
kubectl get nodes
kubectl get pods
hetzner-k3s delete --config cluster-config.yaml
```

# Setup Pulumi

## CloudNativePG CRDs already exist

If `pulumi up` fails with an error like “CRD … exists and cannot be imported into the current release” (missing Helm ownership labels/annotations), it usually means the CloudNativePG CRDs were installed previously outside of this Helm release.

To unblock the install, configure the chart install to skip CRDs:

```sh
pulumi config set cloudnativePgSkipCrds true
pulumi config set cloudnativePgAdoptExisting true
pulumi up
```

If you prefer not to “adopt” existing resources, delete the old operator resources (webhooks/clusterroles) instead and rerun `pulumi up`.

For a brand-new cluster where the CNPG CRDs are not installed yet, keep this unset/false so the chart can install CRDs normally.

```sh
> pulumi new kubernetes-typescript
  stack-name: rust-example-app
  environment: tobkle/dev
  package-manager: npm

> npm install @pulumi/random
> npm i @pulumi/kubernetes
> npm i @pulumi/kubernetesx --legacy-peer-deps
> npm i @pulumi/hcloud --legacy-peer-deps
> export HCLOUD_TOKEN="DEIN_HETZNER_API_TOKEN"
> pulumi config set hcloud:token DEIN_HETZNER_API_TOKEN --secret
> export HPUBLIC_SSH="HETZNER Public SSH KEY"


> pulumi up
  updating dev
  Please choose a stack, or create a new one: dev
  Previewing update (dev)

  View in Browser (Ctrl+O): https://app.pulumi.com/tobkle/rust-example-app/dev/previews/ed857591-0445-48fa-8950-261cbd417576

      Type                              Name                  Plan       
  +   pulumi:pulumi:Stack               rust-example-app-dev  create     e
  +   ├─ kubernetes:core/v1:Namespace   rust-on-nails         create     
  +   ├─ kubernetes:core/v1:Namespace   cloud-native-pg       create     
  +   └─ kubernetes:helm.sh/v3:Release  cloudnative-pg        create     

  Resources:
      + 4 to create

  Do you want to perform this update?  [Use arrows to move, type to filter]
  > yes
```