// SPDX-License-Identifier: CC0-1.0

use std::{error, fmt, io};

use bitcoin::{hex, secp256k1};

/// The error type for errors produced in this library.
#[derive(Debug)]
pub enum Error {
    JsonRpc(jsonrpc::error::Error),
    Hex(hex::HexToBytesError),
    Json(serde_json::error::Error),
    BitcoinSerialization(bitcoin::consensus::encode::FromHexError),
    Secp256k1(secp256k1::Error),
    Io(io::Error),
    InvalidAmount(bitcoin::amount::ParseAmountError),
    InvalidCookieFile,
    /// The JSON result had an unexpected structure.
    UnexpectedStructure,
    /// The daemon returned an error string.
    Returned(String),
    /// The server version did not match what was expected.
    ServerVersion(UnexpectedServerVersionError),
}

impl From<jsonrpc::error::Error> for Error {
    fn from(e: jsonrpc::error::Error) -> Error { Error::JsonRpc(e) }
}

impl From<hex::HexToBytesError> for Error {
    fn from(e: hex::HexToBytesError) -> Error { Error::Hex(e) }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Error { Error::Json(e) }
}

impl From<bitcoin::consensus::encode::FromHexError> for Error {
    fn from(e: bitcoin::consensus::encode::FromHexError) -> Error { Error::BitcoinSerialization(e) }
}

impl From<secp256k1::Error> for Error {
    fn from(e: secp256k1::Error) -> Error { Error::Secp256k1(e) }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error { Error::Io(e) }
}

impl From<bitcoin::amount::ParseAmountError> for Error {
    fn from(e: bitcoin::amount::ParseAmountError) -> Error { Error::InvalidAmount(e) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match *self {
            JsonRpc(ref e) => write!(f, "JSON-RPC error: {}", e),
            Hex(ref e) => write!(f, "hex decode error: {}", e),
            Json(ref e) => write!(f, "JSON error: {}", e),
            BitcoinSerialization(ref e) => write!(f, "Bitcoin serialization error: {}", e),
            Secp256k1(ref e) => write!(f, "secp256k1 error: {}", e),
            Io(ref e) => write!(f, "I/O error: {}", e),
            InvalidAmount(ref e) => write!(f, "invalid amount: {}", e),
            InvalidCookieFile => write!(f, "invalid cookie file"),
            UnexpectedStructure => write!(f, "the JSON result had an unexpected structure"),
            Returned(ref s) => write!(f, "the daemon returned an error string: {}", s),
            ServerVersion(ref e) => write!(f, "server version: {}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;

        match *self {
            JsonRpc(ref e) => Some(e),
            Hex(ref e) => Some(e),
            Json(ref e) => Some(e),
            BitcoinSerialization(ref e) => Some(e),
            Secp256k1(ref e) => Some(e),
            Io(ref e) => Some(e),
            InvalidAmount(ref e) => Some(e),
            ServerVersion(ref e) => Some(e),
            InvalidCookieFile | UnexpectedStructure | Returned(_) => None,
        }
    }
}

/// Error returned when RPC client expects a different version than bitcoind reports.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UnexpectedServerVersionError {
    /// Version from server.
    pub got: usize,
    /// Expected server version.
    pub expected: usize,
}

impl fmt::Display for UnexpectedServerVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected bitcoind version, got: {} expected: {}", self.got, self.expected)
    }
}

impl error::Error for UnexpectedServerVersionError {}

impl From<UnexpectedServerVersionError> for Error {
    fn from(e: UnexpectedServerVersionError) -> Self { Self::ServerVersion(e) }
}
