#![cfg(test)]
#![cfg(not(feature = "testnet"))]

use artimonist::Language::English;
use artimonist::{Error, Xpriv, BIP85};
use std::str::FromStr;

/// # Reference
///   <https://bips.dev/85/>
#[test]
fn bip85_define() -> Result<(), Error> {
    const MASTER_KEY: &str = "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb";
    let master = Xpriv::from_str(MASTER_KEY).expect("master");

    // mnemonic
    const MNEMONIC_COUNTS: [u32; 3] = [12, 18, 24];
    const MNEMONIC_WORDS: [&str; 3] = [
        "girl mad pet galaxy egg matter matrix prison refuse sense ordinary nose",
        "near account window bike charge season chef number sketch tomorrow excuse sniff circle vital hockey outdoor supply token",
        "puppy ocean match cereal symbol another shed magic wrap hammer bulb intact gadget divorce twin tonight reason outdoor destroy simple truth cigar social volcano",
    ];
    for (i, count) in MNEMONIC_COUNTS.into_iter().enumerate() {
        assert_eq!(master.bip85_mnemonic(English, count, 0)?, MNEMONIC_WORDS[i]);
    }
    // wif
    {
        #[cfg(not(feature = "testnet"))]
        const WIF: &str = "Kzyv4uF39d4Jrw2W7UryTHwZr1zQVNk4dAFyqE6BuMrMh1Za7uhp";
        #[cfg(feature = "testnet")]
        const WIF: &str = "cRLuXpEtagka2NVmVtg6pcSdUFHp9pqkhCQSweYhQUWMwkdaaVsk";
        assert_eq!(master.bip85_wif(0)?.pk, WIF);
    }
    // xpriv
    {
        #[cfg(not(feature = "testnet"))]
        const XPRIV: &str = "xprv9s21ZrQH143K2srSbCSg4m4kLvPMzcWydgmKEnMmoZUurYuBuYG46c6P71UGXMzmriLzCCBvKQWBUv3vPB3m1SATMhp3uEjXHJ42jFg7myX";
        #[cfg(feature = "testnet")]
        const XPRIV: &str = "tprv8ZgxMBicQKsPdh5yFmJBEQgjf3oaE8YyyEgS7CnEHXyPe9eGtubocMTq2BdvXjP6E9smCHogUm5ywmbfWPPhpVS3tM2MZbTaCPoTB1Yq51L";
        assert_eq!(master.bip85_xpriv(0)?, XPRIV);
    }
    // pwd
    const PWD: &str = "dKLoepugzdVJvdL56ogNV";
    assert_eq!(master.bip85_pwd(artimonist::Password::Legacy, 21, 0)?, PWD);
    // rsa
    // todo!();
    // rsa gpg
    // todo!();

    Ok(())
}

/// # Reference
///   <https://iancoleman.io/bip39>
#[test]
fn bip85_derive() -> Result<(), Error> {
    let master = Xpriv::from_str(MASTER_KEY)?;
    // mnemonic
    for (i, &ws) in MNEMONICS.into_iter().enumerate() {
        let count = ws.split_whitespace().count() as u32;
        assert_eq!(master.bip85_mnemonic(English, count, i as u32)?, ws);
    }
    // wif
    for (i, &wif) in WIFS.into_iter().enumerate() {
        assert_eq!(master.bip85_wif(i as u32)?.pk, wif);
    }
    // xpriv
    for (i, &xpriv) in XPRIVS.into_iter().enumerate() {
        assert_eq!(master.bip85_xpriv(i as u32)?, xpriv);
    }
    Ok(())
}

const MASTER_KEY:&str = "xprv9s21ZrQH143K2oVZZGYKXDp3zzT7sqFwmA7TXztxhKdMguQrmdXsxnrXT7jhuQH2TCgWTgChPmQUCkqnT6Fxtkb99KZE6GFBW2MMZGBqhnc";

const MNEMONICS: &[&str] = &[
  "hen run argue curious battle used census panel couple club stick bargain ordinary fame bid cabbage equip spring south surprise sand buddy angry tunnel",
  "thumb whale anxiety neglect gather replace picnic foster million gym anger lab upper unhappy recall pen math material unfold world syrup regular defense very",
  "fossil chuckle magic electric mom invite exchange toy human eye phone letter fox depart vital close friend stadium calm absurd police pipe laptop client",
  "play end hawk capable elegant lady survey surround render comfort victory base loud alpha insane fuel rookie warfare lawsuit earn during scene roast slender",
  "flag unique volume cute program answer wine laugh insane emerge student keen control ghost shove glow acid access twice local reduce license crowd foil",
  "siren name finish ginger quiz print valve section permit alcohol fabric gospel flower artist snap swift market camp source fragile evolve draw gather lion",
  "adult elite design judge lab square cousin urban wheat bargain claw lion wood drop enforce sister ethics rifle",
  "mad tent broccoli usage any twin practice spawn try security sand dinner oxygen prison secret guard sword surge",
  "install find pride betray smoke wheel away parent cruel chat sample come",
  "left govern affair salute canvas athlete adjust sunset manage capital buddy electric",
];

