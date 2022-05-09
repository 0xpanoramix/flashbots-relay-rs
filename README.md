# <h1 align="center"> flashbots-rpc-rs </h1>

Rust client for interacting directly with the Flashbots RPC endpoint.

## How does it work ?

Using a client from the [reqwest](https://crates.io/crates/reqwest) crate, you can use a set of methods to interact with the following Flashbots relays endpoints:

- [ ] `eth_sendBundle`
- [ ] `eth_callBundle`
- [X] `eth_sendPrivateTransaction`
- [X] `eth_cancelPrivateTransaction`
- [X] `flashbots_getUserStats`
- [X] `flashbots_getBundleStats`

It uses [ethers-rs](https://github.com/gakonst/ethers-rs) to perform ECDSA signatures and the main inspiration when it comes to code architecture comes from the [opensea-rs](https://github.com/gakonst/opensea-rs) project.

So, thanks [Georgios K.](https://github.com/gakonst) for the indirect help !

## Getting started !

### Installation

TODO.

### Quickstart

TODO.

## Author

Made with ❤️ by 🤖 [Luca Georges François](https://github.com/0xpanoramix) 🤖