#![cfg(test)]
#![cfg(not(feature = "rawfmt"))]

use artimonist::{Xpriv, BIP49};
use bitcoin::hex::FromHex;

#[test]
fn bip49_derive() {
    use test_data::*;
    let seed = Vec::from_hex(SEED_HEX).expect("seed");
    let master = Xpriv::new_master(artimonist::NETWORK, &seed).expect("master");

    for i in 0..ACCOUNT_XPRIVS.len() {
        let (xpub, xpriv) = master.bip49_account(i as u32).expect("account");
        assert_eq!(xpub, ACCOUNT_XPUBS[i]);
        assert_eq!(xpriv, ACCOUNT_XPRIVS[i]);
    }
    for i in 0..WALLETS.len() {
        let (address, wif) = master.bip49_wallet_harden(0, i as u32).expect("wallet");
        assert_eq!(address, WALLETS[i].split_whitespace().next().unwrap());
        assert_eq!(wif, WALLETS[i].split_whitespace().last().unwrap());
    }
}

/// # References
///   <https://iancoleman.io/bip39/>
#[cfg(not(feature = "testnet"))]
mod test_data {
    pub const SEED_HEX: &str = "5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc19a5ac40b389cd370d086206dec8aa6c43daea6690f20ad3d8d48b2d2ce9e38e4";
    // pub const MASTER_KEY: &str = "yprvABrGsX5C9jantZVwdwcQhDXkqsu4RoSAZKBwPnLA3uyeVM3C3fvTuqzru4fovMSLqYSqALGe9MBqCf7Pg7Y7CTsjoNnLYg6HxR2Xo44NX7E";
    pub const ACCOUNT_XPRIVS: [&str; 5] = [
        "yprvAHwhK6RbpuS3dgCYHM5jc2ZvEKd7Bi61u9FVhYMpgMSuZS613T1xxQeKTffhrHY79hZ5PsskBjcc6C2V7DrnsMsNaGDaWev3GLRQRgV7hxF",
        "yprvAHwhK6RbpuS3fpLb97uQpag9G8qSNUDDhkjFykzVHDyGKM7whzqnWWkdVHbtRce6L1oSEAoHcakGozAMBaeV56o4u9FzoR5SdbsjienYCZK",
        "yprvAHwhK6RbpuS3iPyv1P1qY6aQ1uW27wZJE2GvSfCBvpgeKjD5WykvC3wxb5bQ16d42k8VHUUkURGmFdSWbsG2FJAJKd7poH1QwGpuZQyJq8c",
        "yprvAHwhK6RbpuS3n2ZPK6kdocSjKuLbBQe7Mo7YUpoHDMAbTjVsGE5ejauoBXCK8CeExhHfB7F5EGFathDfnvrsKyTny9sVnrPKUsBhTVgSsXR",
        "yprvAHwhK6RbpuS3pjYxA1ZLzuud3TLWqrMaffNAbM1pqP5Mp5nLmidBiseNngiS3azPR8DyHLdiKAAvYu7KR7UnGqc452tEPnvVg8DLGZaFqBL",
    ];
    pub const ACCOUNT_XPUBS: [&str; 5] = [
        "ypub6Ww3ibxVfGzLrAH1PNcjyAWenMTbbAosGNB6VvmSEgytSER9azLDWCxoJwW7Ke7icmizBMXrzBx9979FfaHxHcrArf3zbeJJJUZPf663zsP",
        "ypub6Ww3ibxVfGzLtJR4F9SRBicspAfvmvw54yern9Q6qZWFC9T6FYA34K57La5Sgs8pXuyvpDfEHX5KNZRiZRukUWaVPyL4NxA69sEAqdoV8ve",
        "ypub6Ww3ibxVfGzLvt4P7QYquEX8ZwLWXQH9bFCXF3boVADdCXYE4X5AjrGSSM7SYF14twFWFkePzPykRkjkryxrMSVRLFkSSd7FpK2quWqDnXF",
        "ypub6Ww3ibxVfGzLzWdrR8HeAkPTswB5asMxj239HDCtmghaLXq1omPuHPEH2ocbVY8gF2HStshDmZ4ShGUCSiCsMtL8LFg4PNFd1X2fNsUpPji",
        "ypub6Ww3ibxVfGzM3DdRG36MN3rMbVB1FK5S2tHmPjRSPicLgt7VKFwSGfxrdxFtXkWJPK1jLQcZjovEGhUWcgHij9AESLeRVvaUCJnKpz5ELca",
    ];
    pub const WALLETS: [&str; 20] = [
        "3FkmL1FpK3nnZHrt8JxRr9gNB2yTTDYbm4		L56c34m1hg1hr6cXjWfrzFA3W55bJicisptfJbA8gicrP6zDHCFs",
        "3FL4MomMCjwBFFdh8MFN8k6QFkk5y9D2JN		KwocwFfQpwNnCiCgkRVkZdojQfge12ZTCpu8267MJzAJtgXj7TFa",
        "37XCCVJWNJxHrCxMEY6GQX7MDxPon4JQHk		L3QgFTi8dnpbCUyCpmoGnmtTU6MsAeqQxK4711bNDzBmcw5rKBNb",
        "3Cww5zwuLHw9GMZZXABQ95vyKdsT3gGeUU		L2y2qjhyno681CsnHgo6W4skLrjVNoypJ5DVEM8LYqYkVqHpKiZx",
        "36xjJrWqC2doui9P8r1ZvwFfT6Bw316te4		L3fvDNoLbjGGAdd21AXKpr95HWKf5NWXpKGfuQg3iNeLDs5mSave",
        "32RduZYi5HX1MfhQaYAMosNmRRmACHKC4o		L52o1iNi4YakHrKWHBXNAKkVmUwEXFAFwHZosu69TMZv8W3Ri5Np",
        "33CFKP9sYxd1SuaxuH5XZqtaZYWprunUSU		KxJHNK1Yr4NRXJyyXscXBQhvRRRRVhfbD92UFFF1J17Qcf7UQfNs",
        "39kgVgBjrmA72roPjerEFTBd3kr5CwNdF4		L47C8mhNZ2vhwDHGRC377Kh98sVXXjDkJD2aWGR1xP7iyf9CtciT",
        "3CcCfXDhCV4wo5CpvUWJdexic8VUWAE83n		L4kAQUP25VeTM6zycLmugcwZgmrGEhFHCyTakMjHDdTDS5bz8NMX",
        "3KhMoooPhuVDe3Yyzf5QaVjaJ6utJdM9Ua		L3o14H7rLfRAFu9DmvnrU5GxwfUUQfJzyMK3DotjGaK7oCGsdd1o",
        "3BPuYS6wBHpwH3WXr1F5mBSXC7JfVHfU2o		KxGozRk6XMv3qigmYGc8T2TLLhnxbgWx27kbQyEtzfKxZLzuVxve",
        "38RR2ALS6Cvf2UXe6JaJvDAXhyNSgmiALR		KxqZmnCpAAjXypRTKuma4qZToDDJG9Hhwzz2A93T4BcpcCAwnzfs",
        "3Mo9eBb2fUAEp3QYr2u3HpKnqf8VEwaesS		KxaGBN7PeCR3p36axbbwtUt9Upu79RsXLXWUY5eBog75CWPn8djL",
        "37zpvau9PNMBjE7H15fpAe7D26RmTq8Rb4		KybNZ9RQ8KT6bs1eMnScMbvW79DmTyCgDgvs4FQDbqmyCwMUTozK",
        "3L6LD896NEDMkP98e2DwLUibxMhPzzFVxc		L2KZR4Yh4HsnS6QQVkemTpiKAwhkDZf4mN3EPRaUp54TyjFqjEjF",
        "3QeK1AGaGDMRrFbWojR8fXApNmmMWTuAq8		L4npoKb4nw5xPH55ojCvxZ3i1oKN9D3VVPF6B4KwZbK5LE3eFijU",
        "32isByi3x7ecd63TqBpnbz6u5AagVFv6fP		KyHXkW4izvpC5V4xxv4i7zZ8L8WJFBK63SDXefYvcWc3R7pC6bro",
        "3Lisq7onC81zYBgVXw3VqciAoS3mUhPjks		KwHgG6qntCD1ctFpbgJirYAc3dT1NDTxRpVWPDqTMCG8ddjCWM84",
        "39bNYXFkWNoY1hujLoYjEMtWBRGheBxCCz		KxxrxnsxDVN7i1tTBn9sdwVWC8WA5jbkzPMg33KqqX6dBoQEL7Lp",
        "3JXybcGd7S71QCLRde2wcqnaAyGVWiakTa		L1FvkRq2CfDYPLRmAmZ3kCAD5s9jEsj9kjH3hDcW3jTVNDVQVh52",
    ];
}

