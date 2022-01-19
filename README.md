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
