#![cfg(test)]

use artimonist::{Xpriv, BIP32};
use bitcoin::hex::FromHex;

#[cfg(not(feature = "testnet"))]
#[test]
fn bip32_derive() {
    use test_data::*;
    let seed = Vec::from_hex(SEED_HEX).expect("seed");
    let master = Xpriv::new_master(artimonist::NETWORK, &seed).expect("master");
    assert_eq!(master.to_string(), MASTER_KEY);

    (0..WALLETS.len()).for_each(|i| {
        let (address, wif) = master.bip32_wallet(format!("m/0/{i}")).expect("wallet");
        assert_eq!(address, WALLETS[i].split_whitespace().next().unwrap());
        assert_eq!(wif, WALLETS[i].split_whitespace().last().unwrap());
    });
}

/// # References
///  <https://iancoleman.io/bip39/>
#[cfg(not(feature = "testnet"))]
mod test_data {
    pub const SEED_HEX: &str = "5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc19a5ac40b389cd370d086206dec8aa6c43daea6690f20ad3d8d48b2d2ce9e38e4";
    pub const MASTER_KEY: &str = "xprv9s21ZrQH143K3GJpoapnV8SFfukcVBSfeCficPSGfubmSFDxo1kuHnLisriDvSnRRuL2Qrg5ggqHKNVpxR86QEC8w35uxmGoggxtQTPvfUu";
    pub const WALLETS: [&str; 20] = [
        "13iX7DteNj1gV7zhe4t6o9FX9CArR5wZxz   KxDTisYSXy8fwZBtxiXKNRCkipFviJLosSceTf7BXmC79xAqzDBW",
        "1QAUQ4opPaGfnow7qBMVcmMhYg9Ubv33x9   KzbaM51UVBognvXyuMMPTgrWnEZxdwTgZDqvmbf8SyqNyetVWGqm",
        "1Ad1AjfGdt9cGhnV2gCkx4TN78kPz3cF8m   L5QG1cxQgawk7Xf8NSbwJFPbzz13uFuvMT9uGsxKgZuwHYsGC2iP",
        "1KU2f75ZSi5JfhtWaUNvoCCaVLLu9kYQ1H   KxiAyNsQKSZpTZUiQA81v59bEav7vj7odnvRVicdW8FvtGRTxZNn",
        "1NP846dvKho7ufQvPrN3tAHhHfJadhXsEj   L37CpBJGXFfzkrmuSje8cDdrdE6pUztb2SnT5wCyrpTe9FspWoSc",
        "1KxZ9kfh8rJCjH1oLhuTw5btboa4E1w44w   KyRQRecjqZMcpzKaL2Zjy3xprjLQ9WcFpR4rM41SoLZhdEtHaqm1",
        "18cis4G3YR2TnDb3k6tqMYyG6o9ber1xdB   L3ebhJn9jxTydGddFE9PoojKJCtpoUtaQ5PU8P1tQQzzbhKLBGZj",
        "1LQv82yMGF5adtg8Ejyq2kAYjp2YwrRP5Q   L1WJZ2BXLX1Hg7rwMdrGxTek3E9pWJB4q6GLoFAumU3r9b1HB31Z",
        "1DhurUvHbUnLUkSo7mDWSmR9sHCFUCfXgh   KwmRubC8EfzzZANM2QdBK4dy94KJmyZWnwGk2xybe23cssQnqEiV",
        "1A3fGE1Y6ZxPttzJQHYrJWigSk5G8xXjh9   KzD6v4cVtULiUWXMYHguXUKkq7TDYHt4vvuaE4HtxhUXdmami14H",
        "1CXACUeLkn1UqPFxJCLWWNrzdyYqmigD8j   KxQQpE5d6eRL1RNPd7Uy8ewdLBwkTcw61Lm9fXUdTzw2YfsFxNJP",
        "13hXik6BvyhnGhoNb89VhRsadShEUFYhbs   L3VwuqtfPsdZemiaG2aMvF4MsaqEcrKEqnGFTw2GREksBg8M6nCt",
        "1J4egXu5rVtRWECQQ4i591pnQvKZeAzGds   KyBYWXEADUeQwUkCTMJywk7BkfgvjHTxpKBGk1UebLJpWn3D3Lgz",
        "1iTSCvQD8AtjpNNmHfrDu2B8F7APgFwzF    L4JYu3WE33uQJhdmS8k6ZF7DRCiFTTu2f5Hif5K9szsbToXTZvCv",
        "1HGzL6sa5gbmfguyMqNHfkqwh3JBNN9kE7   L1VCffL53pZcka3WYig9SrFfe2xWsLMyyreQp8mdVYu3JRtTdjQQ",
        "1Khz4rGgAFg7xtcFspmBpBExo44BPWF9nw   L2oh98Qxy7FYjTtFLtxgSsbFpgyqSxKaLLFfnJtQfns5Liz99D27",
        "1M9fFGtcSLT7bCqCn2vt1bR4WeGZBevRuk   KxoVKEvwhTKVqVqGWgXsBAthjBhwBKXeBmaRcBiW4cjceTTumCra",
        "18ZGvrPmd3eWuHU6PyBEUZzU1XKRro7cK2   Kz7iYdjEwjFjqgr3fWvSbV324sQznYVFEVa9uDupFdMWQ1PzYY1M",
        "1AJzgcH86TM3seLx36KfXHAyZ63JDDmrTW   L2HnL642pPXJfQcBWR6zfiu21xkperTkbhRjPWyU7hpky2RQa8UC",
        "1CBSsCNBKMrtKU6HaweqAByUwjWDBbfVQX   KwvdaQnh4XT5AFJb73YwbtPHLhmPUAh2biRb1Y16iTJazbmYUFkr",
    ];
}
