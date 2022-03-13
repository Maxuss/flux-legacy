
#[macro_export]
macro_rules! convert_nbt_type {
    ({
        $(
            $k:ident: $v:tt
        ),* $(,)*
    }) => {
        $crate::nbt::NbtTag::Compound(nbt!($($k:$v,)*))
    };
    ($var:ident) => {
        $var.into()
    };
    ($lit:literal) => {
        $lit.into()
    };
    ([$($ele:tt),* $(,)*]) => {
        $crate::nbt::NbtTag::List(vec![$($crate::convert_nbt_type!($ele),)*])
    };
    ([I;$($ele:tt),* $(,)*]) => {
        $crate::nbt::NbtTag::IntArray(vec![$($ele,)*])
    };
    ([L;$($ele:tt),* $(,)*]) => {
        $crate::nbt::NbtTag::LongArray(vec![$($ele,)*])
    };
    ([B;$($ele:tt),* $(,)*]) => {
        $crate::nbt::NbtTag::ByteArray(vec![$($ele,)*])
    };
    ($expr:expr) => {
        $expr
    }
}

#[macro_export]
macro_rules! convert_nbt_key {
    ($name:ident) => {
        stringify!($name)
    };
    ($lit:literal) => {
        $lit.to_string()
    };
    ([$var:ident]) => {
        $var.to_string()
    }
}

#[macro_export]
macro_rules! nbt {
    (
        $(
        $k:tt: $v:tt
        ),* $(,)*
    ) => {
        Compound::new(HashMap::<String, NbtTag>::from([
            $(
            ($crate::convert_nbt_key!($k).into(), $crate::convert_nbt_type!($v)),
            )*
        ]))
    };
}