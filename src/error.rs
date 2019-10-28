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

//! # Transaction Builder Error definitions

use std::error::Error;
use std::fmt;

/// KeyResult definition - Shorthand for: Result<T, TxBuilderError>
pub type TxBuilderResult<T> = Result<T, TxBuilderError>;

/// KeyError data holder
#[derive(Debug)]
pub enum TxBuilderError {
    BuildError(u16),      // 1000
    JsonError(u16),       // 2000
    PacketError(u16),     // 3000
    TxBodyError(u16),     // 4000
    TxBuildError(u16),    // 5000
    TxGenerateError(u16), // 6000
    KeyError(u16),        // 7000
}

impl fmt::Display for TxBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TxBuilderError::BuildError(ref code) => {
                let error = TxBuilderErrorHandler::get_build_error(code);
                write!(f, " Error - {} : {}", code, error)
            }

            TxBuilderError::JsonError(ref code) => {
                let error = TxBuilderErrorHandler::get_json_error(code);
                write!(f, " Error - {} : {}", code, error)
            }

            TxBuilderError::PacketError(ref code) => {
                let error = TxBuilderErrorHandler::get_packet_error(code);
                write!(f, " Error - {} : {}", code, error)
            }

            TxBuilderError::TxBodyError(ref code) => {
                let error = TxBuilderErrorHandler::get_txbody_error(code);
                write!(f, " Error - {} : {}", code, error)
            }

            TxBuilderError::TxBuildError(ref code) => {
                let error = TxBuilderErrorHandler::get_txbuild_error(code);
                write!(f, " Error - {} : {}", code, error)
            }

            TxBuilderError::TxGenerateError(ref code) => {
                let error = TxBuilderErrorHandler::get_txgenerate_error(code);
                write!(f, " Error - {} : {}", code, error)
            }

            TxBuilderError::KeyError(ref code) => {
                let error = TxBuilderErrorHandler::get_key_error(code);
                write!(f, " Error - {} : {}", code, error)
            }
        }
    }
}

impl Error for TxBuilderError {}

struct TxBuilderErrorHandler;

impl TxBuilderErrorHandler {
    fn get_build_error(code: &u16) -> &str {
        match code {
            1000 => "Error building the transaction packet",
            _ => "Unknown Error",
        }
    }

    fn get_json_error(code: &u16) -> &str {
        match code {
            2000 => "Error converting array to JSON",
            2001 => "Error converting object to JSON",
            _ => "Unknown Error",
        }
    }

    fn get_packet_error(code: &u16) -> &str {
        match code {
            3000 => "Error getting string from packet data",
            3001 => "Error getting JSON from packet data",
            _ => "Unknown Error",
        }
    }

    fn get_txbody_error(code: &u16) -> &str {
        match code {
            4000 => "No transaction body",
            _ => "Unknown Error",
        }
    }

    fn get_txbuild_error(code: &u16) -> &str {
        match code {
            5000 => "No transaction data",
            5001 => "Error fetching input from PacketData",
            5002 => "Error fetching output from PacketData",
            5003 => "Error fetching readonly from PacketData",
            5004 => "No packet data to sign",
            5005 => "Packet data not built yet",
            5006 => "Contract not set",
            5007 => "Namespace not set",
            5008 => "Input not set",
            _ => "Unknown Error",
        }
    }

    fn get_txgenerate_error(code: &u16) -> &str {
        match code {
            6000 => "Error generating RSA key",
            6001 => "Error generating Elliptic Curve key",
            _ => "Unknown Error",
        }
    }

    fn get_key_error(code: &u16) -> &str {
        match code {
            7000 => "Error signing data with Elliptic Curve key",
            7001 => "Error signing data with RSA key",
            7002 => "Error getting keys PEM",
            _ => "Unknown Error",
        }
    }
}
