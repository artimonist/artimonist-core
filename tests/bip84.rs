#![cfg(test)]

use artimonist::{BIP84, Xpriv};
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
        let (address, wif) = master.bip84_wallet(0, i as u32).expect("wallet");
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
        "bc1qzqvjup3rktlrf73znrpzk4624qz53wevx0uppa		KxQduvengQDdUgp8LZ73XXpswQYzKb3AXc1gedGp2oJu4mTjvNAd",
        "bc1qxddz3r05vresrk5x0p25duyzp8vwear7y8ha33		L2gZznCuktRfnXAPBaEezDTB3wFUFm1Kkou3higacJvfUyMhudDC",
        "bc1qddl5ywj2stxk0vfdj0rk056mrakurpqpmceyz5		L5TLmFGXXnvb3WN6VSJrQRSRWPDAsAGSYrpP2AVGXhLJreNyu6fq",
        "bc1qv4jxs2up2rsl5ezzd802uhdparm74v87wsn2e9		Kx9f87Qm1hG7rEqa5YxdTVN42BqZK2q8Gsg2bUXcr57ncY6CH8h7",
        "bc1qd9pxnwwnlqqm2y5v49c0t2na7lfjys0h6q24za		L3Q4zLSY37j5vsVypQSQFyAUMSLaCZ3J4cM8VqqGwhKwypatS4Qk",
        "bc1q2ungt97p2ytr3zsl48tvxmyynlf2fhs3caqakd		Kxp5axLFbLbBE3AY38DMoGnyVEsTp7jJtqRnToJCnyf8dehoggZj",
        "bc1q9cqcfr9cvyuj5s6gktynyk6ch7cj2rfj89dgpe		KyhLNF7aLJoztFQaFM3S5BAyZ1e2YjCrhZeAa7AVpj9Wmki1s5D8",
        "bc1qx4uhzuyr0n88kpdtju5jfh0k5ytc5pf66rngvy		KwN7ZbhKSR9Fb5jWpYKK37z7qxNtUmjDkyVNNaLbzKFPc34SGfCU",
        "bc1q3qvjrt9lre59pnsgt83kfa479pplkxumhd3c33		KxufXvsowdXbziCZBo4ityRXtvA6mBCDRdDmTipfshp7KG2NUEyB",
        "bc1q46teak90symxx75g9cvax0xn70z06sd5t8epvq		KzJhCddXZhj2iCF3vXvtdfG9g2M48xGezWdbLBoLcWoAtaNKGs7t",
        "bc1qvr4z7xnmtq5gj7xw4y0mn7dza6l7m0qjse5kqf		L1pgn137Yrs1bNJ1rejFFAXba5eJmM1WgfBmMnW1bUQHGYLiysJi",
        "bc1qkgchq2sjpstnhkzvqjptuvgpkx2kpvr4y89pnz		L5az8ExSTUZkaE9iS9bqAoSHQAzp6UumeoX5wDz9rKzPDfsHvUWo",
        "bc1qwhqzcq8srgx49hlvctxakq2tpnf6jvqrjjhykl		Kxaz67ixxKXEtc7vu66VPFqtfdYKi4PHQn66ZULVRwJE9wRdUNFL",
        "bc1q2fzd3caz5qkdj4kajvu7d540a7n4hqvdl5hdww		L2rtyaXVv4o4AyWkMLLZ54QridFy6t692UCZh5iLUjBJA1uxFSNR",
        "bc1qf466ceh6thfyfxeyznfee43eqhtvys6vk029uz		L1T56n97K4y3r7953j6rvSWYSXvxMxnMwBTFT9JPToE8mvoke5Hq",
        "bc1qmvf4q90f5hyv8382xnxj655z2a74at655j9lnt		L4XQ4M9ziA4cy1DTB6RYyiobhLb9AH8KTmFU5gNDojbg5wUbBQoA",
        "bc1qv6shp233dzujxn684cnyzu0hvyxyrc705xe0zr		L5EE5Tr3DAFYevkCyiTGPBz4MHKkqBLXffiGQAqpH6NPS7ZBWYyS",
        "bc1qcdd6mfnlcgs7kqemlvl23dfa5ym0f3mehxxvlc		L3Eg3mHTS9M1u9tAcuvf4ijLfpUdZi2oxn9pVndjjmUgZBbUMvid",
        "bc1q5juh456g6nhdwdvnvm44zt6tc4euwt76drlmff		L5dq1AzjJtyiT6ehEnugw6wWvWjBFHkAHgPVnJxghNbZb5ZxdCa9",
        "bc1qv9fafhz8jk8wftt95tyjkqcl28ueagtudry935		KyoZS4hVFUfrU7S4kmmk77yeSoE7mkgyrKu8FDpHBTfiSE6WQZAB",
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
        "tb1qekt349zqrvvehwug0dwt43fjvaxak3qy0k0jey		cMfgBX4sLAEDwyweANnBZHhFN4rPvMqkJqzi2yLTHUBEagDWcWur",
        "tb1qh8c8ge4cgwnz0xeejkhsmd46kkajkw9k5006s8		cUYKyvDZ713rVQLLLa7Qd8Q2tVXg84mTjk6PCZnmayYtX3Sn8hE5",
        "tb1qtpndp8rtvkay5j7w6uc5fc8ut2e5kfdgtrvce6		cPCgVVKdTe5Qb9uxZZgPCzKvp9o6WvtzJB1g3o16awc7ax2y7dLF",
        "tb1q7x5atz5czu5cametc80rdppufac9rtwu8xyw6k		cNCLqt8k6eGrwTY9VkQJqP4LiL1QHLsqHtGnHmdDq26kz7C2qefM",
        "tb1qkceh8tgqgvy8y8ejpal6hkd7htzauua2xgkjvy		cRGFPteTc4dCUexrprZV4Asoqn1FZcD6s17hkVFNtDUVxpeHh543",
        "tb1qpf03r63yl0lf5k49lty998c27s67skm6yuyj9v		cUtMvYzUqLDGQRNR8Ec85jMfxC4bkAUXRaPJtravx5AM8QNzAsUY",
        "tb1qh4mr2lkg7k8f6mpvc8ncqcnfdqc38n0tejk3pd		cSQG5CJGExrhy3ueyNe4cZFvTe4BSrR4S37UuB8UyGmMn8uscord",
        "tb1qk4raufe4lh6p76lau7jfrzn3cw7wfcatngs92e		cSbdYb6uBQHeNVvZWH9FuK2TKLqU1RhoY9yuSC9b7o8Tdgc4YtkK",
        "tb1qjgrhr2vqg0kdfkrjx9f0wdug2y3j63639etxxn		cNHfFtD5R2uAoS9Hi6GZSB43hg2LAgQY3cPjxtn1T1MGLeKfUgen",
        "tb1qzvx6kdalvu0f2xwxrj52tf29nqu8l0mt8nhgwt		cPpVHC3LkaHHCHStKd6WPmRGwdbesHoUeCV7HNjb37hFwZ7vSSj9",
        "tb1qwdg85ernydqalq903d3uge62vf6ph0dj3j9rae		cQ4GX61imXFGpC6YmRGcXZvizanTn2sZLQbmpcW4vN9vrZ4G8ohA",
        "tb1qrdqe30u4sdlnl0jsg29vlxx44a7pz6eyr6jqrg		cQ2Ut3gHpwdpehfBjvLyUTe6u2KgKVrkced4Up1XHFHGXW9eiqkv",
        "tb1qlxfnprj9ps85l2gtduau3mtx782c7s7yuhpm03		cU2tskiMtvr1ozqAK3Sby1mShGCJjtccmSUft9qqmT82A1sFKvKh",
        "tb1qt2rzmtszhdxawny6al636mp8zy0exym5z23x8f		cW6uMzv51FD6yocsmaubRyFTCSAtyjVywp568EqY8UDqWzRLVbDm",
        "tb1qu4qp74gttfu7ann484vnhkyu6punn8nxge58ep		cS1AYFnyfTafyt92pJaEXHLFcnGrjSoVhUHPStVWC8gpNprCpNXw",
        "tb1qu586alfngrszc29a86ay03qlqgmqf87xn3r9mk		cTjc4W2Sn24pmFPpiW3gGA3QVPC3J1zwyLc3Nr3Ei8zKLtoqFHZ4",
        "tb1qkfpucdyh7xa5ttqh9qdalj2l4cc9z4xvnc2jcu		cVPSw7BwTLiHSAx6EDWHxjnHkJZcv6xzs2hdUsrqpvQZT1woL4ps",
        "tb1qzzqxyenqphtwzrre2y3vfxwzzw4fsuajzcv2r0		cVpMbMLdHqcRBT6BWvRExrdubCecach7T7G1RV12vjk4NgkASGut",
        "tb1qpmkc7twnuu0qsn20tjrtqzvpyhfx0mlwnw427j		cMb9xBETf69M9gQspzAmWXE63QjVfcdNahCumPCKt6pCrSxYKKtE",
        "tb1q0c9e7n05v9tx6qa97ne5ucly2kdvs4zp3jhtld		cQnCWSZ71C4ZAQC6SbF4Uo8qEbFCUzEbRmdYdvxEoohzwNuiomq3",
    ];
}