/// # References
///   <https://iancoleman.io/bip39/>
#[cfg(feature = "testnet")]
mod test_data {
    pub const SEED_HEX: &str = "5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc19a5ac40b389cd370d086206dec8aa6c43daea6690f20ad3d8d48b2d2ce9e38e4";
    // pub const MASTER_KEY: &str = "uprv8tXDerPXZ1QsVNjUJWTurs9kA1KGfKUAts74GCkcXtU8GwnH33GDRbNJpEqTvipfCyycARtQJhmdfWf8oKt41X9LL1zeD2pLsWmxEk3VAwd";
    pub const ACCOUNT_XPRIVS: [&str; 5] = [
        "uprv91G7gZkzehuMVxDJTYE6tLivdF8e4rvzSu1LFfKw3b2Qx1Aj8vpoFnHdfUZ3hmi9jsvPifmZ24RTN2KhwB8BfMLTVqaBReibyaFFcTP1s9n",
        "uprv91G7gZkzehuMZgnwfF8yRPJGAFFKFGCzCtEe7Xx6GjHXFFuoGb28rCusKPkE3r1MJBcbgRDmAhdy4e6uuzSDf5wG5kEzaBb72vttGxdeBJj",
        "uprv91G7gZkzehuMc5ffY8unB578wBng41DFdGqTpPBHcdP5GBT5zsBxxU7XwvabmbgYf17xTJ5wmAHpfkN7mvCzDsy1x8qVe5TxTGuGPD3SihR",
        "uprv91G7gZkzehuMdmeJTZUvhLQEoQdPFexBi5tXv6bWNDA5WA8fL5HRSVPh62QVC9UGUxEnLWGk9ZdSunEQD28cL6n3F34qyNUk7XHfAPFPSX8",
        "uprv91G7gZkzehuMfpMddtV9h2tb6z9DgCTdajJHZV92qeT1FpGfRVUecuPtKy1Bc1pvdy1hFcF6c81WMyzcH6bbyvCgkqrXfWKYwUevH7fQoZZ",
    ];
    pub const ACCOUNT_XPUBS: [&str; 5] = [
        "upub5EFU65HtV5TeiSHmZZm7FUffBGy8UKeqp7vw43jYbvZPpoVsgU93oac7Wk3u6moKegAEWtGNF8DehrnHtv21XXEMYRUocHqguyjknFHYfgY",
        "upub5EFU65HtV5TenAsQmGfynXEziH5oeivqa7AEuvMhq4pW84Ewp8LPQ1EMAh5ixzyv6dqtgaQAoeYv17Uxdr21DKfzyp4nYVFQcwRdpNRFGRm",
        "upub5EFU65HtV5TepZk8eASnYD3sVDdATTw6zVm4cmauAxv48ynEYQWDWGS1oDdJ9yHUHZoQupYVewuNMsFd5ADVuRW4xSRx4NRozNgjd9jVKbd",
        "upub5EFU65HtV5TerFimZb1w4ULyMSTsf7g35Jp8iV17vYh4NxToscbfzHiAwH9YPEU4EEthRJ4DPm1mSW2ckij8iTGdtRPUajvBAFHE2HPiVgB",
        "upub5EFU65HtV5TetJS6jv2A4AqKf1yi5fBUwxDtMsYePyyz8cboy2nuAhiNBEhfGj51oMYeAFjibLauwvguepXHfLGS967oiUsQhRACzN2zGWe",
    ];
    pub const WALLETS: [&str; 20] = [
        "2N3YPQePQzykUGQ9BzHyzn7ESXfCWsR9pir		cP2AuPom4hMzYW8LrFinCvhvER5KH6BLCXuggMrb3tokshaDnApK",
        "2N5vSCeaajMGSa6nCnZPJAZfWgnubLoM1w8		cUFERUKbpBD1zJVjGzkxUBuZVqM84AnoMN2WrFSNYTyDrwE5rcTY",
        "2N8pdnigLLsT6i5vCMrPDnUNLsiS1szBQ7V		cPXBq6y2BrWELFHTtDZHzJtycPm6Y3Hdb2JMwMYdX8RBVRgGJJ2n",
        "2N43tfksLQMkjD4k4jVS1m88vK68yYhJBoU		cTuv4bpLVKSwfFM68RRbU93cpVqPJdPuYnDcD8wpV86Ke51by3GA",
        "2N2EFcvBxd5EqMPYnGaGFqcTGLx22QGe9ga		cPNQ5K7XKttR6WqwReQcGWD1Hp1rGZi8JDqZV3qvkLiaJ1LDH7eP",
        "2N1rebHDGjvFyZ1LVSeppgjuUKFFMYycEGf		cPuUX3cBtcjehuYPKp23Qy53m79j5Sca9Q7uuzAsheVbmB12RyFv",
        "2NA3iv3kknk7USzbjmuY7sPEi5Y3BDVoW3q		cTyHUQPEQdYtNMMyVHJ6sAggwWfoVpjS5cuhiJhB9SLD2nSQuLzZ",
        "2MyRipSKveEVmLbWGQxPh8XY4qQBJfqCbDv		cQLQZ1k42SqKExAsx4CYNEVvPRcDbdWi9x2QiDRTgiMVDK8vxN9C",
        "2N69jXMiRVREujSwEsU4fZqWJ611W2iUCDL		cQw5bxKr3KwiCzNFJFf7HqakQkVsoquCdAN8sqGUYgJvXAUH347w",
        "2NCV2wXK3w2vwwkNYQPD5TeRbC1PyVMiw2y		cUz6MtVh16yhihnc2Tt2yiNzDcxFAzNqFwPqtUZtHRs3y2V7YLPp",
        "2MyNe5ZrV7JPKf31a5m61ZiSAzdP52o5NtU		cTEWkQsJEWxGJX3Kk7pC1LqczUpKYA4p1N8mBYvD3B2kvRX1NDCq",
        "2Myw5e1fY1niio5bWveheJyRhuyVaaKLaZx		cR6KxDx5D3KMgLC4EkQT5AwYxGH2QG5G4pSfc8GEj9B2pTFdbUDC",
        "2N3qgzg6pKA6f3YdAFhNBgE4QLgC65r9Z1b		cP3jqmohQNcEeFcCw52sKrh6pwDTCeuEmMzsVRNhzGBeU61sGbea",
        "2MwaeYUrD7opkDutgqzAn1eF7Er5hWLaBMy		cVD6PotmNUoSMyhSo13mhvneqq8ZbXmJudxxZjPYinCVBFA5G8qA",
        "2N53kGdYqqjHqSgaim7dLAJdMa6Z63NC5nw		cU46b3yRVf38hwRriLd7F3d7gJ6YCDUYqXsykQJ45bnu2tfB3iJr",
        "2NDJfb5tz7VJQJPw73vT2SG2NMb5g4Knks7		cRx4k7qSgWe8NtKSEvxQ51YZYbiiw1wcptkaCpN6N1UA5rmr4U9s",
        "2NEQCsQ4cuUVBTMdoS9vQ2FuYVYvbw4uKUZ		cTL28cyop86JA1y2akENGaYXHRAo4yZooo9m1aBH3uxDB9wiqLya",
        "2NEiYyxMp8XWG5GACPXAWKEvGXbMpGFkeuK		cVasHDmS3G4kPXhm3MS2Peg5tH6jj8gZ9NG9P5LZbTXRy394WiRu",
        "2N1n7xorsBkBCCccEyMCLUiWFaz1Gyv7DHs		cR6SwLza8RL2UXDEE8AQjxQzkdnKtBd1rcVbtVW5SaDQ5HNfkSRK",
        "2MxxSUWs3qFR381Q9Rxm76eoiVJCGwnnKyT		cUbt21iGFyEQSFJk5g1HjXjCaXYooEab8en26seHWDPr64qebRLN",
    ];
}
