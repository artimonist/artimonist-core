mod data;
use std::{
    collections::HashMap,
    sync::{LazyLock, OnceLock},
};

use data::*;

/// Mnemonic words language
#[cfg(not(feature = "multilingual"))]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Language {
    /// English
    #[default]
    English = 0,
}

#[cfg(not(feature = "multilingual"))]
const WORD_LIST: [&[&str; 2048]; 1] = [&ENGLISH];

/// Mnemonic language
#[cfg(feature = "multilingual")]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Language {
    /// English
    #[default]
    English = 0,
    /// Japanese
    Japanese = 1,
    /// Korean
    Korean = 2,
    /// Spanish
    Spanish = 3,
    /// SimplifiedChinese
    SimplifiedChinese = 4,
    /// TraditionalChinese
    TraditionalChinese = 5,
    /// Franch
    French = 6,
    /// Italian
    Italian = 7,
    /// Czech
    Czech = 8,
    /// Portuguese
    Portuguese = 9,
}

#[cfg(feature = "multilingual")]
const WORD_LIST: [&[&str; 2048]; 10] = [
    &ENGLISH,
    &JAPANESE,
    &KOREAN,
    &SPANISH,
    &CHINESE_SIMPLIFIED,
    &CHINESE_TRADITIONAL,
    &FRENCH,
    &ITALIAN,
    &CZECH,
    &PORTUGUESE,
];

impl Language {
    /// Get mnemonic word at index  
    ///   0 <= index < 2048  
    /// # Panics  
    ///   index >= 2048  
    #[inline]
    pub fn word_at(&self, index: usize) -> &str {
        WORD_LIST[*self as usize][index]
    }

    /// Get mnemonic word index  
    /// return None if not exists.
    pub fn index_of(&self, word: &str) -> Option<usize> {
        let words = WORD_LIST[*self as usize];
        let dic = WORD_DICS[*self as usize].get_or_init(|| {
            let iter = words.iter().enumerate().map(|(i, &s)| (s, i));
            HashMap::from_iter(iter)
        });
        debug_assert!(dic.len() == 2048);
        dic.get(word).copied()
    }
}

static WORD_DICS: LazyLock<[OnceLock<HashMap<&str, usize>>; 10]> =
    LazyLock::new(|| core::array::from_fn(|_| OnceLock::new()));
