# `netrelay`

Tiny utility for relaying TCP traffic. This tool is useful for proxying raw TCP traffic to navigate firewalls or bridge networks.

## Build

```sh
git clone https://github.com/oddity-ai/netrelay.git && \
  cd netrelay && \
  cargo build --release
```

## Installation

The easiest way to install `netrelay` is to put in your path somewhere, for example:

```sh
cp target/release/netrelay ~/.local/bin/netrelay
```

## Usage

`netrelay` accepts two arguments: the inbound address to bind to, and the outbound address to forward traffic to. For both, use the format: `<address>:<port>`.

For example, if you want to relay all incoming traffic on port 8080 to a different host in the network with IP address 10.0.0.100:

```sh
netrelay 0.0.0.0:8080 10.0.0.100:8080
```

To have `netrelay` run in the background, you can use `screen`:

```sh
screen -dmS netrelay -- sh -c 'netrelay 0.0.0.0:8080 10.0.0.100:8080'
```
