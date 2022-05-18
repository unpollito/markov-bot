use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMarkovChain {
    #[serde(deserialize_with = "LaxNumber::deserialize")]
    pub chat_id: i64,
    pub entries: HashMap<String, Vec<ChatMarkovChainSuccessor>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMarkovChainSuccessor {
    pub word: String,
    #[serde(deserialize_with = "LaxNumber::deserialize")]
    pub num_times: u32,
}

// Adapted from https://github.com/mongodb/bson-rust/issues/85
#[allow(non_snake_case)]
pub mod LaxNumber {
    use serde::{Deserialize, Deserializer};

    pub trait LaxFromF64 {
        fn from(v: f64) -> Self;
    }

    macro_rules! impl_laxfrom64 {
        ($to:ident) => {
            impl LaxFromF64 for $to {
                fn from(v: f64) -> $to {
                    v as $to
                }
            }
        };
    }

    impl_laxfrom64!(u32);
    impl_laxfrom64!(i64);

    pub fn deserialize<'de, T, D>(d: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: LaxFromF64,
    {
        f64::deserialize(d).map(T::from)
    }

    pub fn deserialize_nullable<'de, T, D>(d: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: LaxFromF64,
    {
        Option::<f64>::deserialize(d).map(|x| match x {
            None => None,
            Some(v) => Some(T::from(v)),
        })
    }
}
