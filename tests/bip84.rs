#![cfg(test)]
#![cfg(feature = "extfmt")]

use artimonist::{Xpriv, BIP84};
use bitcoin::hex::FromHex;

#[test]
fn bip84_derive() {
    use test_data::*;
    let seed = Vec::from_hex(SEED_HEX).expect("seed");
    let master = Xpriv::new_master(artimonist::NETWORK, &seed).expect("master");
    for i in 0..ACCOUNT_XPRIVS.len() {
        let (xpub, xpriv) = master.bip84_account(i as u32).expect("account");
        assert_eq!(xpub, ACCOUNT_XPUBS[i]);
        assert_eq!(xpriv, ACCOUNT_XPRIVS[i]);
    }
    for i in 0..WALLETS.len() {
        let (address, wif) = master.bip84_wallet(0, i as u32, false).expect("wallet");
        assert_eq!(address, WALLETS[i].split_whitespace().next().unwrap());
        assert_eq!(wif, WALLETS[i].split_whitespace().last().unwrap());
    }
}

/// # References
///  <https://iancoleman.io/bip39/>
#[cfg(not(feature = "testnet"))]
mod test_data {
    pub const SEED_HEX: &str = "5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc19a5ac40b389cd370d086206dec8aa6c43daea6690f20ad3d8d48b2d2ce9e38e4";
    // pub const MASTER_KEY: &str = "zprvAWgYBBk7JR8Gjrh4UJQ2uJdG1r3WNRRfURiABBE3RvMXYSrRJL62XuezvGdPvG6GFBZduosCc1YP5wixPox7zhZLfiUm8aunE96BBa4Kei5";
    pub const ACCOUNT_XPRIVS: [&str; 5] = [
        "zprvAdG4iTXWBoARxkkzNpNh8r6Qag3irQB8PzEMkAFeTRXxHpbF9z4QgEvBRmfvqWvGp42t42nvgGpNgYSJA9iefm1yYNZKEm7z6qUWCroSQnE",
        "zprvAdG4iTXWBoAS2cCGuaGevCvH54GCunrvLJb2hoWCSuE3D9LS42XVg3c6sPm64w6VMq3w18vJf8nF3cBA2kUMkyWHsq6enWVXivzw42UrVHG",
        "zprvAdG4iTXWBoAS5UacB71CnJcGsysMwFKYQNVYuvEswDuKtWoGaYv8f8sE8Ct5tdBtSqyDFUvGaFqyQnUNeN3Sy4rqvccrvKggNY7fVJDoR95",
        "zprvAdG4iTXWBoAS7QDczVnyf1rUes62vzZpvt64vynpXQyHqLn24vM3Cb5UUriFp8vrYSMcLfwMCzMjopmDhym1MuksdDCYCdB38jWJDTARSne",
        "zprvAdG4iTXWBoAS8eBiQh5Wg7pegHJFWFYLeEz3ZjRoRviMaX5SiK6d8PdNmY9RkAvRB5PYdAev6ib68HgFYrFbmm4LsxzTZzg3Rf4fGUi43BN",
    ];
    pub const ACCOUNT_XPUBS: [&str; 5] = [
        "zpub6rFR7y4Q2AijBEqTUquhVz398htDFrtymD9xYYfG1m4wAcvPhXNfE3EfH1r1ADqtfSdVCToUG868RvUUkgDKf31mGDtKsAYz2oz2AGutZYs",
        "zpub6rFR7y4Q2AijF6Gk1bofHLs1d66hKFamhXWdWBup1Em25wfabZqkDqvaieV63fDQFaYmaatCG7jVNUpUiM2hAMo6SAVHcrUpSnHDpNzucB7",
        "zpub6rFR7y4Q2AijHxf5H8YD9SZ1S1hrLi3PmbR9iJeVVZSJmK8R86EPCwBhyTaycoeXEVqLigViktQUy2tt3yLnvcZ7BcXz9QxHrLjaTeJn3xL",
        "zpub6rFR7y4Q2AijKtJ66XKz29oDCtvXLTHgJ71fjNCS5kWGi97AcTfHkPPxL9GNPzR2TaqfcJx2WrcfQEHCjx7LcJz3jwwvQm4D1fcW7aiGxfT",
        "zpub6rFR7y4Q2AijM8GBWicX3FmPEK8juiGC1TueN7qQzGFLTKQbFrQsgBwrco3DgKidS4DwYUC12UULUux5XvPtgzmy1HoDpDhGABnnEyBQzsL",
    ];
    pub const WALLETS: [&str; 20] = [
        "bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu   KyZpNDKnfs94vbrwhJneDi77V6jF64PWPF8x5cdJb8ifgg2DUc9d",
        "bc1qnjg0jd8228aq7egyzacy8cys3knf9xvrerkf9g   Kxpf5b8p3qX56DKEe5NqWbNUP9MnqoRFzZwHRtsFqhzuvUJsYZCy",
        "bc1qp59yckz4ae5c4efgw2s5wfyvrz0ala7rgvuz8z   L1WWMekCNikUJwrkmxqXWafFu3gmzJ777WpPGgz5Y1o7U11hrDDs",
        "bc1qgl5vlg0zdl7yvprgxj9fevsc6q6x5dmcyk3cn3   KxJU9SA93qW5aCvjGsSgP5VeJZJEH9mncd7xKiwvWS17Zj7uJfYc",
        "bc1qm97vqzgj934vnaq9s53ynkyf9dgr05rargr04n   L2QCEZGLY6ZziKLbBqxeVtWr6n2oLYvm9vqwKszVjKtibjRSdZws",
        "bc1qnpzzqjzet8gd5gl8l6gzhuc4s9xv0djt0rlu7a   KzGeiLiLVFB57HCJSSxKZ5jMskkfmoLu8Dbuk71EdxdvKW8TnN3j",
        "bc1qtet8q6cd5vqm0zjfcfm8mfsydju0a29ggqrmu9   L2RTnCyi62n1B4M9G4kbBSSneUndyyZXnpEo3nNPqufVZjMnYYeu",
        "bc1qhxgzmkmwvrlwvlfn4qe57lx2qdfg8phycnsarn   L5WhDhMhur5ZXu1DoivXXsunpBAWa1LAxim3oWueBc72RCyRre3G",
        "bc1qncdts3qm2guw3hjstun7dd6t3689qg4230jh2n   L3WT6HSTqCULLpzjgY22tbFUxPki2VNckQLsj1W3qUU4zbpL1KnE",
        "bc1qgswpjzsqgrm2qkfkf9kzqpw6642ptrgzapvh9y   L5ViR9iWB9GcidHex2ebJyYSZJxAnud1Zffrsn5MWRWtY5rHRhfo",
        "bc1qd30z5a5e50jtgx28rvt64483tq65r9pkj623wh   L2NMiGJSGKJU8dsHexkSiDmikFS75n35m1XRnsENwvKKY6ed432Z",
        "bc1qxr4fjkvnxjqphuyaw5a08za9g6qqh65t8qwgum   L27RpvZ8QxQ6HTePGPCCNoGv8KTyAopdAXXDKWnsWns52JerUCpF",
        "bc1q8txvqq8kr0nhkatkrmeg7zaj45zpsef2ylc9pq   L4q6g9CedjHd2HzLNVtYR9aztp41ynGyHWJGUbgd3WCV6TFHZ4fm",
        "bc1qgr7f3jfuzhpe45h3dnqxxjr3ml0de4ad2w3ysd   KzMzhX7LzCEq5uyCRh4HRzZwFu164BddxnQHW8cHyZU7gnVG4CPa",
        "bc1q4fxs7lhw70m7nn7u6hqsa0glyt045ls5vdl6hs   L3FHnpvUsX29FyiDHw1ovPKV6tB79xxV7Diqs3Lw8jDx9Kh5sVV7",
        "bc1qgtus5u58avcs5ehpqvcllv5f66dneznw3upy2v   KwtZzNcXasfJ5h2npAxgZvqW61HzctSC2ptQ99uBhKvqBnwYyoWn",
        "bc1q7kv2wwzgh2zej88ywrjvnpvmqy2emefc8ar3za   L2pPtVjevHsFps7dY97oEy1Q6GRTqF3XTDbo4EEMgqqVQrWFP4aS",
        "bc1qrz46a4gt0sghvvyt4gy5kp2rswmhtufv6sdq9v   L2UN8yp5j2fz9eYv3ubbYPEdbcvBapm8p459Q961mrAKkrWXTKHT",
        "bc1qf60uv69k0prrdxkpmh94u9cwmkpkl0t0r02hgh   L3QNMsEknpvLMN8Ss7WUr5NzY55FyQ1qSosHnuWm9euJor28smyP",
        "bc1q27yd7vz8m5kz230wuyncfe3pyazez6ah58yzy0   KzR98GtShmc26WDUTHw9sqjCsDib61FMzqaqohjwGnCfbeRoQigD",
    ];
}

