#![cfg(test)]

use artimonist::{Xpriv, BIP44};
use bitcoin::hex::FromHex;

#[test]
fn bip44_derive() {
    use test_data::*;
    let seed = Vec::from_hex(SEED_HEX).expect("seed");
    let master = Xpriv::new_master(artimonist::NETWORK, &seed).expect("master");
    assert_eq!(master.to_string(), MASTER_KEY);
    (0..ACCOUNT_XPRIVS.len()).for_each(|i| {
        let (xpub, xpriv) = master.bip44_account(i as u32).expect("account");
        assert_eq!(xpub, ACCOUNT_XPUBS[i]);
        assert_eq!(xpriv, ACCOUNT_XPRIVS[i]);
    });
    (0..WALLETS.len()).for_each(|i| {
        let (address, wif) = master.bip44_wallet(0, i as u32, false).expect("wallet");
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
    pub const ACCOUNT_XPRIVS: [&str; 5] = [
        "xprv9xpXFhFpqdQK3TmytPBqXtGSwS3DLjojFhTGht8gwAAii8py5X6pxeBnQ6ehJiyJ6nDjWGJfZ95WxByFXVkDxHXrqu53WCRGypk2ttuqncb",
        "xprv9xpXFhFpqdQK5owUStFsuAiWUxYpLkvQn1QmVDumBKTvmmjkNEZgpMYoAaAftt3JVeDhRkvyLvrKathDToUMdz2FqRF7JNavF7uboJWArrw",
        "xprv9xpXFhFpqdQK7yf7Y6EWVHpBJk4xWSVYU83PcdP7ezFV4ynXY3eLJindWrtVN3cSnUiSU5wmsAYUyqptgbyRya9pbamsLEN5EMWcKs8eaY9",
        "xprv9xpXFhFpqdQKCeHi4FWqUyLTYuYWe6eKxFg7gvntBhXUy9JgVp9w7H3dieFGzsRXDpEG6Ci4yc4ubnhNUB1iy1wBVjeBhzroAmwK5Gb2J7X",
        "xprv9xpXFhFpqdQKFQ5HTBK8nBw1RqbvHHGzg65nokBtjqLxw5nFg3xrZS4FebcsbdgB3tXCArh2nrQibEnAuE9sKtz7adUJLrXgoUnTZRo8bhf",
    ];
    pub const ACCOUNT_XPUBS: [&str; 5] = [
        "xpub6BosfCnifzxcFwrSzQiqu2DBVTshkCXacvNsWGYJVVhhawA7d4R5WSWGFNbi8Aw6ZRc1brxMyWMzG3DSSSSoekkudhUd9yLb6qx39T9nMdj",
        "xpub6BosfCnifzxcJJ1wYuntGJfF2zPJkDeG9ELNHcKNjezuea4tumswN9sH1psMdSVqCMoJC21Bv8usSeqSP4Sp1tLzW7aY59fGn9GCYzx5UTo",
        "xpub6BosfCnifzxcLTjae7mWrRkurmuSuuDPqLxzR1njDKnTwn7g5axarX77NB5STUkWYSzaLN77HBYD6hpvLkHeUJuPTBvTnA6jhTUHyNrRg3j",
        "xpub6BosfCnifzxcR8NBAH3qr7HC6wP13ZNBKUbiVKCVk34Tqwdq3MUBf5N7ZuQJ1WVtDWiBoqwYSQfbj4KfwBNjDHkEJXeHcTXJsHi7euVzyad",
        "xpub6BosfCnifzxcTt9kZCr99KsjysSQgjzr3K1Pc8bWJAswot7QDbH77ENjVrPmJp1esxpFCDDNp6VjLabQmCXYgWwawju9mbWHqUvJYyvs8VF",
    ];
    pub const WALLETS: [&str; 20] = [
        "1LqBGSKuX5yYUonjxT5qGfpUsXKYYWeabA   L4p2b9VAf8k5aUahF1JCJUzZkgNEAqLfq8DDdQiyAprQAKSbu8hf",
        "1Ak8PffB2meyfYnbXZR9EGfLfFZVpzJvQP   KzJgGiEeGUVWmPR97pVWDnCVraZvM2fnrCVrg2irV4353HciE6Un",
        "1MNF5RSaabFwcbtJirJwKnDytsXXEsVsNb   L4BL9ZGzuQJFoRqGfjsgHeYzD1C72y2VmJaY6sqdtaRkfxUFrJXu",
        "1MVGa13XFvvpKGZdX389iU8b3qwtmAyrsJ   Kzj5uojwkWiBXY5TBxuYZYuDhYnWnHh9rjBz2j8j2kpBXYEoT4Kk",
        "1Gka4JdwhLxRwXaC6oLNH4YuEogeeSwqW7   KyDaMAANJW6LfNvATYzYnAaoE5EUaHUZ2pyUziQSeBumkDkKNcpC",
        "19a7HGg32ecPQo49rDeM2NSFJHPqrwSJto   L1mjZzjojywhnWEgYPipgWg6dxGrn72xnh4ibtFErcg3komjLFrm",
        "1GuMEkKyqqRz3jKZJPNxZNoJv72rRDm88o   KxESzjCpjwhSXE5XzkfQhNMV458Li9CmdSGhGwUvLKNbvsmNXfoY",
        "1B1wDxGPrfqWSi4qvQvaPdunD6kon3CeDG   L3iGmVfjgAMg3M3CFny16bqXnLFZpkE2NS9UKbug85N15TpL6kkL",
        "1BMZTqDtNogSEs1oZoGxRqfR6jS2tVxvHX   L39RT86hzT5hrvvr4rA14bnFTMp2v1CfmJ5Vbej7wfWQYh3QaBAT",
        "1DUrqK4hj6vNNUTWXADpbqyjVWUYFD7xTZ   Kzz9UuyFETFDQJQwYpmbeczidappScVue57VxqAU2EN5aUjxRZxg",
        "146emAmGumhnsT9nPCALU2JWeS4koxfFRB   KwwxhiBFKL1ecPqTBbaUtTMerg2ZX1TChc4GeEJGTieBFtB1Zc4j",
        "1Jvsixwb9y4WTn9zEbhb7ykhcwZ5LAjBFy   L5S8BfWfMsdXgYLAzn7DEetP3jMMTCpTCQ8vCT2A4WHX2j9LzPCt",
        "144gUAoVG1aRpYxQMySn3zZtPVYPiJuYYq   Kx8mBuqbQ4wZHgC6jRB2Su4gUqT15boipGjFSdcKjRmv2YPwuCQW",
        "1Bsv7mspy7do7hL6gNRhurusW5yiYqmnsH   KzNFMLZiPX266dqGeLh63YY7n5gceJgAaAN8Khgo1Dm37iqY2jML",
        "1JLvy1tbH1ztV3txZYJfdzke9gmtyRYJw9   KzQbEg5KETc2GaRzButyYbrwpPHjRJfu3s6CNVQoKpHDdnaJQ18J",
        "1NtocLbFFPYPNGeEsDn2CYY4GbfLGLpTFr   KzVYdFFZ3ecN49uzK4a8UvueovxhcTm5imGrQ2ptfD19wDMZksDV",
        "1LQxYTxaKkRDVmViwvD9X1jQpwr2Mb19xg   L3RerJ8aHPgkSM42tNkpHBn365ebgXnfpqZmok53s4dsdxSufn8C",
        "1NbpX8tyfTEuyyme5gSjWvgpZRMTtyrazM   L2tbg7qK7QnPHwCn8L5cQMKgLugApeSSREKUSh8i36wkjH2u957w",
        "18xPZpJUxJhuhTyysVEwJAAoYUwX5T3cFV   L1qjPVcdxj1zMhUKsE73w7d2Xz8siSufptQ9PYXB1CBVprvgciwK",
        "19hp5PzFjsD6z1hwMucUbLHAYeYDWdvB1B   KwNJGRsrZXfzLJRYy8tJecXZgFkPMPF3ERBBFnz2qW8xhsymMH9f",
    ];
}

/// # References
///  <https://iancoleman.io/bip39/>
#[cfg(feature = "testnet")]
mod test_data {
    pub const SEED_HEX: &str = "5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc19a5ac40b389cd370d086206dec8aa6c43daea6690f20ad3d8d48b2d2ce9e38e4";
    pub const MASTER_KEY: &str = "tprv8ZgxMBicQKsPe5YMU9gHen4Ez3ApihUfykaqUorj9t6FDqy3nP6eoXiAo2ssvpAjoLroQxHqr3R5nE3a5dU3DHTjTgJDd7zrbniJr6nrCzd";
    pub const ACCOUNT_XPRIVS: [&str; 5] = [
        "tprv8fPDJN9UQqg6pFsQsrVxTwHZmXLvHpfGGcsCA9rtnatUgVtBKxhtFeqiyaYKSWydunKpjhvgJf6PwTwgirwuCbFq8YKgpQiaVJf3JCrNmkR",
        "tprv8fPDJN9UQqg6sLZ6AvtfGBeWfsBrEW77ADxq2zG4tvGaU6XZXgG3F1Ty9AX7iSNebaKjPUc2izXkPwRevysCzvJXxRugs6hgGM1qANMX5K4",
        "tprv8fPDJN9UQqg6vVgkCrdowywX28TAJtDAeSBdorfiNzfoQKGw4agSkkztoY36iPcoMfk1D1yvy8Hpk7BECsHdw69FzksEHWnizZ3HPN3FzWH",
        "tprv8fPDJN9UQqg6xL675tW9BwCCYTYgYpKkFtgqPf2aQnqSrVyBpojJ8QXC1q5Y4fUDyq9rBqTTNfeWaXe4h6yYwn8fSHfNb52fKgyp5myQGcc",
        "tprv8fPDJN9UQqg6zhs7FijMy5CWDFNohDAgRnXFepunP9dZdaDwFLDkYtfDSKumPvckNaSce5NZUZkaq49pdW9guDYZ1nA8o2RobHCpghPFKf9",
    ];
    pub const ACCOUNT_XPUBS: [&str; 5] = [
        "tpubDC5FSnBiZDMmhiuCmWAYsLwgLYrrT9rAqvTySfuCCrgsWz8wxMXUS9Tb9iVMvcRbvFcAHGkMD5Kx8koh4GquNGNTfohfk7pgjhaPCdXpoba",
        "tpubDC5FSnBiZDMmkoat4aZFfbJdEthnPqJ1jXZcKWJNKC4yJanLA55dRW5qKJRRvAo1SwaXeUx2ayUQyVJ6eCbABbBB8Wn3T7dAuVJRnZgntVC",
        "tpubDC5FSnBiZDMmoxiY6WJQMPbdb9y6UDQ5DjnR6Ni1oGUCEoXhgyW2wFckyfRhA3gqFMVuZJ9NBvs89u5Kph3BQ9EBUazLdL1a9531x9hcaYp",
        "tpubDC5FSnBiZDMmqo7tyYAjbLrK7V4ci9WeqCHcgB4sq4dqgzDxTCYtJu94Bz89JQARd35pn3JkraRgKXVuJCAHnkdNBFxZsNMSN4NUeB8qW2o",
        "tpubDC5FSnBiZDMmtAtu9NPxNUrcnGtjrYMb1682wLx5oRRxU4Uhsj3LjPH5cTfiTsLqPrhy8i49xRCx3e1UeoEzADnWnYzC2tdohGWgvuytUX5",
    ];
    pub const WALLETS: [&str; 20] = [
        "mkpZhYtJu2r87Js3pDiWJDmPte2NRZ8bJV   cV6NTLu255SZ5iCNkVHezNGDH5qv6CanJpgBPqYgJU13NNKJhRs1",
        "mzpbWabUQm1w8ijuJnAof5eiSTep27deVH   cUATcNZMgKQn5vUYuVvKVnoQUKcvyJuZvHyHFHfoi5mm4E1T7Gs3",
        "mnTkxhNkgx7TsZrEdRcPti564yQTzynGJp   cNREak6acZyFCozMdabAUDCYaGmksJYgkG2i7QAWB4JcsyFaEhST",
        "mpW3iVi2Td1vqDK8Nfie29ddZXf9spmZkX   cMkqWcKh1t7uVo3qesfTh8KxtLaXyzujCWUX8sUJ6YvSzq9ZjgTH",
        "n2BMo5arHDyAK2CM8c56eoEd18uEkKnRLC   cVYduBSdbxiiLu5iAg7MQWS9VBNN5yNcB2qrHeo9pQHS7ejxSXNK",
        "mvWgTTtQqZohUPnykucneWNXzM5PLj83an   cTJxi5oL1PVpFRZC9M6UwxoQYYm6t9q6Snueg7SLVfkBDktAbkQq",
        "muTU2Av1EwnsyhieQhyPL7hgEf883LR4xg   cMm9ZtGPZ1eGWUrfmTCossm46KGv9qidjhchxSsLMW1XDyjrVewo",
        "mwduZ8Ksa563v7rWdSPmqyKR4y2FeB5g8p   cUQp3cYo5FPwT9QWJdeoxrK4bP8SJgiSvXNiXnRyFReznNjh3aJE",
        "miyBE85ro5zt9RseSzYVEbB3TfzkxgSm8C   cQ86oD4EmPDzFbkxJykTx1Ap4ZQD327NeTz4LCy1wYv95MNhubH9",
        "mnYwW7mU3jajB11vrpDZwZDrXwVfE5Jc31   cNZ8vJCC6QwViqExtGrSkHaoQjFtwasoSvyqHiQBcbWEM3nd4dRn",
        "mx3YNRT8Vg8QwFq5Z5MAVDDVHp4ihHsffn   cVqkkhtqHrGREqqZuwZZbxGubTUKHMpazs1hbPY36GrKfpqswRZv",
        "myHL2QuECVYkx9Y94gyC6RSweLNnteETsB   cR99FD41Xi75uTqXNhqKbakG5y7AZTmJfZSZFCdNyA3hmsaYjPNh",
        "mqevqtsdeR7WuqwiXnyFU72ULK627W2mFH   cQe4j6dZ4DPeFFu6rjiCKzynr1467uuCLuyhmkz1unsTtkb569Ks",
        "mmKyDn8NJwXvqFqWDNR9QnMfd8mwrHvynF   cVVVfpR1ZhL7JbKzPYXvecJmdaui5tUwLatxMzrP5Q4JE7tgWwGk",
        "mnDmjqLKEBBMnzWtrz5LptNChiQNxYLK84   cTx4Up81n9eHPx586rDZFMjRwGuMBnY4b2cFViVFmpUpuupVeo1V",
        "n1MsayUmxjiUyrbQs6F2megEA8azR1nYc1   cMdBC6AE1mCHmpfXs24sWdrE2Xuk6mDML9eQdjLamtjwP4RaAZ3U",
        "mhhTTZMmNTjT4zzS5xVpXSDan9iHy31Z2b   cPAWznpxqD2bfiaNNwjGptywngn5HLsQgLTZExpyupChiLyrGimh",
        "mp8ML8bKSiheUJPompTj5GZEWJUPmr1eiH   cRoV92z5PD5Bg96iYnXezhPPa6EfQL11Ukk8jk5LdJPD6Z3HLLfV",
        "mjtvWKf25G3heJkzVkBRYNmZmPypdEY3hj   cQwp3C6Fg2Y597RmPDXJmy9u1rqBDZgnwQBMZsC3LoA1bDbAvkQe",
        "n3Zb38sLaM21q8dwDNZq7AsJda9omg6PuP   cQnBoSiQpHMwkaxkJPaHHzgdb3gbZtn8HEzYDmrTK7SLySyg5SMc",
    ];
}
