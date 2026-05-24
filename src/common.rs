macro_rules! no_fmt {
    ($($beautiful_code:tt)*) => { $($beautiful_code)* }
}

pub(crate) use no_fmt;

#[expect(clippy::match_bool)]
pub(crate) const fn not(value: bool) -> bool {
    match value {
        true => false,
        false => true,
    }
}
