<div align="center">
  <a href="https://patron.works/">
    <img src="https://github.com/Brushfam/patron-backend/raw/master/Patron.png" alt="Logo"  >
  </a>

  <p align="left">
    &#128226; &#128226; &#128226; We are thrilled to announce <a href="https://patron.works/">Patron</a>, which brings smart contract verification functionality to the Polkadot ecosystem. &#128226; &#128226; &#128226;
  </p>
</div>

> Smart contract verification ensures the security, reliability, and trustworthiness of dApps and blockchain platforms. With [Patron](https://patron.works/), you can simplify the deployment flow, manage your builds and make the Polkadot ecosystem more secure and transparent.
<br/>
So, in other words,  <a href="https://patron.works/">Patron</a> is an all-in-one contracts platform, which allows you to build and verify ink! smart contracts inside of an isolated environment, explore contract verification details.

![OpenBrush](https://user-images.githubusercontent.com/88630083/218825486-accc2d8c-bc5c-4b92-a278-a5b9009fd6f5.png)

[![Docs](https://img.shields.io/badge/docs-%F0%9F%93%84-blue)](https://Brushfam.github.io/openbrush-contracts)
[![telegram chat](https://img.shields.io/badge/Telegram-blue.svg?style=flat-square)](https://t.me/openbrush)
[![element chat](https://img.shields.io/badge/Element-green.svg?style=flat-square)](https://matrix.to/#/!utTuYglskDvqRRMQta:matrix.org?via=matrix.org&via=t2bot.io&via=matrix.parity.io)
[![discord chat](https://img.shields.io/badge/Discord-purple.svg?style=flat-square)](https://discord.com/invite/EARg6RCThP)

OpenBrush is maintained by the [Brushfam](https://www.brushfam.io/) team, and was created to make ink! development faster, safer and easier. We plan to integrate most of the features OpenBrush into ink!. OpenBrush provides documentation with FAQ section.

If you have any questions regarding OpenBrush, you can join the [Brushfam Element channel](https://matrix.to/#/!utTuYglskDvqRRMQta:matrix.org?via=matrix.org&via=t2bot.io&via=web3.foundation) to find your answers and meet other ink! smart contracts developers, or ask questions regarding ink! development on Element, Discord, or Telegram OpenBrush channels by the links above.

## Summary
**OpenBrush is a library for smart contract development on ink!.**

Why use this library?
- To make contracts **interoperable** to do **safe** cross-contracts calls (by having the same functions signature among every contracts)
- To ensure the usage of [Polkadot Standards Proposals](https://github.com/w3f/PSPs)
- To ensure the usage of the **latest & most secure** implementation
- Useful contracts that provide custom logic to be implemented in contracts
- To **save time** by not writing boilerplate code
- Useful features which can simplify development
- All contracts are upgradeable by default

Which Standard tokens & useful contracts does it provide?
- **PSP22** - Fungible Token (*ERC20 equivalent*) with extensions
- **PSP34** - Non-Fungible Token (*ERC721 equivalent*) with extensions
- **PSP37** - *ERC1155 equivalent* with extensions
- **Ownable** Restrict access to action for non-owners
- **Access Control** Define set of roles and restrict access to action by roles
- **Reentrancy guard** Prevent reentrant calls to a function
- **Pausable** Pause/Unpause the contract to disable/enable some operations
- **Timelock Controller** Execute transactions with some delay
- **Payment Splitter** Split amount of native tokens between participants

### Default implementation in ink! traits

You can provide a default implementation in the traits method and have internal functions. 
You can use the ink! trait as a native rust trait with several restrictions regarding 
external functions(functions marked `#[ink(message)]`).

```rust
#[openbrush::trait_definition]
pub trait Governance: AccessControl {
    #[ink(message)]
    fn execute(&mut self, transaction: Transaction) -> Result<(), GovernanceError> {
        self.internal_execute(transaction)
    }

    fn internal_execute(&mut self, transaction: Transaction) -> Result<(), GovernanceError> {
        ...
    }
}
```

### Modifiers

Solidity smart contracts provides modifiers to restrain function call to certain pre-defined parameters. OpenBrush provides attribute macros to use standardised modifiers.
You can use our useful contracts to use as modifiers, or define your own modifiers.

```rust
// Before execution of `mint` method, `only_owner` should verify that caller is the owner.
#[ink(message)]
#[modifiers(only_owner)]
fn mint(&mut self, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
  self._mint_to(Self::env().caller(), ids_amounts)
}
```

### Wrapper around traits

You are enough to have a trait definition
(you don't need directly a contract that implements that trait) 
to call methods of that trait from some contract in the network
(do a cross contract call).

```rust
// Somewhere defined trait
#[openbrush::trait_definition]
pub trait Trait1 {
    #[ink(message)]
    fn foo(&mut self) -> bool;
}

// You can create wrapper in the place where you defined the trait
// Or if you import **everything** from the file where you define trait
#[openbrush::wrapper]
type Trait1Ref = dyn Trait1;

{
    // It should be `AccountId` of contract in the network that implements `Trait1` trait
    let callee: openbrush::traits::AccountId = [1; 32].into();
    // This code will execute a cross contract call to `callee` contract
    let result_of_foo: bool = Trait1Ref::foo(&callee);
}
```

> **Note**: The trait should be defined with `openbrush::trait_definition`.
The callee contract should implement that trait.

### Additional stuff

- You can use [`test_utils`](https://github.com/Brushfam/openbrush-contracts/blob/main/lang/src/test_utils.rs#L39)
to simplify unit testing of you code.
- You can use [`traits`](https://github.com/Brushfam/openbrush-contracts/blob/main/lang/src/traits.rs) that provides some additional
functionality for your code.
- Read our **documentation** in [doc](https://learn.brushfam.io/docs/openbrush).
- Go through our **examples** in [examples](examples) to check hot to use the library and ink!.
- Check the [**example of project struct**](https://github.com/Brushfam/openbrush-contracts/tree/main/example_project_structure) and [according documentation](https://learn.brushfam.io/docs/OpenBrush/smart-contracts/example/overview).

Not sure where to start? Use [the interactive generator](https://openbrush.io) to bootstrap your contract and learn about the components offered in OpenBrush.

### ‼️ Important ‼️

Events are not supported currently due to how ink! currently handles them.  
The identifiers of events must be based on the name of the trait. At the moment, ink! doesn't support it,
but it must be fixed with this [issue](https://github.com/paritytech/ink/issues/809).

### Issues to be resolved before the library becomes production-ready:
* [Event's identifiers are based on the naming of the storage structure](https://github.com/Brushfam/openbrush-contracts/issues/2)

## Installation & Testing
To work with project you need to install ink! toolchain and NodeJS's dependencies.

1. So, you need an actual installer [rustup](https://www.rust-lang.org/tools/install).
2. [ink! toolchain](https://use.ink/getting-started/setup)
3. NodeJS deps you can install via `yarn` command

### Build
```
$ yarn build
```
If you want to build in release mode, you can use this command
```
$ yarn build:release
```

### Tests

You can run unit tests by `RUSTFLAGS="-D warnings" cargo test --workspace --features test-all -- --test-threads=10` command from the root of the directory.

To run integration test you need to start the node with contract-pallet.
- [Setup and start the node with contract-pallet](https://github.com/paritytech/substrate-contracts-node)

After you can run tests by `npm run test` command. It will build all contracts required for integration tests and run them.

## FAQ

### Was it audited?

OpenBrush was audited by OpenZeppelin team.

## License

OpenBrush is released under the [MIT License](LICENSE).
