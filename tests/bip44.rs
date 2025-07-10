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
        let (address, wif) = master.bip44_wallet_harden(0, i as u32).expect("wallet");
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
        "1NgVJi2e9v7pf6jYuqMo9bhJhH5GRwvSiQ		KzbTzbKeotsem631kMcsvxovtH8atjurUwyNo7baXNN3G1nKkiVx",
        "1Br1iDx4gxJCg7NqPvaXnRnqTSzHeGxseD		KzwAvX5sYb89vH2dFMTCipAMvAUAoXvst2LRA23tGQ6dPFX6dxd4",
        "1FhgNUpASU6NGmB7NdwHiRAhvkywLC4pR7		Kx7jWJDQyyfb7qS42Gr2Z13F9Ga7aSnLgVGNTR3YnPrycT4s4Tjv",
        "1Dr8vy6V8Ut7AmAXNrvRL4eoHgKWzcuafa		L3SCq3zW1qhSgVuiVguepBY7xUcFmeDXkBJSfAQjFtWgCSE916ne",
        "1F4Pe52s4dqrivPY14KytnU1nRai61YMLg		L167R2R443aaWGofqRPjLeEJq51H5ofpzZxDH1rjXcUdFvAmkLRM",
        "1BdHUNgD1VYp4mUYeBSDHjMUWJtjrHADtt		L52UsNzK6wZc8yYQy3u5aVx3MZdufCWvsfGp6S22oY6QwJvAnbN2",
        "1FqZ6s264b3nyTGX5K3ZjowpSCwuGd1Q7g		KzG1zDwLMdtMnPbXrUFrD4K339i6zqfr7FFUKDCTAmQuiTHNYgar",
        "1Fh97gs6xGNwfxNNRKY6xDwFDQTGAWxUae		L3rmc35j5N23nnGf1A4G2DXw74HDvyw4cgXxzXD33a8LEwwXUbyX",
        "1aKD5tqrJpFcdqUUVowMi24YySDdBM84k		KxTRvZFvdVTtJYJP6f7Gg8QgFiDbnUpjJJmuyqZmZrnedmGtcTCe",
        "19r6dZAaykzEq8RPnm2H4CozFHQxD3FzAJ		L31w32b2Qe5rCvScbhaCYC1nRcxYuixqBjPVYVu4LYZmsz99CQYD",
        "13vcyqwqELVDe2uRnNdMC8pjyJraQeksSG		KzVmyMY3JyPppz6DDdXLqbkANaCDTswWxwwiFdukz36W4dVNcz93",
        "1B4xagHEwvxLzpMCu5ykFtTUZWjVykG3B1		L2wWjJjXw9M51U1UydWLx5Dq2UG5aNwv3M7ZhEbtQg4A1T4eFosT",
        "1wQerrHacLXwTKTFxu2bqHJFqFHnWJvsp		Ky4RK9XPVPJLCjkGnnyVzTgJyQGxhwCav219r1pqtu724jVvNVen",
        "13unaaoEBFtUTHB98ttNn14SMP4TeBauvo		L5AoNuHCbUh6DD9PhrhHY758mpKptUgoaWdQje7JJuNi5qkEqhXq",
        "1K14R6Bi93fteVhCnD9Mb3sgaZ95fzrxkR		KygYWEhfU6iNiVtBTLG1tUu7xuVixPE8SkX2D8AhdREfKtytezz9",
        "1L8VwY591K8tGmtByMrFKZvCrj7B4bxY8H		KzHPDjkqHTc4kx8QSEAKc9tz9nsHP6C8oLwEkx26fZSYcCeqAeNY",
        "15Xa382HfGYB7bh9UZB2JjRdVdpXgg8zYr		L2rodQ4rSvRUVCfezLsStJ6Ce7hsir7qvSZ9RWizQ6LZHm2NYNm4",
        "1NUGqhQVRM8vJC9EUKk2xLzcbkbvBr2TxN		KymiNcBqKpRPH4sWWUGhJpC9SDCrENajWBh5UDYYqnF5ZxkB75mb",
        "1MMxg2W4boatg5DPBAc23MaeyV7Kx8wnX9		KxSrXpWZdwNd5eSC3Bu7bz4CuehS4mN21BC8kAS11pvdfFznajr1",
        "188UjFTJicz5cyFNrpqDfRqiiQiaFu5bv4		L1uG6tPf9NpaEfRTimHw8Qjiy82NVoovQ3xdKL7vu6dJjVwXyTPF",
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
        "mgoYaUaaaCK5Wo2NkMvdS3yyQeGR3mXmGY		cPxsK5KcJ5BhFnGwsqNGAfudhYM1xhsonEMvkb1Qq4VKV4hcRFw4",
        "msU4EiHEQEKMzGCSXu5cdNeXLomuK58RtQ		cQSq6gvctaLzTyHr7yE3hvHZssMys9DZvE4pg4aNrXtVnEYLtS5t",
        "mmHF1E1Rs5b2wDHErZYfzk5tJvUmiaX1zT		cQ3a5asbeqofmSWdQyZjdL8dySoZaFPz4X1LLSwY5Cyf3fCjcaNv",
        "mfqFQFwZy92FUM8ejq45oEmFo6imWRwkWm		cPpxwDAzvv1SKPrn2ABnXeeXLhBCWvi3WirsSwfa7PoeBtdSrihW",
        "n1MwhNM3S4f3hDiJTtThvtGqm2pd7HjV1B		cSPiKP9dTnTjcFi67BDaEXRCgS25zd9XdvZxYz8Ah1h3711dvAQj",
        "myhPUnjTr5gHHFKnAxWdBr1TdYyeToaLoJ		cRecphWQGUQgTSb3WwnT3mpTHwiTYabVatMVZLyScp8p2tkgzcWY",
        "mrMQNjhc4vSdum4jFv6yEWxscAEGdm3EPf		cRGn7tUn8td4QqCResjJSUWyvkcsxz7FVSrzb7Bn4L5geLiawyc2",
        "mm1SGxNfQmtgo7zZgfKh6746hxcYtn4P6e		cTEHiYDe54nopw3rctBznDERbtAeXtTvNoMLc9cZggkqRMdpJrBb",
        "mjXV6QPd5BTeW6qmRNnLfgLUgvqWwjfWyY		cS84aUPvWENYKgWDzD8RN3vF2bPjC5bYRHcPkpE85hSsdA66eaT9",
        "mxP51NoGmafT71aTiXbWqUnnEXH6B8V3VQ		cUgKZK1qxoWok468a2xhzMGT6QZ9v7m4ZGZXSLaio8kPAY65iyLi",
        "mq6EYE2Qt24BK1FZFs5gsnvvceDM7BKJmd		cUz4vY8NFcc5cnK3VpDAKaLg94oUktbC1PiNFoAr1kTBQqigwFsk",
        "mgciH2NcGrGbDZMS4JvPpfwMmX6Ar88wEd		cSwm7bg39Ve6gTjUEjY5NHWA6NcecyPX5hCK3adCwfM552ahGf2e",
        "n4T6NX98d5NaVpSXrBvFKjE3fcePtXcmbZ		cRDPi947DEHLHGA1vGrwop4MjhaqkvTxZA6mSEYgWZH3AcXQEHxA",
        "mqpTQoBNB4Sc4uyjQ7vvU2L2AYpMZ2iVKc		cMb7whqcHG2Qe2oXCdTCfHPZ5ZJrooawyBkKc6ZsHNZhAWG5VHP3",
        "mk83LfEgiBqUZ8NM2d9Ez7cYDDaenP4Uud		cQv13w8q7kHgUqX87EZq3CWm6ccC1uVx36N69mqmhCQVmjFoSxBJ",
        "miwyW5eXkjPrW4Et4uvaHz5ZGhAwMciE6t		cP6tu9BgUZk2EHdFZfDvsMvmHhSBZtcqT3H4VoGv2hakDJRn39Cb",
        "mmdAL1C25ZvCMhznhjvEChPT6Xgthxdf54		cTJgnNeUahfdxKGEQkCkN62Udrd8Xzy8feFqAnm9SXRD7C2E8Mhu",
        "mfxLWPkhVbeuKnwipuLCmGqQBe9e2sjR9F		cTwVJaNjz6CYBf5Qw2eUx9u1ew74taXdwdFU1pmejJYCLP9p2Rjj",
        "myy6r8Jq2aRHQitdrZFxqaaJppySLXLq61		cMtyCFLaKztv3v983BzNVJgrpYJR9taSTuyag2m35DCH9JdMLsS7",
        "misEAtav1GAsvpyYG6UPbEwzeFTJQebwWi		cU3yDr4G3b3wrwpp5VD3bXxJ69ZupwW3keVJdeLPXLn6jrkJGTtk",
    ];
}
