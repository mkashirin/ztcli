# ZTCLI

ZTCLI (ZeroTier CLI) is a minimal CLI that combines ZeroTier Central and
ZeroTierOne Service essential functionality. This tool allows for seamless
LAN gaming and ZeroTier-hosted free plan network manipulation.

## Prerequisites

ZTCLI is based on [ZeroTier](https://www.zerotier.com/) products: ZeroTier
Central and ZeroTierOne Service. (The first is a free network hosting platform
with a dashboard and pricing plans, the latter is the agent that acts as a
network switch.) Since that, you need to register a ZeroTier account and create
an API token for the ZeroTier Central. You also need to install the agent
(zerotier-cli) to connect to the network.

## VLAN Setup

First, download the [ZTCLI binary](https://github.com/mkashirin/ztcli/releases/download/0.1.0/ztcli.exe).
Open the directory with the downloaded binary via terminal.

Generate an API token for your ZeroTier Central account and bind the evironment
varibale named `ZTC_API_TOKEN` to it like so:
```powershell
Env:ZTC_API_TOKEN="<ZeroTier Central API token>"
```
Then create a network via ZeroTier Central API as follows:
```powershell
.\ztcli.exe central network create --private --name="Example Net"
```
This would print out the brand new network short description with the ID. Use
this ID to connect to the network on your machines like this:
```powershell
.\ztcli.exe one network post --id="<ID of Example Net>"
```
After that you need to authorize all the members on the network as in the line
below:
```powershell
.\ztcli.exe central network update --authorize-all --id="<ID of Example Net>"
```
You can now list the peers available for communication like so:
```powershell
.\ztcli.exe one peers
```
It would list all the nodes your node knows about and their addresses. Now check
the peer resolvability as in the following script:
```powershell
ping <IP of any peer>
```

If the ping went well, congratulations! You now have a VLAN, to play with your
friends for example.

## Future

### Self-hosting

In the future ZTCLI will support fast controller setup (self-hosting). This
would allow for no ZeroTier account registered, since we wouldn't need any free
host to run the network.

### VPN Setup

ZeroTier can be used to setup a VPN Exit Node, but the process involves much
CURLing and typing long one-liners. ZTCLI will make this procedure more
intuitive and straight forward.