/// # References
///  <https://iancoleman.io/bip39/>
#[cfg(feature = "testnet")]
mod test_data {
    pub const SEED_HEX: &str = "5eb00bbddcf069084889a8ab9155568165f5c453ccb85e70811aaed6f6da5fc19a5ac40b389cd370d086206dec8aa6c43daea6690f20ad3d8d48b2d2ce9e38e4";
    // pub const MASTER_KEY: &str = "vprv9DMUxX4ShgxMLfvb8sFY4xFFKyTibwTfoydH3beVutr1L3bWHhRn3f2SqSo3vdUacd6QuuUxmN8BYoGhX2J4okpwCMh4nwdq9EqbdGgioRF";
    pub const ACCOUNT_XPRIVS: [&str; 5] = [
        "vprv9K7GLAaERuM58PVvbk1sMo7wzVCoPwzZpVXLRBmum93gL5pSqQCAAvZjtmz93nnnYMr9i2FwG2fqrwYLRgJmDDwFjGiamGsbRMJ5Y6siJ8H",
        "vprv9K7GLAaERuM5CAKPEd5qaDFXn67e95YPxcSUXpD7A1dvei4bQLCuH8DDz2RjtR5bS6nHyoSXbaMZ2K2DzVUrZ9SAYjwuZV39iTyRsiQG7N9",
        "vprv9K7GLAaERuM5DFr1Ttaid8DLounReCnQiAHUxsC5uRLrmnNzfNpJAG5tMjeaKewgZPSDXF7FPHtQYJheUhKWfi7HrtcS32ZpxDffPXVJch2",
        "vprv9K7GLAaERuM5G14fahHyTr9tWpnnbN7rRiGuRAXMUMFVTNFfrwpmf4uA2LgCKffNTAYvh8Lrh8iqJcppBYGYJhqpLz5Gi4FHRK2t1KvF47k",
        "vprv9K7GLAaERuM5JyVUaYvnGreXKBAhNut27A5FQL9NyPFuPCt9HNqtivxnfUMb3XWMNUiJr7spe8SSK9dGnaWwZHb3xdVTT9nWmhAtDJKksfp",
    ];
    pub const ACCOUNT_XPUBS: [&str; 5] = [
        "vpub5Y6cjg78GGuNLsaPhmYsiw4gYX3HoQiRBiSwDaBXKUafCt9bNwWQiitDk5VZ5BVxYnQdwoTyXSs2JHRPAgjAvtbBrf8ZhDYe2jWAqvZVnsc",
        "vpub5Y6cjg78GGuNQePrLecqwMCGL7x8YYGFKqN5LCciiMAuXWPjwsX9pvXhqKJdkzDeoE9xvFGM1j6cVLPqHEVDK5idBAye5LzWyqxjXcen358",
        "vpub5Y6cjg78GGuNRjvUZv7izGA5Mwcv3fWG5PD5mFbhTksqeai9Cv8Yi4QND26sa1T6mcQTXMB91biBrBKfSFrfQKeguxmqK1cga6QBi5ZS5o5",
        "vpub5Y6cjg78GGuNUV98gipypz6d4rdGzpqhnwCWDYvy2gnULAapQV92CsDdsdM5SKfQxUqwQHUpFYoqrcmarxjkVxiUMJfQW9ZDwgz2iGacU5X",
        "vpub5Y6cjg78GGuNXTZwgaTndzbFsD1BnNbsUNzrCiYzXintG1DHpvA9GjHGWkGCcrjui8XWYqGTJNJcvHyvYWCdRtvFuAmujMFUpAEby54Srm3",
    ];
    pub const WALLETS: [&str; 20] = [
        "tb1q6rz28mcfaxtmd6v789l9rrlrusdprr9pqcpvkl   cTGhosGriPpuGA586jemcuH9pE9spwUmneMBmYYzrQEbY92DJrbo",
        "tb1qd7spv5q28348xl4myc8zmh983w5jx32cjhkn97   cQFUndrpAyMaE3HAsjMCXiT94MzfsABCREat1x7Qe3Mtq9KihD4V",
        "tb1qxdyjf6h5d6qxap4n2dap97q4j5ps6ua8sll0ct   cRe5KDj3rcZJAtZVmWe3G2rdGdXyKCjVbWBVDXqSg2WHq1qq6MNe",
        "tb1qynpgs6wap6h9uvy7j0xlesew2w82qn038zm5km   cVeaVbQmeiwkYUqoWcmGJMHLppY8N1DtZqQ8tFUchhJZa6AKSMXd",
        "tb1q677973lw0w796gttpy52f296jqaaksz0555pg2   cVUnp4ArCgWBBNCnat7YeeCsoZPag3cKHhxYp9YLhfKS96MjLKzm",
        "tb1qr7scvm07ta0ldzlrmk7rnmc9lk356yar6zfu45   cVcKpznDBkA83HeX6beHD7D6EJ1XRkcP8mrHpnmMUzkgJwtTkppp",
        "tb1q4e9q5taxnsvc6m0uxv6h75mkzvnkxeqkckuztr   cNmkULcPZC4gXGVJTGNzNnw1Me1QpeEVnxY8qGBpKNwvaR5er4e1",
        "tb1qfsryn6hh2yhpxpp7m9dh54x89wettyfk45jn6y   cVSKaLPiLMG5BDzsJDXSqqAzJedGjJTCVTdEKM8eZ65Pdo5v3UYY",
        "tb1qk9ca9jh7a2muk2venu26qsc2an5cvnwpetqelf   cTb6dS7GbsvRQdypGVmExBRV1bJq8Pn4fyx1SSdViksVLxnkKtPG",
        "tb1ql483wsftk62xvt4k5w608h2w9yy2nrnmjlzzx9   cS1TKTLUB8i1BWhZax6M3AxfrjNUQzAKq2UVUz82mN4GP99r85FY",
        "tb1qz62u6t0px5tpyplrxuh2zyw6ycejyt9jd8glze   cNUeAibgrGsMhZP9FUiihbfwNvnC8JLkaiwyzgwXSGseHXJvm4LA",
        "tb1qextge928njsn94qu5jhc80uyx3wpz0fjqneen4   cNZXVKCzfoT3RnEHKL3GzvADvw4G75QjjMqgAmjeV6okS8cWpdDn",
        "tb1qevqc0f9vv69y8l5328heyap7vh4sm3pf7zvhhe   cVWK73DKKsfEjMDGeAhXHZHj1Jn8KpLPLwoTaEuX5cCWuJD9Tk9r",
        "tb1qdl5kf4928qng9t28cntwslha9ppwxdcceun2tg   cNyrMCtMyVC2eZ9aTytQTatadb54PKjwzckJfMZ51icYbwhsQHe7",
        "tb1qqs2f8gy3d42pwj68yk7y42fa8cfr59uper5403   cSPShFXDLg54kXCzjm1qWLp2PVcCwrX4HAcQhsb2LUZizcDBVn69",
        "tb1qhtwqm3x7wn0zteznkkzpamzrm345js9kd9nxed   cTYXXU51E6bUjL5J5Xytxqm33L8KhNzNQeYqUB5McDjzsHNuTRSR",
        "tb1qfvczjgwnc6l4tr4ee8vlffr6hznf7u28xnm2yh   cMyARrzYF8KbmdaowRvZpHgxhsB16TwJtM2pcBenfkhsHSkWoAb9",
        "tb1q8sh5sqh7lml070lxt540408xuhklxkvytnq9yt   cRZs9WJJptzZDkbMgGgLHskpjdaecX7zzFJ9PdSMwpzs8L6NdYbV",
        "tb1q7lttzr63l6z89rys3c8zh9rt6aj78feldp69yp   cU2NDE4JSxVPhdce3vpAKHviu1Kc7ettKE6J2SqX8ruUo8jWnwas",
        "tb1q4kestxh2w7r7h5hxvn4pn2qv2dldvylgj6t2kr   cQfiQNGJDNRpGZDyLWSN1HrjSHVKBt9pe8ntBTW2tyU5aCVtVegW",
];
}