const WIFS: &[&str] = &[
    "L5nxqPAVHzRtfXoXYfkN9QwdreiD1phDJjQ3gbB3qY3RCVeGa52e",
    "L3xwgEA3QVe8yeJ5ft3AYu7kmAvwtyqT9nPvFZmK9JgUqvtmjfYb",
    "L5mH6wSp8wLZS6RPUz5vswUiniAB8PjFRs776jtUiM7nGSHfRJzf",
    "L5AMyUoJ37FCjhjccFbYHv4D82r9TeHYeZCaw63JsteL8t3jrgr1",
    "L29Ar6DZydWgzMTEk6N2NdQZVt3Kvz2HScSyHz4ska6sDqjSwdHq",
    "L1838SuXC7QMLpP1iTaYVE5TjWgLTvV4DgaGrmAbXrh2iKaDd4GM",
    "KxrPUsyTrf3GKGcLnqWdnKD1eUdGm58ge7iCUbimqbAcnXgc4xK2",
    "L4ohbgCpUNBGKd3ZZWhzDoPmv77BJ4yHJRvmDaXk2WaxHHhMPWYA",
    "L3s3BaF1n6muPTbSu8PAQ1oe1vpE1LDvok1zB1abnVu7CajgfQkJ",
    "L5gutB1i2Xa2Hjfg22RRNJgiMo4RzvrWYSb11t6xsBmmBeVcse52",
];

const XPRIVS: &[&str] = &[
  "xprv9s21ZrQH143K2zu81ow68qkwiEd24XpVh2kV5q7m77vV934zsnrwkypRD5FJwRn8gC8twKNaSUy6oScTuHEEE69g8EBCMwjgk13mXoHMn4s",
  "xprv9s21ZrQH143K32UKqSrsHNX8S3wdJPPCZ6hPV4zw9k2LDNU24KmtfShT6iosfP6wbBFe1tG1GRzQhbDBJe7QbRmzK7LKF8PZi129dbakjGj",
  "xprv9s21ZrQH143K2eUuxQsskkGqUjcfkecqAmN8Cr9WXHFWtKxYm5ja4ieMgBWVThK2XipGaWAgM22GyM4NvXrAi2kdVGUbmV4w7BVGZEi2rLD",
  "xprv9s21ZrQH143K2bYKXa39nNfoKFtSZt3FTC1jYFaUJyU9xr7XmQrMsRwrHPU6nD9aCCU3cEKL7ouc2rTyACeWACRc6ie77z5FF7JLXw6ze8Q",
  "xprv9s21ZrQH143K3sW7ojvdesWNE3RYC2zLsxe1vVZby4wsU152RXQ2GLxWTtuaUqSubCdfFQ4Z58RMNjgBp4AqNkvMUEqP2JRULEHQc3HR7oc",
  "xprv9s21ZrQH143K4FaPzpwaqKJ83c5QnXj5MSo2LntMnbu3g6oyqfMkKMU66gzE4MWUCheYf8hZExTQkSQvWbbNHknzre8m7tMyUaAmW8iT4uf",
  "xprv9s21ZrQH143K3NoewWhwDUE1FetvivrJ2P9P8PCeqjqbj5JoVLvzovskbDXzrT42LHqxka4UfroaPjMRKgtZw3pg1ysTzRAevwJCXAheZhQ",
  "xprv9s21ZrQH143K3jSs1QnmBLhtV8ydWChUUFX3rHrvTXnHnCZ3zVkV4NPMvWdtui2Ezjvb1S6tsMYAhRTXT4SP7kisiBAZuFQc4x7WoMXmLbh",
  "xprv9s21ZrQH143K2QYDTBNWktxHEi1r4qnQAKXLem4gz3xt7GkMYy2Bny2G2Wjiggv7LLff8oEnmKxN7REvCeGacqCxM1aXtDKbspQzZfC9s2G",
  "xprv9s21ZrQH143K2TiR1zN6Siw88rUxPV7FTiX1hX3ZEyFYeBaKevsHVeCzbLyotN7B8kAQ8c1yAhN9QEvieZ3Z77eRVBJBvvxtDTLa3pidocU",
];
