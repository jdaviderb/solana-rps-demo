# Solana RPS Demo

```bash
# tree ./src/program-rust/src

src/program-rust/src
├── accounts # Serialize / Deserialize binary data
│   ├── command.rs 
│   ├── player_account.rs
│   └── ranking_account.rs
├── accounts.rs # Account module base (Serialize / Deserialize binary data)
├── bet.rs # Bets Helper
├── command_handlers # Command handlers module
│   ├── create_bet.rs
│   └── fight.rs
├── command_handlers.rs # command handler module base
├── config # config module
│   └── player.rs # player config module
├── config.rs # config module base
├── lib.rs # entrypoint
└── validations.rs # program validations
```

## Quick Start

[![Open in
Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/solana-labs/example-helloworld)

If you decide to open in Gitpod then refer to
[README-gitpod.md](README-gitpod.md), otherwise continue reading.

The following dependencies are required to build and run this example, depending
on your OS, they may already be installed:

- Install node (v14 recommended)
- Install npm
- Install Rust v1.56.1 or later from https://rustup.rs/
- Install Solana v1.8.2 or later from
  https://docs.solana.com/cli/install-solana-cli-tools

If this is your first time using Rust, these [Installation
Notes](README-installation-notes.md) might be helpful.

### Configure CLI

> If you're on Windows, it is recommended to use [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) to run these commands

1. Set CLI config url to localhost cluster

```bash
solana config set --url localhost
```

2. Create CLI Keypair

If this is your first time using the Solana CLI, you will need to generate a new keypair:

```bash
solana-keygen new
```

### Start local Solana cluster

This example connects to a local Solana cluster by default.

Start a local Solana cluster:
```bash
solana-test-validator
```
> **Note**: You may need to do some [system tuning](https://docs.solana.com/running-validator/validator-start#system-tuning) (and restart your computer) to get the validator to run

Listen to transaction logs:
```bash
solana logs
```

### Install npm dependencies

```bash
npm install
```

### Build the on-chain program

There is both a Rust and C version of the on-chain program, whichever is built
last will be the one used when running the example.

```bash
npm run build:program-rust
```

```bash
npm run build:program-c
```

### Deploy the on-chain program

```bash
solana program deploy dist/program/helloworld.so
```
