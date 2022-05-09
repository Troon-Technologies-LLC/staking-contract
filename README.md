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

| Function Name  | Description | Parameters | Explorer Link / Response|
| ------------- | ------------- | ------------- | ------------- |
| ft_transfer_call | Method to stake tokens  | ```receiver_id: AccountId,amount: U128,memo: Option<String>,msg: String``` | https://bit.ly/3kTZ7ZN |
| claim_reward  | Method to claim reward stake tokens | ```stake_id: StakeId```  | https://bit.ly/3kUSvug| 
| ft_unstake  | Method to unstake tokens  | ```stake_id: StakeId```  | https://bit.ly/3sit5e7  | 
| get_staking_history  | Method to get staking history of a user  | ```account_id: AccountId,from_index: Option<U128>,limit: Option<u64>``` | https://bit.ly/3922hba | 
| get_claim_history  | Get the details about how many times users have calimed their reward  | ```stake_id: StakeId```  | https://bit.ly/3MYG1h5  |
