#![cfg(test)]
#![cfg(not(feature = "testnet"))]

use artimonist::Mnemonic;

#[test]
fn bip39_mnemonic_english() {
    use test_data_english::*;

    for (i, str) in MNEMONICS.iter().enumerate() {
        let mnemonic = str.parse::<Mnemonic>().expect(&format!("mnemonic: {str}"));
        let master = mnemonic.to_master("").expect("master");
        assert_eq!(master.to_string(), MASTERS[i]);
    }
    for words in INVALIDS {
        let result = words.parse::<Mnemonic>();
        assert!(result.is_err(), "{words:?}");
        println!("{}", result.unwrap_err().to_string());
    }
}

#[test]
fn bip39_mnemonic_multilingual() -> Result<(), artimonist::Error> {
    use test_data_multilingual::*;

    for (i, str) in MNEMONICS.iter().enumerate() {
        let mnemonic = str.parse::<Mnemonic>().expect(&format!("mnemonic: {str}"));
        let master = mnemonic.to_master("")?;
        assert_eq!(master.to_string(), MASTERS[i], "{str}");
    }
    for words in INVALIDS {
        let result = words.parse::<Mnemonic>();
        assert!(result.is_err());
        println!("{}", result.unwrap_err().to_string());
    }
    Ok(())
}

/// # Reference
///   <https://iancoleman.io/bip39>
mod test_data_english {
    pub const MNEMONICS: [&str; 5] = [
        "remind harvest mountain swim romance sense grunt current culture draw pistol favorite",
        "remind harvest mountain swim romance sense grunt current culture draw pistol feel mask blue tube",
        "remind harvest mountain swim romance sense grunt current culture draw pistol feel mask blue truth manage relief august",
        "remind harvest mountain swim romance sense grunt current culture draw pistol feel mask blue truth manage relief amused truth gravity sketch",
        "remind harvest mountain swim romance sense grunt current culture draw pistol feel mask blue truth manage relief amused truth gravity slight problem rocket mule",
    ];
    pub const MASTERS: [&str; 5] = [
        "xprv9s21ZrQH143K4asoSH4zmwMrJvxkeFwokE8UpDmPGLvSD8XNTpZHNNsuG5jFRYw3NQEM9zB6sJNVa27RvDBVJAcmrnkZ6W6StTxe4oYVTs7",
        "xprv9s21ZrQH143K4SdWxYU8S5YYzZoseouR2zPka7mfECDPimhMygaJiq8VXiVwhtAhQKnbMZh2f5aUtJS4xNZTuqELC6amqz1rRPWxXQLsS5T",
        "xprv9s21ZrQH143K3SbgQTBgnN9NvdNw1yMpgR6ijeKxsJvHAdy5RrbBXvBfhv35CZjfB9CPw49UvAvwaWPtdh4srjeGYgenE8fEFZFkVYEoU9d",
        "xprv9s21ZrQH143K2fUEWa7UkHXiKqfVMyAUn7eP3JsgKvccKZyHaPsTZ1nrz1dPQSFFytmmgN1vgEN5x7JUPkZg6xQjYmeqAPASRz25rcwiJ7n",
        "xprv9s21ZrQH143K2mswwMzKbYfcfW8bBAk3cyKeA1uUWXBrregZG3iove38mxLrsoN4piBo7dMRurUgLwxohusmWNRBdvAKWyucth3mNRnrYpV",
    ];
    pub const INVALIDS: &[&str] = &[
        "march cost giggle innocent blossom carpet region improve panel impact below wolf never marble battle topple lock wage",
        "函 布 语 厚 象 导 届 宪 旅 亮 意 叠 雪 凡 第 床 棚 腊",
        "manga choque gasolina ironía aval broca pipa ochenta impulso imperio asno volcán manejar muerte arte terapia logro vecino",
        "conoscere gregge longevo bagnato bussola ricreduto lido pianta levigato atono voga naturale ostacolo aspro terme mordere veduto",
    ];
}

