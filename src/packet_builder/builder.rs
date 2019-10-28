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

use serde_json::{json, Value};

// STD
use std::collections::HashMap;

// Internal
use super::PacketValue;
use crate::error::{TxBuilderError, TxBuilderResult};

/// Provides build methods
#[derive(Clone)]
pub struct PacketBuilder {
    data: PacketData,
}

/// Stores the data built by PacketBuilder
#[derive(Clone, Debug)]
pub struct PacketData {
    data: Option<PacketValue>,
    json: Option<Value>,
    is_json: bool,
    built: Option<String>,
}

impl PacketBuilder {
    /// # New
    ///
    /// Generate a new builder and pass it an [`PacketValue`] for consumption.
    pub fn new(data: PacketValue) -> PacketBuilder {
        let mut ior_data = PacketData::new();

        // If the data provided is an object (PacketValue::Object(HashMap)) use the add_map function
        // to add it.
        if let PacketValue::Object(data) = data {
            ior_data.add_map(data);
        } else {
            ior_data.add(data);
        }

        PacketBuilder { data: ior_data }
    }

    /// # New JSON
    ///
    /// Takes serde_json Value type data and creates a new PacketBuilder.
    pub fn new_json(data: Value) -> PacketBuilder {
        let mut ior_data = PacketData::new();

        ior_data.add_json(data);

        PacketBuilder { data: ior_data }
    }

    /// # Build
    ///
    /// Process the given data and store it in an [`PacketData`] object, return the [`PacketData`] object
    pub fn build(&mut self) -> TxBuilderResult<PacketData> {
        if self.data.is_json() {
            let json = self.data.get()?;

            let json = json.to_owned();

            self.data.set_built(json);
        } else {
            let map = match self.data.get_map() {
                Some(map) => map,
                None => return Err(TxBuilderError::BuildError(1000)),
            };

            let serialized = PacketBuilder::to_json(map)?;

            self.data.set_built(serialized);
        }

        Ok(self.data.clone())
    }

    /// # From string
    /// Consumes a string reference and converts it into a [`PacketValue`]
    pub fn from_string(data: &str) -> PacketValue {
        PacketValue::String(data.to_string())
    }
}

// Private functions
impl PacketBuilder {
    /// Walk an array value and convert it to a JSON Value
    fn array_tojson(array: &PacketValue) -> TxBuilderResult<Value> {
        let mut holder: Vec<Value> = Vec::new();
        match array {
            PacketValue::Array(array) => {
                for elem in array.iter() {
                    let data = match elem {
                        PacketValue::String(value) => json!(value),
                        PacketValue::Object(object) => PacketBuilder::object_tojson(object)?,
                        PacketValue::Array(_) => PacketBuilder::array_tojson(elem)?,
                    };
                    holder.push(data);
                }
            }
            _ => return Err(TxBuilderError::JsonError(2000)),
        };
        Ok(json!(holder))
    }

    /// Walk an object value and convert it to a JSON Value
    fn object_tojson(map: &HashMap<String, PacketValue>) -> TxBuilderResult<Value> {
        let mut json = json!({});

        for (key, value) in map.iter() {
            let data = match value {
                PacketValue::String(value) => json!(value),
                PacketValue::Object(object) => {
                    let data = match PacketBuilder::object_tojson(object) {
                        Ok(data) => data,
                        Err(_) => return Err(TxBuilderError::JsonError(2001)),
                    };

                    json!(data)
                }
                PacketValue::Array(_) => PacketBuilder::array_tojson(value)?,
            };

            json[key] = data;
        }

        Ok(json)
    }

    /// Convert a map to JSON
    fn to_json(map: &PacketValue) -> TxBuilderResult<Value> {
        let mut json = json!({});

        if let PacketValue::Object(map) = map {
            for (key, value) in map.iter() {
                let data: Value = match value {
                    PacketValue::String(value) => json!(value),
                    PacketValue::Object(object) => PacketBuilder::object_tojson(object)?,
                    PacketValue::Array(_) => PacketBuilder::array_tojson(value)?,
                };

                json[key] = data;
            }
        }

        Ok(json)
    }
}

// Public
impl PacketData {
    pub fn get_string(&self) -> TxBuilderResult<&str> {
        match &self.built {
            Some(data) => Ok(&data),
            None => Err(TxBuilderError::PacketError(3000)),
        }
    }

    pub fn get(&self) -> TxBuilderResult<Value> {
        if let Some(json) = &self.json {
            Ok(json.clone())
        } else {
            Err(TxBuilderError::PacketError(3001))
        }
    }
}

// Private
impl PacketData {
    fn new() -> PacketData {
        PacketData {
            data: None,
            json: None,
            is_json: false,
            built: None,
        }
    }

    fn add(&mut self, object: PacketValue) -> &mut Self {
        self.data = Some(object);

        self
    }

    fn add_map(&mut self, map: HashMap<String, PacketValue>) -> &mut Self {
        self.data = Some(PacketValue::Object(map));

        self
    }

    fn add_json(&mut self, json: Value) -> &mut Self {
        self.json = Some(json);
        self.is_json = true;
        self
    }

    fn is_json(&self) -> bool {
        self.is_json
    }

    fn get_map(&self) -> &Option<PacketValue> {
        match self.data {
            Some(_) => &self.data,
            None => &None,
        }
    }

    fn set_built(&mut self, data: Value) -> &mut Self {
        self.json = Some(data.clone());
        self.is_json = true;
        self.built = Some(data.to_string());

        self
    }
}
