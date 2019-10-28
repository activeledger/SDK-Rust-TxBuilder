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

use crate::error::{TxBuilderError, TxBuilderResult};

/// Holds the transactions data
#[derive(Debug, Clone)]
pub struct TransactionBody {
    entry: Option<Value>,
    contract: Value,
    namespace: Value,
    input: Value,
    output: Option<Value>,
    readonly: Option<Value>,
    json: Option<Value>,
}

impl TransactionBody {
    pub fn new(contract: Value, namespace: Value, input: Value) -> TransactionBody {
        // Init all as None to create base
        TransactionBody {
            entry: None,
            contract: contract,
            namespace: namespace,
            input: input,
            output: None,
            readonly: None,
            json: None,
        }
    }

    pub fn add(&mut self, key: &str, data: Value) -> &mut Self {
        match key {
            "entry" => self.entry = Some(data),
            "output" => self.output = Some(data),
            "readonly" => self.readonly = Some(data),
            _ => unreachable!(),
        };

        self
    }

    pub fn build(&mut self) -> Value {
        let mut json = json!({});

        json["$contract"] = json!(self.contract.clone());
        json["$namespace"] = json!(self.namespace.clone());
        json["$i"] = self.input.clone();

        if let Some(entry) = &self.entry {
            json["$entry"] = json!(entry);
        }

        if let Some(output) = &self.output {
            json["$o"] = json!(output);
        }

        if let Some(readonly) = &self.readonly {
            json["$r"] = json!(readonly);
        }

        self.json = Some(json.clone());

        json
    }

    pub fn get(&mut self) -> TxBuilderResult<Value> {
        match &self.json {
            Some(json) => Ok(json.clone()),
            None => Err(TxBuilderError::TxBodyError(000)),
        }
    }
}
