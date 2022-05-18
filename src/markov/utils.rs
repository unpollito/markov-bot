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
}
