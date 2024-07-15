# Create Anchor dApp

This is a starter template generated from [Create-Anchor-dApp](https://www.npmjs.com/package/create-anchor-dapp) monorepo CLI generator built by The Wuh.


## Quickstart

Start by:

```sh
pnpm just run-validator
```
```sh
pnpm just deploy-all
```
```sh
pnpm just test marketplace
```

## Not-So-Quick Start

The scripts are there to aid development.

`just [command]` is should be the same as `pnpm program:[command]`, most of the time. Check the `justfile` and root `package.json` to see more. I will be using `just [command]` for rest of the doc.

-  Not every Token extension works on localnet. So we have to build from source. The binary is already included at `/spl/spl_token_2022.so`. You can run:

```sh
 just run-validator
```

which is basically just,
```sh
 solana-test-validator --reset --bpf-program TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb ./spl/spl_token_2022.so
```

- Ran into compilation issues relating to this [issue](issue link). Copied this [fix](https://github.com/coral-xyz/anchor/issues/3044#issuecomment-2184026104) that seemed to work.
```sh
cargo update -p solana-zk-token-sdk@2.0.0 --precise 1.18.17
```
can run:
```sh
just update-deps
```
if you run into the same issues.

- All Tests are run in the `protocol/tests` directory. it does, however, require that you copy idls to the `protocol/src/idl` directory.

- Run `just copy-idl` to do this automatically. `just build` and `just deploy` do this automatically. _sidenote: `just deploy` also runs `just build`. you can run `just deploy` if you want to build and deploy_

- The Rust challenge are in `rust/track-r5`. `marketplace` is the entrypoint. `marketplace-transfer-controller` is the transfer hook program. You probably want to build and deploy `marketplace-transfer-controller` program first.

- Common utility commands:
`just copy-idl`, copys idl and types to the `protocol/src/idl` directory to be packaged

- `just build [program-name]`, same as `anchor build -p [program-name]`, also runs `just copy-idl`

- `just deploy [program-name]`, same as `anchor deploy -p [program-name]`, also runs `just build` & 
- `just copy-idl`.

- `just build-all` and `just deploy-all` runs `anchor build` and `anchor deploy` respectively.

- `just test [script-name]`. same as `anchor run`.


## Context

- Create a Service by creating a Token Group NFT, called `service`
- All Token Group Members are `service_ticket`s
- `ServiceAccount` holds either `service` or `service_ticket` mints
- Get all listings on the Marketplace by fetching all `ServiceAccount`s
