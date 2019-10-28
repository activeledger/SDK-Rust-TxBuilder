/*
 * MIT License (MIT)
 * Copyright (c) 2019 Activeledger
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! # Transaction Builder
//!
//! The transaction builder provides methods to aid with building up a transaction correctly.
//!
//! ## Example
//!
//! ```
//! # use active_tx::{PacketBuilder, TransactionBuilder, Key, packet_data, signees};
//! # use activeledger::key::EllipticCurve;
//! # fn main() {
//! let input = packet_data!({"[streamid]": {"input": "data"}});
//!
//! let input_data = PacketBuilder::new(input).build().unwrap();
//!  
//! let mut tx_builder = TransactionBuilder::new("namespace", "contract");
//! #
//! # let streamid = "";
//! # let key = Key::Ec(EllipticCurve::new("").unwrap());
//! #
//!
//! let signees = signees![{streamid => key}];
//!
//! let tx = tx_builder
//!     .input(input_data)
//!     .unwrap()
//!     .build(signees)
//!     .unwrap();
//! # }
//! ```
//!

mod body;
mod builder;
mod signee;

pub use builder::{Key, KeyType, TransactionBuilder};
pub use signee::Signees;

#[cfg(test)]
mod tests {
    use crate::*;
    use activeledger::key::EllipticCurve;

    #[test]
    fn tx_min() {
        let input = packet_data!({"input": "data"});

        let built_input = PacketBuilder::new(input).build().unwrap();

        let mut transaction_builder = TransactionBuilder::new("namespace", "contract");

        let streamid = "test";
        let key = Key::Ec(EllipticCurve::new(streamid).unwrap());

        let signees = signees![{streamid => key}];

        let tx = transaction_builder
            .input(built_input)
            .unwrap()
            .build(signees)
            .unwrap();

        println!("\nMin:\n{}\n", tx);
    }

    #[test]
    fn tx_all() {
        let input = packet_data!({"input": "data"});
        let output = packet_data!({"output": "data"});
        let readonly = packet_data!({"readonly": "data"});

        let streamid = "test";
        let key = EllipticCurve::new(streamid).unwrap();

        let streamid2 = "test2";
        let key2 = EllipticCurve::new(streamid2).unwrap();

        let signees = signees![{streamid => Key::Ec(key)}, {streamid2 => Key::Ec(key2)}];

        let built_input = PacketBuilder::new(input).build().unwrap();
        let built_output = PacketBuilder::new(output).build().unwrap();
        let built_readonly = PacketBuilder::new(readonly).build().unwrap();

        let mut transaction_builder = TransactionBuilder::new("namespace", "contract");

        let tx = transaction_builder
            .entry("entry")
            .territoriality("terry")
            .selfsign()
            .input(built_input)
            .unwrap()
            .output(built_output)
            .unwrap()
            .readonly(built_readonly)
            .unwrap()
            .build(signees)
            .unwrap();

        println!("\nAll:\n{}\n", tx);
    }

    #[test]
    fn tx_onboard() {
        let key = Key::Ec(EllipticCurve::new("test").unwrap());

        let tx = TransactionBuilder::onboard_tx(key).unwrap();

        println!("\nOnboard\n{}", tx);
    }

    #[test]
    fn tx_onboard_generate() {
        let (_key, tx) = TransactionBuilder::generate_onboard_tx(KeyType::RSA, "testkey").unwrap();

        println!("\nOnboard generate\n{}", tx);
    }
}
