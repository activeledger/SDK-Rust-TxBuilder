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

// STD
use std::collections::HashMap;

// External imports
use activeledger::key::{EllipticCurve, RSA};
use serde_json::{json, Value};

// Internal imports
use super::body::TransactionBody;
use crate::error::{TxBuilderError, TxBuilderResult};
use crate::packet_builder::{Input, Output, Readonly};
use crate::Signees;
use crate::{packet_data, signees};

/// Holds the key to use when signing the transaction packet
#[derive(Clone)]
pub enum Key {
    Rsa(RSA),
    Ec(EllipticCurve),
}

/// Key Type for generating a key and onboarding it
pub enum KeyType {
    RSA,
    EC,
}

/// # Transaction builder
///
/// The transaction builder is used to help build a compatible Activeledger transaction object.
/// To read more about Activeledger transactions you can read the documentation [here.](https://github.com/activeledger/activeledger/blob/master/docs/en-gb/transactions.md)
///
/// This section will guide you through the creation of transaction using this crate.
///
/// ## Transaction structure
/// Lets first have a look at the structure of a transaction.
/// ```json
/// {
///     "$territoriality" : "",
///     "$tx": {
///         "$namespace": "[contract namespace location]"
///         "$contract": "[contract id]"
///         "$entry": "[contract entry point]"
///         "$i": {
///             "[streamid]": {"input data": "here"}
///         },
///         "$o": {},
///         "$r": {}
///     },
///     "$selfsign" : false,
///     "$sigs": {
///         "[streamid]" : "key public pem"
///     }
///
/// }
/// ```
/// We won't go into much detail about all of the separate parts here as that is in documentation linked
/// above.
/// However, it is useful to know how that structure is broken down in terms of this helper.
///
/// This helper breaks the above structure down into two sections.
/// 1. The overall transaction - Everything in the object
/// 2. The transaction packet - everything inside the $tx object, this gets signed
///
/// When using this helper to create a transaction you must first create the packet as that is passed
/// to the main builder. You can create three packets for the three sub objects inside of the packet:
/// $i (input), $o (output), and $r (readonly).
///
/// ## Examples
/// ### Minimal
/// This example will go over creating the most minimal transaction.
///
/// **Note:** This example does include some bootstrapping as we need to generate a key.
/// You may already have a key and very likely will want to reuse it.
/// ```
/// use activeledger::key::EllipticCurve;
/// use active_tx::{PacketBuilder, TransactionBuilder, Key, packet_data, signees};
///
/// // Bootstrapping, we need a key to sign the transaction packet
/// let key = EllipticCurve::new("name").unwrap();
/// let key = Key::Ec(key);
///
/// // You can also wrap the creation call in the Key value
/// // let key = Key::Ec(EllipticCurve::new("name").unwrap());
///
/// // Using the signees macro we can create a Signees struct
/// // This stores a map of keys and the assigned streamid and is used to sign
/// // the packet later.
/// let signees = signees![{"streamid" => key}];
///
/// // Next we need to create the input data, this is the data that will be inside $i: {}
/// // To do this we use the included packet_data macro
/// let input = packet_data!(
///     {
///         "[streamid]" : {"input": "data"}
///     }
/// );
///
/// // Now we need to take the PacketValue created by the macro and pass it to the builder
/// // The builder will convert it to a String and a serde_json Value and store both.
/// // Should you wish to do something with this data after it is built you can retrieve it
/// // using the corresponding methods.
/// let mut input_builder = PacketBuilder::new(input);
/// let input_data = input_builder.build().unwrap();
///
/// // The build method can also be chained onto the creation call
/// // let input_data = PacketBuilder::new(input).build().unwrap();
///
/// // Now that we have the packet sorted out we need to pass the data to the transaction builder.
/// // The transaction must contain a namespace and contract so these are passed directly into
/// // the creation method.
/// // To add the input data we call the input() method and pass it the input_data from earlier.
/// //
/// // Now the builder has all the data it needs to build the contract.
/// // Calling the build function we pass it the signees we defined earlier, the keys will be used
/// // to sign the packet once it has been built.
/// // Calling the .build() method will return a string of the transaction.
/// // This string can be sent to the ledger!
/// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
/// tx_builder.input(input_data).unwrap();
/// let tx = tx_builder.build(signees).unwrap();
///
/// // To generate the transaction all in one go you can chain the methods like so
/// // let tx = TransactionBuilder::new("namespace", "contract")
/// //    .input(input_data)
/// //    .unwrap()
/// //    .build(signees)
/// //    .unwrap();
/// ```
/// ### Additional data
///
/// The additional data is:
///
/// **Packet**
/// * Output
/// * Readonly
/// * Entry
///
/// **Transaction**
/// * Territoriality
/// * Selfsign
///
/// Adding in this extra data is straight forward. It goes without saying that they should be added
/// before calling the build method.
///
/// **Note:** For the sake of space the required data has not been added to the following examples.
///
/// #### Packet
/// ##### Output
/// The output can be generated using the exact same method as the input in the full example
/// ```
/// # use active_tx::{packet_data, PacketBuilder, TransactionBuilder};
/// let output_data = packet_data!({"": ""});
/// let output = PacketBuilder::new(output_data).build().unwrap();
///
/// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
/// tx_builder.output(output);
/// ```
///
/// ##### Readonly
/// The readonly data can be generated using the exact same method as the input in the full example
/// ```
/// # use active_tx::{packet_data, PacketBuilder, TransactionBuilder};
/// let readonly_data = packet_data!({"": ""});
/// let readonly = PacketBuilder::new(readonly_data).build().unwrap();
///
/// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
/// tx_builder.readonly(readonly);
/// ```
///
/// ##### Entry
/// As the entry value is a string we can pass it directly into the entry method without needing to
/// use the [`PacketBuilder`].
///
/// ```
/// # use active_tx::{packet_data, PacketBuilder, TransactionBuilder};
/// #
/// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
/// tx_builder.entry("entry point");
/// ```
///
/// #### Transaction
/// ##### Territoriality
/// ```
/// # use active_tx::{packet_data, PacketBuilder, TransactionBuilder};
/// #
/// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
/// tx_builder.territoriality("territory");
/// ```
///
/// ##### Selfsign
/// Calling this function will set the selfsign value of the transaction to true
/// ```
/// # use active_tx::{packet_data, PacketBuilder, TransactionBuilder};
/// #
/// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
/// tx_builder.selfsign();
/// ```
///
/// [`PacketBuilder`]: struct.PacketBuilder.html

