# Create Anchor dApp

This is a [Turborepo](https://turbo.build/repo) monorepo for managing your Solana projects built with the anchor framework. If you are new to turborepo, you can start by reading the [docs](https://turbo.build/repo/docs).

We would be using `pnpm` for the sake of this doc, but you can follow along with your favorite package manager.

## Quickstart

in your project directory, run:

```sh
solana-test-validator
```
```sh
pnpm program:deploy-all
```
```sh
pnpm program:test-all
```
```sh
pnpm build:protocol
```

if you chose to install a UI, you can run:

```sh
pnpm dev:app
```

## Usage

There is a justfile included in the repo which contains some utility scripts while using the Solana and Anchor CLI.

You can run these scripts with:

```sh
pnpm [program:utility_command] <args>
```

You may not need your package manager if you have `just` ready installed with `cargo`, you can:

```sh
just [utility_command] <args>
```

to get the same result.

All utility scripts in the root `package.json` are prefixed with `program:`.

Run `program:copy-idl` when you want to copy the idl to `protocol/ts/src` where it will be packaged and also used for testing.

Running any `program:build` or `program:deploy` scripts runs `copy-idl` automatically. so you may never need to call `copy-idl`.

Running `program:deploy` also runs `program:build`.

As a Turborepo thing, don't forget to run `build:protocol` when you are done working on the protocol directory.

You can take a look at the `justfile` to better understand what these scripts are doing.

**Enjoy yourself! 🫡**
