import * as pulumi from "@pulumi/pulumi";
import * as hcloud from "@pulumi/hcloud";
import * as dotenv from "dotenv"

dotenv.config({ path: '../.env' })

const config = new pulumi.Config();
const k8sApiSourceIps = config.getObject<string[]>("k8sApiSourceIps") ?? ["0.0.0.0/0", "::/0"];

// Setup Network
const network = new hcloud.Network("HetznerAppNetwork", {
    name: "app-network",
    ipRange: "10.0.0.0/8",
    deleteProtection: false,
    exposeRoutesToVswitch: false,
    labels: {
        environment: "pulumi",
    },
});
const networkId = network.id.apply((id) => parseInt(id, 10));

// Setup Subnet
const network_subnet = new hcloud.NetworkSubnet("HetznerAppSubnet", {
    networkId: networkId,
    type: "cloud",
    networkZone: "eu-central",
    ipRange: "10.0.1.0/24",
    vswitchId: undefined,
});

// Setup Placement Group
const placement_group = new hcloud.PlacementGroup("HetznerAppPlacementGroup", {
    name: "app-group",
    type: "spread",
    labels: {
        key: "app",
    },
});
const placementGroupId = placement_group.id.apply((id) => parseInt(id, 10));

// Setup Firewall
const firewall = new hcloud.Firewall("HetznerAppFirewall", {
    rules: [
        {
            protocol: "icmp",
            direction: "in",            
            sourceIps: [
                "0.0.0.0/0",
                "::/0",
            ],
        },        
        { 
            protocol: "tcp", 
            direction: "in", 
            port: "22", 
            sourceIps: ["0.0.0.0/0"] 
        },
        {
            protocol: "tcp",
            direction: "in",
            port: "6443",
            sourceIps: k8sApiSourceIps,
        },
        { 
            protocol: "tcp", 
            direction: "in", 
            port: "80", 
            sourceIps: [
                "0.0.0.0/0",
                "::/0",
            ],
        }, 
        { 
            protocol: "tcp", 
            direction: "in", 
            port: "443", 
            sourceIps: [
                "0.0.0.0/0",
                "::/0",
            ],
        },                  
    ],
});
const firewallId = firewall.id.apply(id => parseInt(id, 10));

// Setup Server
const server = new hcloud.Server("HetznerAppServer", {
    name: "app-node-1",
    image: "debian-12",
    serverType: "cx23",   // 2 vCPU, 4 GB RAM, 40 GB SSD 
    sshKeys: [
        "HCLOUD_PUB"
    ],
    networks: [
        {
            networkId: networkId,
            ip: "10.0.1.5",
            // macAddress: undefined,
            // aliasIps: [
            //     "10.0.1.6",
            //     "10.0.1.7",
            // ]
        }
    ],
    location: "nbg1",     // fsn1 currently unavailable for new servers
    // datacenter: "fsn1-dc14",
    placementGroupId: placementGroupId,
    deleteProtection: false,
    // iso: undefined,
    keepDisk: true,
    // allowDeprecatedImages: false,
    // rebuildProtection: false,
    // rescue: "",
    backups: true,
    // shutdownBeforeDeletion: true,
    publicNets: [{ 
        ipv4Enabled: true, 
        ipv6Enabled: true 
    }],
    firewallIds: [
        firewallId
    ],
    labels: {
        environment: "app",
    },
    // userData: `#cloud-config`
}, {
    dependsOn: [network_subnet],
});

export const HetznerServer =  server