pub struct TransactionBuilder {
    /*
    Data for $tx object
    entry,
    contract,
    namespace,
    input,
    output,
    readonly
    */
    packet_data: HashMap<String, Value>,

    /*
    territoriality,
    selfsign,
    */
    tx_data: HashMap<String, Value>,

    // Generation and storage holders
    packet: Option<TransactionBody>,
    tx: Option<Value>,
    sigs: HashMap<String, String>,
}

// Public functions
impl TransactionBuilder {
    /// # Builder with namespace and contract
    ///
    /// Create a builder with predetermined namespace and contract.
    ///
    /// Required data: Input
    ///
    /// ```
    /// # use active_tx::TransactionBuilder;
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    /// ```
    ///
    /// It is required that input data be added to the builder before it will build the transaction.
    ///
    /// Additional data can be added using the other transaction builder methods.
    /// Once any additional data has been added, as well as the required input data,
    /// the build function can be run to generate the transaction and return a string of
    /// the transaction.
    /// The get method can be run to get the string again.
    ///
    /// Most of the methods can be chained
    pub fn new(namespace: &str, contract: &str) -> TransactionBuilder {
        let mut packet_data = HashMap::new();

        packet_data.insert("namespace".to_string(), json!(namespace));
        packet_data.insert("contract".to_string(), json!(contract));

        TransactionBuilder {
            packet_data,
            tx_data: HashMap::new(),
            packet: None,
            tx: None,
            sigs: HashMap::new(),
        }
    }

    /// # Blank Builder
    ///
    /// Create a builder that has no data.
    ///
    /// Required data: Input, Contract, Namespace
    ///
    /// ```
    /// # use active_tx::TransactionBuilder;
    /// let mut tx_builder = TransactionBuilder::new_blank();
    /// ```
    ///
    /// It is required that contract, namespace, and input data be added to the builder before it will build the transaction.
    ///
    /// All data can be added by the other methods provided by the builder.
    ///
    /// Most of the methods can be chained
    pub fn new_blank() -> TransactionBuilder {
        TransactionBuilder {
            packet_data: HashMap::new(),
            tx_data: HashMap::new(),
            packet: None,
            tx: None,
            sigs: HashMap::new(),
        }
    }

