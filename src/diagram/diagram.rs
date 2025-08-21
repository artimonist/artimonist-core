use super::{ComplexDiagram, GenericDiagram, Result, SimpleDiagram};
use bitcoin::bip32::Xpriv;
use std::ops::Not;

/// Diagram
pub trait Diagram {
    /// ComplexDiagram cell chars limit
    const CELL_CHARS_LIMIT: usize = 50;

    /// Simple diagram (7 * 7 unicode chars)
    fn art_simple_diagram(&self) -> Result<SimpleDiagram>;

    /// Complex diagram (7 * 7 unicode strings)
    fn art_complex_diagram(&self) -> Result<ComplexDiagram>;

    /// Simple diagram (7 * 7 unicode chars) generate master key
    fn art_simple_master(&self, salt: &str) -> Result<Xpriv>;

    /// Complex diagram (7 * 7 unicode strings) generate master key
    fn art_complex_master(&self, salt: &str) -> Result<Xpriv>;
}

impl<T, U> Diagram for U
where
    U: Iterator<Item = T> + Clone,
    T: AsRef<str>,
{
    fn art_simple_diagram(&self) -> Result<SimpleDiagram> {
        let mut mx = [[None; 7]; 7];
        self.clone().take(7 * 7).enumerate().for_each(|(i, s)| {
            mx[i / 7][i % 7] = s.as_ref().chars().next();
        });
        Ok(SimpleDiagram(mx))
    }

    fn art_complex_diagram(&self) -> Result<ComplexDiagram> {
        let mut mx = std::array::from_fn(|_| std::array::from_fn(|_| None));
        self.clone().take(7 * 7).enumerate().for_each(|(i, s)| {
            let s: String = s.as_ref().chars().take(Self::CELL_CHARS_LIMIT).collect();
            mx[i / 7][i % 7] = s.is_empty().not().then_some(s);
        });
        Ok(ComplexDiagram(mx))
    }

    #[inline]
    fn art_simple_master(&self, salt: &str) -> Result<Xpriv> {
        self.art_simple_diagram()?.to_master(salt.as_bytes())
    }

    #[inline]
    fn art_complex_master(&self, salt: &str) -> Result<Xpriv> {
        self.art_complex_diagram()?.to_master(salt.as_bytes())
    }
}

impl<T> Diagram for [T]
where
    T: AsRef<str>,
{
    #[inline(always)]
    fn art_simple_diagram(&self) -> Result<SimpleDiagram> {
        self.iter().art_simple_diagram()
    }

    #[inline(always)]
    fn art_complex_diagram(&self) -> Result<ComplexDiagram> {
        self.iter().art_complex_diagram()
    }

    #[inline(always)]
    fn art_simple_master(&self, salt: &str) -> Result<Xpriv> {
        self.iter().art_simple_master(salt)
    }

    #[inline(always)]
    fn art_complex_master(&self, salt: &str) -> Result<Xpriv> {
        self.iter().art_complex_master(salt)
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

        let master = "xprv9s21ZrQH143K38UmyXgTiRxiPAbzzfbvAM1VDkBWKbsRQsmJLxtwLn5DZ5vfhjTumysk6vQHcYje6hcc5h8fa6csha7vRn57oUW1iiJF4wC";
        assert_eq!(vs.art_simple_master("")?.to_string(), master);

        let master = "xprv9s21ZrQH143K3LvvHjFDhRKi4wbNZArr16eEommMwGpnQ1sCgRXJdABhhXDbxptEaALWLQLMzKa5qsBr3Q1LVUqHKwprxGUDihyL72WSt2R";
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

        let master = "xprv9s21ZrQH143K2ztRR2MREiT4aNYQ6GmhoKaCocZkSEqX1edh7TwxDAjddsoAJfgQdtuvCRRGLxpznbSymTiL3umPwefhh1aR5ZQQQRC97TV";
        assert_eq!(vs.art_complex_master("")?.to_string(), master);

        let master = "xprv9s21ZrQH143K2xfe53kNiHqjGC6Jv8zMfjAVhPQZw1nur4CnM79LqpSKZpURNEvyC4xjpPGj37efmviNjkpK6mv9LgRyJBSB8rSK18w6yaY";
        assert_eq!(vs.art_complex_master("123abc")?.to_string(), master);

        Ok(())
    }
}
