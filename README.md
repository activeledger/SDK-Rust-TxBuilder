# Activeledger - Transaction Helper

<img src="https://www.activeledger.io/wp-content/uploads/2018/09/Asset-23.png" alt="Activeledger" width="300"/>

Activeledger is a powerful distributed ledger technology.
Think about it as a single ledger, which is updated simultaneously in multiple locations.
As the data is written to a ledger, it is approved and confirmed by all other locations.

[GitHub](https://github.com/activeledger/activeledger)

[NPM](https://www.npmjs.com/package/@activeledger/activeledger)

---

## This Crate

This crate acts as a helper for creating JSON transactions. It can be used in addition to the main [Activeledger Rust SDK](https://crates.io/crates/activeledger) or on its own.
This crate provides Rust developers an easy way to build up transactions without needing other crates, such as Serde.

This crate provides macros as well as builders that help create a transaction with the correct structure.
Additionally it provides two methods of creating a complete onboarding transaction. With and without a provided key.

## Additional Activeledger crates
Adhearing to the Rust mentality of keeping things small we have created other crates that can be used in conjunction
with this one to add additional functionality.

These crates are:
* [activeledger](https://github.com/activeledger/SDK-Rust) - The main SDK. ([Crate](https://crates.io/crates/activeledger))
* [active_sse](https://github.com/activeledger/SDK-Rust-Events) - To build transactions without worrying about the JSON. ([Crate](https://crates.io/crates/active_sse))

## Links
[Visit Activeledger.io](https://activeledger.io/)

[Read Activeledgers documentation](https://github.com/activeledger/activeledger/blob/master/docs/en-gb/README.md)

[Activeledger Developers portal](https://developers.activeledger.io)

[Activeledger on GitHub](https://github.com/activeledger/activeledger)

[Activeledger on NPM](https://www.npmjs.com/package/@activeledger/activeledger)

[This SDK on GitHub](https://github.com/activeledger/SDK-Rust-TxBuilder)

[Report Issues](https://github.com/activeledger/SDK-Rust-TxBuilder/issues)

## License

---

This project is licensed under the [MIT](https://github.com/activeledger/activeledger/blob/master/LICENSE) License