mod test_data_multilingual {
    pub const MNEMONICS: [&str; 15] = [
        "know emerge excuse warfare tape rival bargain often box bottom palace wrist castle dragon caution coral relief famous return intact camera search opera letter",
        "ぜんら　けんこう　こくはく　らっか　ほしょう　ねんかん　うがい　つたえる　えいご　えいが　てさげ　ろんぎ　おそわる　げきとつ　おとなしい　きおん　にんめい　このまま　ねそべる　せっきゃく　おかえり　はさみ　つめたい　そとづら",
        "laurel echar escena veloz suplir previo arpa nervio bache baba oca yerno bruto destino bulto charla pitón exento pomada jarra bonito raspa nobleza lino",
        "夺 服 杂 葱 勘 博 比 乘 头 求 荡 溜 增 举 受 持 贝 副 桌 屋 给 邻 脉 壤",
        "奪 服 雜 蔥 勘 博 比 乘 頭 求 盪 溜 增 舉 受 持 貝 副 桌 屋 給 鄰 脈 壤",
        "hormone domicile effrayer vassal sismique placard asphalte menhir bataille bassin moteur vorace brioche demeurer brutal chlorure pelle encadrer phobie grenat bonifier prodige miauler indexer",
        "meditare errante faticare venato strozzare rizoma asettico pari bava battuto petulante zelo calesse ecco campale conferma rifugio fisico ripresa luminoso briglia sbancato pedalare misurare",
        "식물 발레 법원 현지 컴퓨터 전체 과목 왕비 권투 권리 원장 흉내 꼭대기 물가 난방 대문 장차 보편적 적당히 스물 기숙사 조선 왼쪽 실현",
        "nudle kedluben koncept zdatnost usoudit semeno cenzor plivat dareba cyklista polibek zrychlit dozorce jindy drozd hadr rozmar kouzlo samota nastat donutit sluha poctivec obvinit",
        "hino demanda ditador vassoura silhueta pesquisa apagador marmita atum atriz milenar xadrez bicudo crer biombo capacho papelada ecologia peixe goiaba batedor premiar matutar inalador",
        "たいりょく　ききて　しいん　せきゆ　うよく　おしいれ　にんち　すまい　てちがい　ずほう　うすい　れんらく　たいら　ちてん　うくれれ　みてい　そんみん　らくご",
        "okouzlit hanopis loudal naposled cisterna dotknout rozeznat nahodile polynom nahlas chochol zoufale okno periskop charita vklad odjinud zazvonit",
        "函 布 語 厚 象 導 屆 憲 旅 亮 意 疊 雪 凡 第 棚 床 臘",
        "약품 대장 상관 순수 구청 김포공항 장사 수석 월드컵 수박 광고 회색 약점 예약 관객 특징 아프리카 현관",
        "jardim capricho expandir gingado assinar berlinda pancada gaveta minerar gata arame vitral jararaca lustre apertada surreal injetar vantagem",
    ];
    pub const MASTERS: [&str; 15] = [
        "xprv9s21ZrQH143K46gsuZbSHQMpjPrXAyp552iyrCxhuvZoGmZb1VWGFrSmaWagp16a6aAb4CMxxo5oEHSdfm1T6WFVneV5QBumy8adPcC8kVr",
        "xprv9s21ZrQH143K3HYWAJcKqbTbfQyrZNadnNqKRYfTpH4QGFaDgn6ctCEHobYttNQVgjJ1VzpxZYzUTe3NVStvxsEJBTiZbKzshk9ADh8HgNB",
        "xprv9s21ZrQH143K2pNxcF5kbMAiLN1vtJj7ZuE2VYuVJCKJFZWMYUrAqvMyaAsayit7Zmve94xJaWNm7ESdkpqTt8VYx95CDdC1mT3SxVC4hUU",
        "xprv9s21ZrQH143K4Mp6bZWeQUhbTojCTyt4xHoTZZcZHwzHVoWfN7G46PjcD8Buz4ZXMorY5FNYyQoYfjcV5MXv8XHRWAnMnK2TJvFCAvoKY1a",
        "xprv9s21ZrQH143K2hJSPPaXGmnE3kTHpLNbQEEkeabzBNSceDyZVBCDjxFberWKMn2nhFjkhLSDxn2saqCjZtPFVpf1TRch4hTeJyqShg64gZy",
        "xprv9s21ZrQH143K2SMZDVCe4TVWadZrr7ePKeapLnAau8Vut48S2FEbppWSCqDxjFWvQ7HAySW6LD2mQci1kHRfiEBr9kYCCqaVLw4Gv1ZMHHX",
        "xprv9s21ZrQH143K4W3Y3A8R7v8oZ6T1ig43sbEV7CB38oXAX96BEu3Kd6hbqDVGXCNpJ5VMtckUwkrjyTfVD6a2uRudcac7Foikq77W8iKXJt4",
        "xprv9s21ZrQH143K4NBBPXKYv5tRbb97j9eALChk3pG5g5B7emsifF1d4qRGtePDwfmjjHgGmaUpjRUcnULEvsaNzCrqPDr4yFfLcgEJPZRbboT",
        "xprv9s21ZrQH143K2NrdZqx24PvWTZT5nBJk8iBxNxA5v24NRbeHNN3per4as1VRVStHkvYDTrauApFCe9woYZ8skDcPnvGVAg8A4xHJ5x49G2v",
        "xprv9s21ZrQH143K2otMzKrcnrDUvUgqnY78c22w8ySz8kWzULWkC7aQiutwtQjYWs8Mcb8eMgCnLncx8YX4dAczhEKzb1dY2bkbryr41rJkDyp",
        "xprv9s21ZrQH143K3Fe6AFYswDDBm876rRAHPRqhpaRyeDxjnqzwYMwx2jKGG4rtGsgf6rhPJQpitDnDYizfEifDE2kUHjgVYndjTNjYptZGf8d",
        "xprv9s21ZrQH143K3S2PYNTthd4cjoAwgL6gM9J5QadQEsc9Zs7CrzwpyduiDwMLJUGQPTBjVtdoU4NoBRdL77WfaSNSZfbhiewYw8PFUj3sBhh",
        "xprv9s21ZrQH143K4WAHK9P4WTmDgcxQZocnduVQ1NGAV7zuzPasC7ZhTgE8uek5tZMHmBuTPR9AzTxcE7euQ6CY8xgdjLzzqUJvmaByY3b5Tkr",
        "xprv9s21ZrQH143K2XADqmkgh6yDLsMmWEgE4SCRUzWd1DwDdgBeuiysrM83PwpK4TjFaJRSpDxXbB8Z8fdiUL5Y3wF4KEGWfzPT2Gb87qdTHYj",
        "xprv9s21ZrQH143K3RvgxwrpBEvT5pcBVrZ4jtUEAgWR5D3qpJmUd3c5Q8FRksf4wJvpMPuQwHXuAfmsD3by7fHTvxMqsC5PEPux95NWJm2jydT",
    ];
    pub const INVALIDS: &[&str] = &[
        "函 布 語 厚 象 導 屆 憲 旅 亮 意 疊 雪 凡 第 棚 臘 床",
        "jardim capricho expandir gingado assinar berlinda pancada gaveta minerar gata arame vitral jararaca lustre apertada vantagem surreal injetar",
        "식물 발레 법원 현지 컴퓨터 전체 과목 왕비 권투 권리 원장 흉내 꼭대기 물가 난방 대문 장차 보편적 적당히 스물 기숙사 조선 왼쪽",
        "meditar errante faticare venato strozzare rizoma asettico pari bava battuto petulante zelo calesse ecco campale conferma rifugio fisico ripresa luminoso briglia sbancato pedalare misurare",
    ];
}
