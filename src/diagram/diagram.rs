use super::{ComplexDiagram, GenericDiagram, Result, SimpleDiagram};
use bitcoin::bip32::Xpriv;
use std::ops::Not;

/// Diagram
pub trait Diagram {
    /// Simple diagram (7 * 7 unicode chars) generate master key
    fn art_simple_master(&self, salt: &str) -> Result<Xpriv>;

    /// Complex diagram (7 * 7 unicode strings) generate master key
    fn art_complex_master(&self, salt: &str) -> Result<Xpriv>;
}

impl Diagram for [&str] {
    fn art_simple_master(&self, salt: &str) -> Result<Xpriv> {
        let mut mx = [[None; 7]; 7];
        self.iter().take(7 * 7).enumerate().for_each(|(i, &s)| {
            mx[i / 7][i % 7] = s.chars().next();
        });
        SimpleDiagram(mx).bip32_master(salt.as_bytes())
    }

    fn art_complex_master(&self, salt: &str) -> Result<Xpriv> {
        let mut mx = std::array::from_fn(|_| std::array::from_fn(|_| None));
        self.iter().take(7 * 7).enumerate().for_each(|(i, &s)| {
            mx[i / 7][i % 7] = s.is_empty().not().then_some(s.to_string());
        });
        ComplexDiagram(mx).bip32_master(salt.as_bytes())
    }
}

#[cfg(test)]
#[cfg(not(feature = "testnet"))]
mod diagram_test {
    use super::{Diagram, Result};

    #[test]
    fn test_simple_master() -> Result<()> {
        let vs = [
            ["", "", "", "", "", "", "A"],
            ["", "&", "", "*", "", "", ""],
            [""; 7],
            [""; 7],
            ["", "", "王", "", "", "", ""],
            [""; 7],
            ["", "", "", "", "", "", "😊"],
        ]
        .concat();

        let master = "xprv9s21ZrQH143K2r6v9GGWezApYmVuaGiZYoCpsQFVe9Vwh47yZ2CCgqXJY6g2Kk8Ajrz2PbVNnY5HLw4dPkshmcqX8YBEhcwj4wWQ8UgY5m7";
        assert_eq!(vs.art_simple_master("")?.to_string(), master);

        let master = "xprv9s21ZrQH143K24YcrGNZP2JuYQF4k6M84y2ujCZNF6A227MiXsW5aw3gaY34wn4jA9X8mg39N3WLqF66fxLnUTed42kTGq1dKy41GWQ5QgG";
        assert_eq!(vs.art_simple_master("Thanks Satoshi!")?.to_string(), master);

        Ok(())
    }

    #[test]
    fn test_complex_master() -> Result<()> {
        let vs = [
            ["", "", "", "", "", "", "ABC"],
            ["", "混A1", "", "123", "", "", ""],
            [""; 7],
            [""; 7],
            ["", "", "测试", "", "", "", ""],
            [""; 7],
            ["A&*王😊", "", "", "", "", "", ""],
        ]
        .concat();

        let master = "xprv9s21ZrQH143K317qvL2ScVgHuhJenVLEmA7af5mTWDXjsDbUDSxFxZc5QYkCbaqmbycAhnHhtqJXNwLGt4eeC6VuiDfSUEgrJijAuLJks8X";
        assert_eq!(vs.art_complex_master("")?.to_string(), master);

        let master = "xprv9s21ZrQH143K32BBNz2hduzSS7p8q18MtvDzyGvHFKvMfLRKaS7Bk27BhbMb47X5qeBpEmSiFtsbRv9Zw6QoMDbTEyNo1BU5Qka1PQvAZ4u";
        assert_eq!(vs.art_complex_master("123abc")?.to_string(), master);

        Ok(())
    }
}
