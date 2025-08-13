const CHINESE_SIMPLIFIED: &str = include_str!("raw/chinese_simplified.txt");
const CHINESE_TRADITIONAL: &str = include_str!("raw/chinese_traditional.txt");
const CZECH: &str = include_str!("raw/czech.txt");
const ENGLISH: &str = include_str!("raw/english.txt");
const FRENCH: &str = include_str!("raw/french.txt");
const ITALIAN: &str = include_str!("raw/italian.txt");
const JAPANESE: &str = include_str!("raw/japanese.txt");
const KOREAN: &str = include_str!("raw/korean.txt");
const PORTUGUESE: &str = include_str!("raw/portuguese.txt");
const SPANISH: &str = include_str!("raw/spanish.txt");

/// BIP39 languages
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    /// Chinese Simplified
    ChineseSimplified = 4,
    /// Chinese Traditional
    ChineseTraditional = 5,
    /// Czech
    Czech = 8,
    /// English
    #[default]
    English = 0,
    /// French
    French = 6,
    /// Italian
    Italian = 7,
    /// Japanese
    Japanese = 1,
    /// Korean
    Korean = 2,
    /// Portuguese
    Portuguese = 9,
    /// Spanish
    Spanish = 3,
}

impl Language {
    /// All bip39 languages
    ///   Ordered by BIP85 specification
    pub fn all() -> [Language; 10] {
        [
            Self::English,
            Self::Japanese,
            Self::Korean,
            Self::Spanish,
            Self::ChineseSimplified,
            Self::ChineseTraditional,
            Self::French,
            Self::Italian,
            Self::Czech,
            Self::Portuguese,
        ]
    }

    /// Language's words list
    pub fn wordlist(&self) -> impl Iterator<Item = &str> {
        match self {
            Self::ChineseSimplified => CHINESE_SIMPLIFIED.split_whitespace(),
            Self::ChineseTraditional => CHINESE_TRADITIONAL.split_whitespace(),
            Self::Czech => CZECH.split_whitespace(),
            Self::English => ENGLISH.split_whitespace(),
            Self::French => FRENCH.split_whitespace(),
            Self::Italian => ITALIAN.split_whitespace(),
            Self::Japanese => JAPANESE.split_whitespace(),
            Self::Korean => KOREAN.split_whitespace(),
            Self::Portuguese => PORTUGUESE.split_whitespace(),
            Self::Spanish => SPANISH.split_whitespace(),
        }
    }

    /// Get mnemonic word at index  
    pub fn word_at(&self, index: usize) -> Option<&str> {
        if index < 2048 {
            Some(self.wordlist().nth(index).unwrap())
        } else {
            None
        }
    }

    /// Get mnemonic word index  
    pub fn index_of(&self, word: &str) -> Option<usize> {
        self.wordlist().position(|w| w == word)
    }

    /// Detect word language
    pub fn detect(word: &str) -> Vec<Language> {
        use super::Language::*;
        if let Some(ch) = word.chars().next() {
            let langs = match ch as u32 {
                0x1100..=0x11ff => vec![Korean],
                0x3040..=0x309f => vec![Japanese],
                0x4e00..=0x9f9f => vec![ChineseSimplified, ChineseTraditional],
                _ => match word.is_ascii() {
                    true => vec![English, Italian, Czech, Portuguese, French, Spanish],
                    false => vec![French, Spanish],
                },
            };
            return langs
                .into_iter()
                .filter(|lang| lang.index_of(word).is_some())
                .collect();
        }
        vec![]
    }
}

impl std::str::FromStr for Language {
    type Err = super::Bip39Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "chinese" => Ok(Self::ChineseSimplified),
            "chinesesimplified" => Ok(Self::ChineseSimplified),
            "chinesetraditional" => Ok(Self::ChineseTraditional),
            "czech" => Ok(Self::Czech),
            "english" => Ok(Self::English),
            "french" => Ok(Self::French),
            "italian" => Ok(Self::Italian),
            "japanese" => Ok(Self::Japanese),
            "korean" => Ok(Self::Korean),
            "portuguese" => Ok(Self::Portuguese),
            "spanish" => Ok(Self::Spanish),
            _ => Err(super::Bip39Error::LanguageError),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_length() {
        // words count
        for lang in Language::all() {
            assert_eq!(lang.wordlist().count(), 2048, "{lang:?}");
        }
    }
}
