#![cfg(test)]
#![cfg(feature = "extfmt")]

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
        let (address, wif) = master.bip49_wallet(0, i as u32, false).expect("wallet");
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
        "37VucYSaXLCAsxYyAPfbSi9eh4iEcbShgf   KyvHbRLNXfXaHuZb3QRaeqA5wovkjg4RuUpFGCxdH5UWc1Foih9o",
        "3LtMnn87fqUeHBUG414p9CWwnoV6E2pNKS   KyaMvgopkPDQMQUx2w9a8AiEtA7A84hYzASJWGQiKZ8AJUEj77iV",
        "3B4cvWGR8X6Xs8nvTxVUoMJV77E4f7oaia   L2Z5PN4YPPyGFRCQzxrqa4ChdhFqiba61gQUmCsCW7GaNW452hbM",
        "38CahkVftQneLonbWtfWxiiaT2fdnzsEAN   KwSLFskNqE5xezGjff4N7d66WcT4Nco8YHLZSHwXvJPZnd1MYXfr",
        "37mbeJptxfQC6SNNLJ9a8efCY4BwBh5Kak   L1V4R4JUUfu9aCbDeh3Po8qr56Q1M6cMTdF3pmK3rc3Q75QTtaxg",
        "3QrMAP4ZG3a7Y1qFF5A4sY8MeSUxZ8Yxjy   Ky24PapSDRELSwQWNkYKFBTZgkknDEMnpZhS6wDEek5D3YC54oJC",
        "3NzFBzVHKEVAnYKWXjZKJ3H4n4pUuq2sfg   L2ELb1PV2m3TyG8GArTmTfVSxsWXnWf9btj5WbvWwNjReED25kzw",
        "3KHhcgwPgYF9hE77zaKy2G36dpkcNtvQ33   L3TtfR6bZY9AEnHxfXpuUnwXUJ3dD3uamHByeYGefJGjrGXk7nCj",
        "3LwcWnqXb6f371qkWZRxW9Hbe798zLmpAS   L5hEdSwMkQ2ek5BPVUfXHoU8jojR3gAWFuwj3MMLkXpQ96AxsAKk",
        "3HFZKZgRfzcEbu7ggo4BD9opSrjLAJWVWv   L29cVVezngHc2FwhW4VEFFUeY415iXkzmednDNWCEkpvKbA1xFRj",
        "38mWd5D48ShYPJMZngtmxPQVYhQR5DGgfF   L3frCxVqz5XqgMFTjxSAHot8sFL5MvtcHEeCZrVGqqLaaNJbfzpB",
        "34HSx9QGfkGHupAdtRBpBNTiFHxEXpscdj   KzSC2AYtip4EfSmxzcHttPEwcTe6JzstJRANMBgLs3g1zawYwhbn",
        "3HB1WEujyUJicjgKV4RiBMNRLoWYmLDr1s   Kzt6ttn8GExj1yWgdP3SzsfMWVSXUeGRCfb6pkGRSxrGEux2U3c1",
        "35DhkaiFrp3oCBzxUnmut8nCbDnbykQMbC   Kz3Sx2KnomkJbcqPUJLdDeS3wcyTJeKefSq1nDHnPTtBRonkZwZp",
        "3Drg3sRxhxEDtNi66pmEorZqRaWDgDLnHL   Kx4Q14Y9erGb1bwMZ7qRCAuGnPwbQn7c2R4Du6AChMCDx9bCpExs",
        "3MLaBHZRQBz6h2ADe6DfChSaZmfMYWBfJP   Ky3Q846zzNtaLs4AeyLqS5iKWBSZFCxocDAFUMtkT8jKC9ALuwEH",
        "3NmSLfUSMB3zstyMRMzfFmkPXMufrhsuAc   L2c2m7aAfLSTscoQMWhJpKHbREfsa6naoaxXoDqZqYPhdEyfcEyj",
        "327kJyGsgTixfKdSvK5JqfXbXQQrczhLE9   L3jxuySXSdeMuMy4ZFqJEgCcFbg4gRAZMoX5i7jDJFBxpMhu6Fe8",
        "3EJ3YM6ELZ7f2GsARv7AMpJvHoeTqa9V93   L1Qd9gTYaafFxccjdFRHYQPrq2gxtPzF3MUMNUFnt9jxNwmFUqbP",
        "3FsmoJ9P2eUrKjd2ooa8UJVAeyMVPNkvp2   L4cCLnrvuLCTpjUaztrPxvMiznDh8oRGKakyCAw4xdRPEm9odY4A",
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
        "2Mww8dCYPUpKHofjgcXcBCEGmniw9CoaiD2    cULrpoZGXiuC19Uhvykx7NugygA3k86b3hmdCeyvHYQZSxojGyXJ",
        "2N55m54k8vr95ggehfUcNkdbUuQvaqG2GxK    cUWvs4gEqYVpQ2Kn4aAt6S1Kn2VKvCEFmnmL7XAvrco4frNWD8iJ",
        "2N9LKph9TKtv1WLDfaUJp4D8EKwsyASYnGX    cPxPzgxKn4tE9vTQU3Sq3p7ypG1tTkdirQHjcVA5NqqwCx1vgQfT",
        "2MyVXDzGJgATSdkhKHWvStpBoGEZb1fwjha    cSzfwxWXgcYfYYAPWAxLpa5CcuHBki8d8QQ117S9fckQtvDimmqC",
        "2MuKeQzUHhUQWUZgx5AuNWoQ7YWx6vsXxrv    cU4D6GEByRZLoihqp665rSCdGR41gDsk23FvxM5XdQu4dgeS5F3Q",
        "2N5pTWRLrRdAPGvTd9agPFLFZvPfGNy7xuM    cSE3d8XrFAT44qxqSkczDUgpa5zXFPcufVDh7ty3RSBfvuFSeL9f",
        "2NGH8tu2EGN2EuDdtRhmFFtWDu817ad4ikz    cNGsFhuCDgQpFZHyatEe5kbEgw2FLmyGvSJddkdKqiYCStCbr5nb",
        "2MsdaNvf3b8iGC9rC3ETr5nUvbRPeAMXNWM    cSu3YAmVw7XhyP4aMfXB3K7PiH1ZLy4Zwg9zfPpw865WvZEmapnh",
        "2N1FpQEsArj3hGBdwc6n7e2CzggtRAooJTP    cP2wJy9tPLBJLA6xCr8V7L9mRZcmzGNWoMaVYoLieJJDeBVbcnru",
        "2Mtrpqq7cQznHw9wYnsSKroTdZ6u3fsB4kZ    cTmegCDqDTD5g1qcjVyKgoQpxVMmrbDL5zsPhWoNmqAwQbt3TM5w",
        "2N4aDXmzJykaaihDFws574RyM2aFJpT4EzS    cRaBPnVZeuPLPjSvgd242ogSAVAFjpYzU9S12FMVj3aLHu3Tcq92",
        "2MzGcncD84os4e8yrexdvDK685k2LVg6A9a    cSk1snxDvYKGLqK7KjTEHnAFpYD2AYUhiNwnUYWckkm2s4avAvDR",
        "2N1yy8dDstG2cf3Xt6WzkJZPGV63QpZBFZu    cTyDvdtZQj64gSjg9RpP1tTKGUVHbts7WBkSe7FQzVQZN6NrowMs",
        "2N7AZigbQQje53NybX2AuD9amuF3Kfr5Z8h    cPProvhxCDQbw3LuuAENQZCB4rGqnXrJCAXo7hjsWZXhSjcQ3FXx",
        "2N7Mg3XFUHN9H6kkLjVzxzqB29SCzygjaZX    cQgrkLqqRKpKW15yonEEHGaH7RNVWbY1oHKJWUBP7a7nbTrrUrQ2",
        "2NBRjDXAbHXNpMpo7uKwKyF5hyU8BkzpsM1    cVXm23CU6gP5yTRgapivr9GUeTXMGuvj2x6rJgfgW3zq7fZrpSqi",
        "2MtsJMgXqk95CYzUMvchm2nfPJeKrVwwMSU    cNuZ87Wuq9Ag9eKxrUyPyLfvo13XkSE9L7ZZnS1K3t5UZyxJvUS7",
        "2N9q6uC7DhHGSwhEor7Z7JvniLc3T6dYpVc    cPoouahZX6v44QiSmVcDiYCRrApp4ynmGKPARWiRq3m8ALMs8j8g",
        "2N31gWk3ZQygoANPPn39qC9ZQTc9jhqicsE    cSsjz4Dq5YtsyxvFi9XJaUsx84y4mUs9SNJNj1p3oB55qd8gSC9a",
        "2MtpC7u3H88WHwMDRsDitc5YnAv12ZWGQzm    cT36bGHzShcUX5wGqyvCVPHC9Kt5HrDN3nNwgJy2BUmzoTG1m1qC",
];
}
