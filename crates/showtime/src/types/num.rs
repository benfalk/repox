macro_rules! non_zero {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(::std::num::NonZero<$type>);

        impl $name {
            #[inline]
            pub fn as_number(&self) -> $type {
                self.0.get()
            }
        }

        impl TryFrom<i64> for $name {
            type Error = ::anyhow::Error;

            #[inline]
            fn try_from(value: i64) -> Result<Self, Self::Error> {
                use ::anyhow::Context as _;
                let non_zero = ::std::num::NonZero::new(value.try_into()?)
                    .context("id must be non-zero")?;
                Ok(Self(non_zero))
            }
        }
    };
}
pub(crate) use non_zero;
