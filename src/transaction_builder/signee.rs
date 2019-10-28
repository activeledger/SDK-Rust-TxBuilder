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

use crate::Key;

/// Holds an array of Signees
#[derive(Clone)]
pub struct Signees {
    keys: Vec<Signee>,
}

/// Holds the key and stream ID for use when signing the transaction packet.
#[derive(Clone)]
pub struct Signee {
    pub streamid: String,
    pub key: Key,
}

/// # Signees
///
/// These structs are intended to make it simpler to sign transaction packets.
///
/// The Signees object holds a vector of Signee's, the Signee object contains the streamid and key.
/// *Note:* The streamid could also be the key name for a selfsign transaction e.g onboarding.
///
/// It is recommended that the macro [signees!][macro] is used to generate these structures.
/// To see example usage the macro please see its relevant documentation [here][macro]
///
/// [macro]: macro.signees.html
impl Signees {
    /// # New
    ///
    /// This method generates a new Signees struct
    pub fn new() -> Signees {
        Signees { keys: vec![] }
    }

    /// # Add
    ///
    /// This method is the general add method.
    /// It takes a key and a stream id, the stream id must match one provided in the input ($i) of
    /// the transaction packet.
    pub fn add(&mut self, key: Key, streamid: &str) -> &mut Self {
        let signee = Signee {
            streamid: streamid.to_string(),
            key,
        };

        self.keys.push(signee);

        self
    }

    /// # Selfsign
    ///
    /// This method is use for transactions that will be selfsigned.
    /// Instead of storing a related stream id it will use the keys name. This must still have a
    /// corresponding match in the input of the transaction packet.
    pub fn add_selfsign(&mut self, key: Key) -> &mut Self {
        let name = match &key {
            Key::Ec(key) => key.name.clone().to_string(),
            Key::Rsa(key) => key.name.clone().to_string(),
        };

        let signee = Signee {
            streamid: name,
            key,
        };

        self.keys.push(signee);

        self
    }

    /// # Get
    ///
    /// This method returns a vector of Signee structs.
    /// The Signee struct contains the streamid and key given via the .add() (or add_selfsign) method above.
    pub fn get(&self) -> Vec<Signee> {
        self.keys.clone()
    }
}
