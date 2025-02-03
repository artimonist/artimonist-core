/// impl Deref for wrapper type struct T(U)
macro_rules! ImpDeref {
    ($t: ty, $o: ty) => {
        impl std::ops::Deref for $t {
            type Target = $o;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl std::ops::DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// impl From for wrapper type struct T(U)
macro_rules! ImpFrom {
    ($t: ty, $o: ty) => {
        impl From<$o> for $t {
            fn from(s: $o) -> Self {
                Self(s)
            }
        }
        impl From<$t> for $o {
            fn from(t: $t) -> Self {
                t.0
            }
        }
    };
}

/// impl Display for wrapper type struct T(U)
macro_rules! ImpDisplay {
    ($t: ty) => {
        impl std::fmt::Display for $t {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

/// impl PartialEq for wrapper type struct T(U)
macro_rules! ImpPartialEq {
    ($t: ty, $o: ty) => {
        impl PartialEq<&$o> for $t {
            #[inline]
            fn eq(&self, other: &&$o) -> bool {
                self.0 == *other
            }
        }
    };
}

pub(crate) use ImpDeref;
pub(crate) use ImpDisplay;
pub(crate) use ImpFrom;
pub(crate) use ImpPartialEq;
