//! Test the JSON-RPC API against `bitcoind v0.18.1`.

#![cfg(all(feature = "client-sync", feature = "v18"))]

mod client;

client::impl_constructors!();

// == Blockchain ==
mod blockchain {
    use super::*;

    crate::impl_test_v17__getblockchaininfo!();
    crate::impl_test_v17__getbestblockhash!();
    crate::impl_test_v17__getblock!();
}

// == Control ==
mod control {
    // use super::*;

    // crate::impl_test_v17__stop!();
}

// == Generating ==
mod generating {
    use super::*;

    crate::impl_test_v17__generatetoaddress!();
}

// == Network ==
mod network {
    use super::*;

    crate::impl_test_v17__getnetworkinfo!();
}

// == Wallet ==
mod wallet {
    use super::*;

    crate::impl_test_v17__createwallet!();
    crate::impl_test_v17__loadwallet!();

    crate::impl_test_v17__getnewaddress!();
    crate::impl_test_v17__getbalance!();
    crate::impl_test_v17__sendtoaddress!();
}
