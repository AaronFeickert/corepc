/// Requires `Client` to be in scope and to implement `unloadwallet`.
#[macro_export]
macro_rules! impl_test_v22__unloadwallet {
    () => {
        #[test]
        fn unload_wallet() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let wallet = format!("wallet-{}", rand::random::<u32>()).to_string();
            bitcoind.client.create_wallet(&wallet).expect("failed to create wallet");
            let _ = bitcoind.client.unload_wallet(&wallet).expect("unloadwallet <random-wallet>");
        }
    };
}
