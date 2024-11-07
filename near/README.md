# :rocket: ScoreBox Contract | NEAR

The smart contract exposes two methods to enable storing and retrieving credit scores from the NEAR network.

---


### Functions
Function call
```bash
    # upload credit score on blockchain
    pub fn upload_score(
        &mut self,
        score: u16,
        description: String,
        beneficiary: AccountId,
        amount: Balance
        ) -> bool { ... }
```

View call
```bash
    #query a user's scores (all)
    pub fn get_scores(
        &self,
        account_id: String
        ) -> ScoreVec { ... }
```


### Quickstart
Must-have: Rust, rustup, rustc, Near CLI, rust-toolchain, target to compile WASM
```bash
rustup target add wasm32-unknown-unknown
```

Compile and deploy
```bash
./build.sh
export a=ethsf.testnet
yarn build && near deploy --wasmFile res/contract.wasm --accountId $a
near call $a init '{"owner_id": "ethsf.testnet"}' --accountId $a
```

Interact
```bash
near call $a upload_score '{"score": 371, "description": "Congrats! 371 points.", "beneficiary": "ethsf.testnet", "amount":1}' --accountId $a
near view $a get_scores '{"account_id": "'$a'"}' --accountId $a
```