    /// # Transaction String
    ///
    /// Get the built transaction as a string.
    /// Note that the build method returns the same data.
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key, signees};
    /// # use activeledger::key::EllipticCurve;
    ///
    /// let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    ///
    /// let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    ///
    /// let signees = signees![{"streamid" => key}];
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    /// tx_builder
    ///     .input(input)
    ///     .unwrap()
    ///     .build(signees)
    ///     .unwrap();
    ///
    /// let tx = tx_builder.get().unwrap();
    /// ```
    pub fn get(&self) -> TxBuilderResult<String> {
        match &self.tx {
            Some(tx) => Ok(tx.to_string()),
            None => Err(TxBuilderError::TxBuildError(5000)),
        }
    }

    /// # Transaction JSON
    ///
    /// Get the built transaction as a Serde JSON value
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key, signees};
    /// # use activeledger::key::EllipticCurve;
    ///
    /// let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    ///
    /// let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    ///
    /// let signees = signees![{"streamid" => key}];
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    /// tx_builder
    ///     .input(input)
    ///     .unwrap()
    ///     .build(signees)
    ///     .unwrap();
    ///     
    /// let tx = tx_builder.get_json().unwrap();
    /// ```
    pub fn get_json(&self) -> TxBuilderResult<Value> {
        match &self.tx {
            Some(tx) => Ok(tx.clone()),
            None => Err(TxBuilderError::TxBuildError(5000)),
        }
    }

    /// # Territoriality
    ///
    /// Set the territoriality value
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key};
    /// # use activeledger::key::EllipticCurve;
    /// # let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    /// # let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    ///
    /// tx_builder.territoriality("territory");
    /// ```
    pub fn territoriality(&mut self, territoriality: &str) -> &mut Self {
        self.tx_data.insert(
            String::from("territoriality"),
            json!(territoriality.to_string()),
        );

        self
    }

    /// # Entry
    ///
    /// Set the entry value
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key};
    /// # use activeledger::key::EllipticCurve;
    /// # let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    /// # let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    ///
    /// tx_builder.entry("entry");
    /// ```
    pub fn entry(&mut self, entry: &str) -> &mut Self {
        self.packet_data
            .insert(String::from("entry"), json!(entry.to_string()));

        self
    }

    /// # Contract
    ///
    /// Set the contract value
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key};
    /// # use activeledger::key::EllipticCurve;
    /// # let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    /// # let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    ///
    /// let mut tx_builder = TransactionBuilder::new_blank();
    ///
    /// tx_builder.contract("contract");
    /// ```
    pub fn contract(&mut self, contract: &str) -> &mut Self {
        self.packet_data
            .insert(String::from("contract"), json!(contract.to_string()));

        self
    }

    /// # Namespace
    ///
    /// Set the namespace value
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key};
    /// # use activeledger::key::EllipticCurve;
    /// # let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    /// # let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    ///
    /// let mut tx_builder = TransactionBuilder::new_blank();
    ///
    /// tx_builder.namespace("namespace");
    /// ```
    pub fn namespace(&mut self, namespace: &str) -> &mut Self {
        self.packet_data
            .insert(String::from("namespace"), json!(namespace.to_string()));

        self
    }

    /// # Input
    ///
    /// Set the input value
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key};
    /// # use activeledger::key::EllipticCurve;
    /// # let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    ///
    /// let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    /// tx_builder.input(input);
    /// ```
    pub fn input(&mut self, input: Input) -> TxBuilderResult<&mut Self> {
        match input.get() {
            Ok(data) => self.packet_data.insert("input".to_string(), data),
            Err(_) => return Err(TxBuilderError::TxBuildError(5001)),
        };

        Ok(self)
    }

    /// # Output
    ///
    /// Set the input value
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key};
    /// # use activeledger::key::EllipticCurve;
    /// # let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    ///
    /// let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    /// tx_builder.input(input);
    /// ```
    pub fn output(&mut self, output: Output) -> TxBuilderResult<&mut Self> {
        match output.get() {
            Ok(data) => self.packet_data.insert("output".to_string(), data),
            Err(_) => return Err(TxBuilderError::TxBuildError(5002)),
        };

        Ok(self)
    }

    /// # Readonly
    ///
    /// Set the input value
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key};
    /// # use activeledger::key::EllipticCurve;
    /// # let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    ///
    /// let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    /// tx_builder.input(input);
    /// ```
    pub fn readonly(&mut self, readonly: Readonly) -> TxBuilderResult<&mut Self> {
        match readonly.get() {
            Ok(data) => self.packet_data.insert("readonly".to_string(), data),
            Err(_) => return Err(TxBuilderError::TxBuildError(5003)),
        };

        Ok(self)
    }

    /// # Selfsign
    ///
    /// Set selfsign to true
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key};
    /// # use activeledger::key::EllipticCurve;
    /// # let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    /// # let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    /// tx_builder.input(input).unwrap();
    ///
    /// tx_builder.selfsign();
    /// ```
    pub fn selfsign(&mut self) -> &mut Self {
        self.tx_data
            .insert(String::from("selfsign"), json!(String::from("true")));

        self
    }

    /// # Sign
    ///
    /// Using a given key and stream ID sign the transaction data packet.
    /// Generally this is used to add more signatures to a transaction, as it requires the build
    /// method to be run first.
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key, signees};
    /// # use activeledger::key::EllipticCurve;
    /// # let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    ///
    /// let streamid = "id";
    /// let streamid2 = "id2";
    ///
    /// let key = Key::Ec(EllipticCurve::new(streamid).unwrap());
    /// let key2 = Key::Ec(EllipticCurve::new(streamid2).unwrap());
    ///
    /// let signees = signees![{streamid => key}];
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    /// tx_builder.input(input)
    ///     .unwrap()
    ///     .build(signees)
    ///     .unwrap();
    ///
    /// let signees2 = signees![{streamid2 => key2}];
    ///
    /// tx_builder.sign(signees2);
    /// ```
    pub fn sign(&mut self, signees: Signees) -> TxBuilderResult<&mut Self> {
        let signees_array = signees.get();

        let packet = match self.packet.clone() {
            Some(mut packet) => packet.get()?,
            None => return Err(TxBuilderError::TxBuildError(5004)),
        };
        let packet = packet.clone();

        for signee in signees_array.iter() {
            let signature =
                TransactionBuilder::sign_internal(&packet.to_string(), signee.key.clone())?;
            self.sigs.insert(signee.streamid.clone(), signature);
        }

        let json = match &self.tx {
            Some(json) => json,
            None => return Err(TxBuilderError::TxBuildError(5005)),
        };

        let mut json = json.clone();

        json["$sigs"] = json!(self.sigs.clone());

        self.tx.replace(json.clone());

        Ok(self)
    }

    /// # Build
    ///
    /// Using the data provided, compile it into the correct form for a transaction.
    /// Returns a transaction in the form of a string.
    ///
    /// ```
    /// # use active_tx::{TransactionBuilder, packet_data, PacketBuilder, Key, signees};
    /// # use activeledger::key::EllipticCurve;
    /// # let input = PacketBuilder::new(packet_data!({"data": "data"})).build().unwrap();
    ///
    /// let streamid = "id";
    /// let key = Key::Ec(EllipticCurve::new(streamid).unwrap());
    ///
    /// let mut tx_builder = TransactionBuilder::new("namespace", "contract");
    /// tx_builder.input(input).unwrap();
    ///
    /// let signees = signees![{streamid => key}];
    ///
    /// let tx = tx_builder.build(signees).unwrap();
    ///
    /// ```
    pub fn build(&mut self, signees: Signees) -> TxBuilderResult<String> {
        let mut json = json!({});

        // Contract, namespace and input are all required, if any are missing throw an error
        let contract = match self.packet_data.get("contract") {
            Some(contract) => contract,
            None => return Err(TxBuilderError::TxBuildError(5006)),
        };

        let namespace = match self.packet_data.get("namespace") {
            Some(namespace) => namespace,
            None => return Err(TxBuilderError::TxBuildError(5007)),
        };

        let input = match self.packet_data.get("input") {
            Some(input) => input,
            None => return Err(TxBuilderError::TxBuildError(5008)),
        };

        let mut tx = TransactionBody::new(contract.clone(), namespace.clone(), input.clone());

        let checked = ["contract", "namespace", "input"];

        // Loop packet_data map and add additional data
        for (key, val) in self.packet_data.iter() {
            // Ignore if key in checked
            if !checked.iter().any(|v| v == &key) {
                tx.add(key, val.clone());
            }
        }

        self.packet.replace(tx.clone());

        let built_packet = tx.build();
        self.packet = Some(tx);

        json["$tx"] = built_packet.clone();

        let signees_array = signees.get();

        for signee in signees_array.iter() {
            let signature =
                TransactionBuilder::sign_internal(&built_packet.to_string(), signee.key.clone())?;
            self.sigs.insert(signee.streamid.clone(), signature);
        }

        json["$sigs"] = json!(self.sigs.clone());

        for &e in &["territoriality", "selfsign"] {
            if let Some(data) = self.tx_data.get(e) {
                let key = format!("${}", e);

                json[key] = data.clone();
            }
        }

        self.tx.replace(json.clone());

        Ok(json.to_string())
    }

    /// # Onboard transaction
    ///
    /// Given a key, generate a transaction to onboard the key to the ledger.
    ///
    /// ```
    /// # use activeledger::key::EllipticCurve;
    /// # use active_tx::{TransactionBuilder, Key};
    ///
    /// let key = Key::Ec(EllipticCurve::new("keyname").unwrap());
    ///
    /// let tx = TransactionBuilder::onboard_tx(key).unwrap();
    /// ```
    pub fn onboard_tx(key: Key) -> TxBuilderResult<String> {
        // Create an onboarding transaction for the given key

        let (key_name, key_type) = match &key {
            Key::Rsa(key) => (key.name.clone(), "rsa"),
            Key::Ec(key) => (key.name.clone(), "secp256k1"),
        };

        let pem = TransactionBuilder::get_pem(key.clone())?;

        let input = packet_data!({
            key_name.clone(): {
                "type": key_type,
                "publicKey": pem
            }
        });

        let mut input_builder = crate::PacketBuilder::new(input);
        let input = input_builder.build()?;

        let signees = signees!(key);

        let mut tx_builder = TransactionBuilder::new("default", "onboard");
        let tx = tx_builder.selfsign().input(input)?.build(signees)?;

        Ok(tx.to_string())
    }

    /// # Onboard transaction
    ///
    /// Given a key type and name, generate a key and use it to build a transaction to onboard that key to the ledger.
    ///
    /// Returns the generated key and the transaction
    /// ```
    /// # use active_tx::{TransactionBuilder, KeyType};
    ///
    /// let (key, tx) = TransactionBuilder::generate_onboard_tx(KeyType::EC, "keyname").unwrap();
    /// ```
    pub fn generate_onboard_tx(
        key_type: KeyType,
        key_name: &str,
    ) -> TxBuilderResult<(Key, String)> {
        // Generate a key and onboard it

        let key = match key_type {
            KeyType::RSA => {
                let key = match RSA::new(key_name) {
                    Ok(key) => key,
                    Err(_) => return Err(TxBuilderError::TxGenerateError(6000)),
                };
                Key::Rsa(key)
            }
            KeyType::EC => {
                let key = match EllipticCurve::new(key_name) {
                    Ok(key) => key,
                    Err(_) => return Err(TxBuilderError::TxGenerateError(6001)),
                };
                Key::Ec(key)
            }
        };

        let tx = TransactionBuilder::onboard_tx(key.clone())?;

        Ok((key, tx))
    }
}

