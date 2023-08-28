### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```


## Guards
Guards can be thought of as a sort of "parental controls" for your Krypton wallet. In theory, anything that can be checked or proven on-chain can be made into a guard,
for example the Native Sol Transfer Guard allows a user to transfer up to a certain amount of sol per day, and will cause transactions to fail if a user tries to send
more than that amount. You can add custom guards by adding new variants to the `Guard` enum. After adding a new variant, create an instruction to initialize the new guard,
and configure it to contain any state that's necessary for enforcing your new guard. There is currently no way to deactivate or remove a guard from a Krypton wallet once one
has been initialized, but the Wallet Program may be extended to enable such functionality. One way to do this is to modify the Wallet Program to allow
guardians to sign a "remove guard" instruction, then the recovery UI could be modified to allow guardians to create and sign these transactions, once a threshold of guardians
have signed and sent "remove guard" instructions, the program can deactivate the guard for a given PDA wallet. This approach is designed to reduce the vulnerability of the guards
system by making it necessary for guardians to call the "remove guard" instruction, in case a user's device has been compromised. For example, if an attacker gains access to one's 
Krypton wallet, and there was no requirement for guardians to approve the removal of a guard, then the attacker could simply remove any guard related to transferring funds, then
transfer a victim's funds wherever they wanted. This approach is reasonable but the tradeoff is that a user is required to notify their guardians whenever they want to change their configured guards.

