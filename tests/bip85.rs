#![cfg(test)]
#![allow(unused)]

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