// Private functions
impl TransactionBuilder {
    /// Match key type then pass to signing function
    fn sign_internal(data: &str, key: Key) -> TxBuilderResult<String> {
        match key {
            Key::Rsa(key) => TransactionBuilder::sign_rsa(data, key),
            Key::Ec(key) => TransactionBuilder::sign_ec(data, key),
        }
    }

    /// Sign data using elliptic curve
    fn sign_ec(tx: &str, key: EllipticCurve) -> TxBuilderResult<String> {
        let signature = match key.sign(&tx.to_string()) {
            Ok(sig) => sig,
            Err(_) => return Err(TxBuilderError::KeyError(7000)),
        };

        Ok(signature)
    }

    /// Sign data using RSA
    fn sign_rsa(tx: &str, key: RSA) -> TxBuilderResult<String> {
        let signature = match key.sign(&tx.to_string()) {
            Ok(sig) => sig,
            Err(_) => return Err(TxBuilderError::KeyError(7001)),
        };

        Ok(signature)
    }

    /// Get the keys public PEM string
    fn get_pem(key: Key) -> TxBuilderResult<String> {
        let pkcs8pem = match key {
            Key::Rsa(key) => key.get_pem(),
            Key::Ec(key) => key.get_pem(),
        };

        match pkcs8pem {
            Ok(pem) => Ok(pem.public),
            Err(_) => Err(TxBuilderError::KeyError(7002)),
        }
    }
}
