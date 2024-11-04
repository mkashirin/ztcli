<h1 align="center">ZTCLI</h1>

<p align="center">
ZTCLI (ZeroTier CLI) is a minimal CLI that combines ZeroTier Central and
ZeroTierOne Service essential functionality. This tool allows for seamless
LAN gaming and ZeroTier-hosted free plan network manipulation.
</p>

## Prerequisites

ZTCLI is based on [ZeroTier](https://www.zerotier.com/) products: ZeroTier
Central and ZeroTierOne Service. (The first is a free network hosting platform
with a dashboard and pricing plans, the latter is the agent that acts as a
network switch.) Since that, you need to register a ZeroTier account and create
an API token for the ZeroTier Central. You also need to install the agent
(zerotier-cli) to connect to the network.

## VLAN Setup

First, download the [ZTCLI binary](https://github.com/mkashirin/ztcli/releases/download/0.1.0/).
Open the directory with the downloaded binary via terminal.

### Using Central API

Generate an API token for your ZeroTier Central account and bind the evironment
varibale named `ZTC_API_TOKEN` to it like so:
```bash
ZTC_API_TOKEN="<central-api-token>"
```
Then create a network via ZeroTier Central API as follows:
```bash
./ztcli.exe central network create --name="<name>" --private 
```
This would print out the brand new network short description with the ID. Use
this ID to connect to the network on your machines like this:
```bash
./ztcli.exe one network post --id="<nwid>"
```
After that you need to authorize all the members on the network as in the line
below:
```bash
./ztcli.exe central network update --id="<nwid>" --authorize-all
```

### Using Self-hosted Controller

First, create a network on the controller to host and give it a name like this:
```bash
./ztcli.exe one controller network post --name="<name>"
```
It should return the network ID. Then join the network created from other
nodes as follows:
```bash
./ztcli.exe one network post --id="<nwid>"
```
After that, the addresses of the nodes can be viewed by execution of the line
below on the controller hosting the network:
```bash
./ztcli.exe one controller member list --network-id="<nwid>"
```
Then, as with the Central API, the network members should be authorized by the
controller like so:
```bash
./ztcli.exe one controller network post --id="<nwid>" --authorize-all
```

### Check Nodes Connection

You can now list the networks joined:
```bash
./ztcli.exe one network list
```
It would list all the networks your node has joined to about and their addresses.
But more importantly it would list addresses assigned to your node, that other nodes
can resolve it through. Now check the peer resolvability as in the following script,
by running it on another node on your network:
```bash
ping "<your-assigned-address>"
```
If the ping went well, congratulations! You now have a VLAN, to play with your
friends for example.

## Future

### VPN Setup

ZeroTier can be used to setup a VPN Exit Node, but the process involves much
CURLing and typing long one-liners. ZTCLI will make this procedure more
intuitive and straight forward.

## Licencing

This project is distributed under the MIT open source licence.
