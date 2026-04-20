macro_rules! read_only {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name($type);

        impl ::std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl<T: Into<$type>> From<T> for $name {
            #[inline]
            fn from(value: T) -> Self {
                Self(value.into())
            }
        }

        impl $name {
            #[inline]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}
pub(crate) use read_only;
