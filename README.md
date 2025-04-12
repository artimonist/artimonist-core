<div align="center">
  <h1>Artimonist</h1>

  <p>A tool for generating mnemonics based on diagrams.</p>

  <p>
    <a href="https://crates.io/crates/artimonist"><img alt="Crate Info" src="https://img.shields.io/crates/v/artimonist.svg"/></a>
    <a href="https://docs.rs/artimonist"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-artimonist-green"/></a>
    <a href="https://github.com/artimonist/artimonist-cli"><img alt="cli" src="https://img.shields.io/github/v/release/artimonist/cli?label=artimonist-cli&logo=github&color=green"></a>
    <a href="https://www.artimonist.org/simple.htm"><img alt="web" src="https://img.shields.io/badge/www.artimonist.org-gray?logo=html5"></a>
  </p>
</div>

[Documentation](https://docs.rs/artimonist/)

## Example
```
let values = vec!['🍔', '🍟', '🌭', '🍦', '🍩'];
let indices = vec![(1, 1), (1, 5), (5, 5), (5, 1), (3, 3)];
let diagram = SimpleDiagram::from_values(&values, &indices);
```
The simple diagram looks like this:

|　|　|　|　|　|　|　|
|--|--|--|--|--|--|--|  
|  |🍔|  |  |  |🍟|  |
|　|  |  |  |  |  |  |
|  |  |  |🍩|  |  |  |
|　|  |  |  |  |  |  |
|  |🍦|  |  |  |🌭|  |
|　|  |  |  |  |  |  |

Generate **mnemonic**:
```
let master = diagram.bip32_master("🚲🍀🌈".as_bytes())?;
let mnemonic = master.bip85_mnemonic(Language::English, 15, 0)?;
```
`lady announce wife please settle connect april hour caution split festival genuine logic digital dignity`

Generate wallet **private key**
```
let priv_key = master.bip85_wif(0)?;
```
`L25LxS22MwRpEnnFs81XitJyrkimpZGLjgKHRAikLxJoxWMkVuHd`

Generate **xpriv**
```
let xpriv = master.bip85_xpriv(0)?;
```
`xprv9s21ZrQH143K47Cxw6R8QnGdAru5BaK7kT5awzC9VvmpXnpCQPdEmPyJeR9w3FeJ3hmEBRCRLGhMNpnkcM9q2w3J3T55bSSqMLRDpJLZU4B`

Generate **password**
```
let pwd = master.bip85_pwd(Password::Emoji, 20, 0)?;
```
`🙏✋🍕🌻🎄🙏👍🔔🔔🍺💊🍄🍺⚡✋👌😍🚗🍎🚗`
  
  
## Entropy Evaluation  
mnemonic 12 words entropy = (2048)¹² = (2¹¹)¹² = 2¹³²  
mnemonic 24 words entropy = (2048)²⁴ = (2¹¹)²⁴ = 2²⁶⁴  
_(In fact, because the tail of 4bits/8bits is a checksum, the real entropy is 2¹²⁸/2²⁵⁶.)_

#### Only Emoji Characters  
[emoji characters amount](https://en.wikipedia.org/wiki/List_of_emojis) 1,431 ≈ 2¹⁰  
9 cells permutation in 7 * 7 grid = (A₄₉⁹) = 49! / 40! ≈ 2⁴⁹  
9 emoji characters in simple diagram = (2¹⁰)⁹ * 2⁴⁹ = 2¹³⁹ > 2¹³²  
18 cells permutation in 7 * 7 grid = (A₄₉¹⁸) = 49! / 31! ≈ 2⁹⁵  
18 emoji characters in simple diagram = (2¹⁰)¹⁸ * 2⁹⁵ = 2²⁷⁵ > 2²⁶⁴  

So, **9 emoji characters provide the equivalent encryption strength of 12 mnemonics.**  
and **18 emoji characters provide the equivalent encryption strength of 24 mnemonics.**

| mnemonic words | emoji characters | entropy |
| --- | --- | --- |
| 12 | 9 | 2¹³⁹ |
| 15 | 11 | 2¹⁷⁰ |
| 18 | 13 | 2²⁰⁰ |
| 21 | 16 | 2²⁴⁵ |
| 24 | 18 | 2²⁷⁵ |

#### Any Unicode Characters  
[unicode characters amount](https://en.wikipedia.org/wiki/List_of_Unicode_characters) 155,063 ≈ 2¹⁷   
6 cells permutation in 7 * 7 grid = (A₄₉⁶) = 49! / 43! ≈ 2³³  
6 unicode characters in simple diagram = (2¹⁷)⁶ * 2³³ = 2¹³⁵ > 2¹³²  
12 cells permutation in 7 * 7 grid = (A₄₉¹²) = 49! / 37! ≈ 2⁶⁵  
12 unicode characters in simple diagram = (2¹⁷)¹² * 2⁶⁵ = 2²⁶⁹ > 2²⁶⁴  

So, **6 unicode characters provide the equivalent encryption strength of 12 mnemonics.**  
and **12 unicode characters provide the equivalent encryption strength of 24 mnemonics.**

| mnemonic words | unicode characters | entropy |
| --- | --- | --- |
| 12 | 6 | 2¹³⁵ |
| 15 | 8 | 2¹⁸⁰ |
| 18 | 9 | 2²⁰² |
| 21 | 11 | 2²⁴⁷ |
| 24 | 12 | 2²⁶⁹ |

**ComplexDiagram can be filled with 50 unicode characters in a cell, providing better encryption strength.**  
_(In fact, higher entropy values are meaningless because the length of the private key is 256 bits.)_ 