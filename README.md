# Brick Trade Contracts

## Building

To build run:

```bash
./build.sh
```

## Testing

As with many Rust libraries and contracts, there are tests in the main fungible token implementation at `ft/src/lib.rs`.

Additionally, this project has [simulation] tests in `tests/sim`. Simulation tests allow testing cross-contract calls, which is crucial to ensuring that the `ft_transfer_call` function works properly. These simulation tests are the reason this project has the file structure it does. Note that the root project has a `Cargo.toml` which sets it up as a workspace. `fungible-token-contract` and `staking-contract` are both small & focused contract projects, the latter only existing for simulation tests. The root project imports `near-sdk-sim` and tests interaction between these contracts.

You can run all these tests with one command:

```bash
cargo test
```

## Contract Interfaces For Staking

| Function Name  | Description | Parameters | Example | Explorer Link / Response|
| ------------- | ------------- | ------------- | ------------- | ------------- |
| ft_transfer_call | Method to stake tokens  | {
  ""receiver_id"": ""staking_bkrt.testnet"",
  ""amount"": ""5000000000000000000000000000"",
  ""msg"": ""{\""ft_symbol\"":\""BKRT\"",\""ft_account_id\"":\""ft_bkrt.testnet\"",\""decimal\"":24,\""duration\"":15778800,\""staked_by\"":\""ahsans.testnet\"",\""staking_plan\"":\""BKRTPremium6\""}""
}  | Content Cell  | Content Cell  |
| Content Cell  | Content Cell  | Content Cell  | Content Cell  | Content Cell  |
| Content Cell  | Content Cell  | Content Cell  | Content Cell  | Content Cell  |
| Content Cell  | Content Cell  | Content Cell  | Content Cell  | Content Cell  |
| Content Cell  | Content Cell  | Content Cell  | Content Cell  | Content Cell  |
| Content Cell  | Content Cell  | Content Cell  | Content Cell  | Content Cell  |
| Content Cell  | Content Cell  | Content Cell  | Content Cell  | Content Cell  |
| Content Cell  | Content Cell  | Content Cell  | Content Cell  | Content Cell  |
| Content Cell  | Content Cell  | Content Cell  | Content Cell  | Content Cell  |
| Content Cell  | Content Cell  | Content Cell  | Content Cell  | Content Cell  |
