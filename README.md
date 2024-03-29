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

## ⚖️ License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
