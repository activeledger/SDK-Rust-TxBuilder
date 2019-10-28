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

// External
use serde::Serialize;

// STD
use std::collections::HashMap;

pub type Input = PacketData;
pub type Output = PacketData;
pub type Readonly = PacketData;

mod builder;

pub use builder::{PacketBuilder, PacketData};

/// Holds recursive values for the $i (input), $o (output), and $r (readonly) objects of a transaction packet.
#[derive(Serialize, PartialEq, Eq, Debug, Clone)]
pub enum PacketValue {
    String(String),
    Array(Vec<PacketValue>),
    Object(HashMap<String, PacketValue>),
}

#[cfg(test)]
mod tests {

    use crate::*;
    use serde_json::json;

    #[test]
    fn input_macro() {
        let object = packet_data!({"array": ["array", "of", "strings"], "subobj": {"object style" : "in brackets"}});
        let mut builder = PacketBuilder::new(object);

        let input = builder.build().unwrap();

        println!("Macro: \n{}\n", input.get().unwrap());
    }

    #[test]
    fn input_json() {
        let json = json!({"I am": "json", "heres": ["an", "array"], "andbool": true});
        let mut builder = PacketBuilder::new_json(json);

        let input = builder.build().unwrap();

        println!("Json: \n{}\n", input.get().unwrap());
    }
}
