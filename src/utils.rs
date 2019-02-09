//! Utils macros

#[macro_export]
macro_rules! expect {
    ($src:ident, $index:expr, $expect:tt) => {
        $src.as_bytes().get($index).filter(|&&b| b == $expect)
    };
    ($src:ident, $index:expr, $expect:expr) => {
        $src.as_bytes().get($index).filter(|&&b| $expect(b))
    };
}

#[macro_export]
macro_rules! skip_space {
    ($src:ident) => {
        $src.as_bytes()
            .iter()
            .position(|c| c != b' ' && c != b'\t')
            .unwrap_or(0)
    };
    ($src:ident, $from:expr) => {
        $src[$from..]
            .as_bytes()
            .iter()
            .position(|&c| c != b' ' && c != b'\t')
            .map(|i| i + $from)
            .unwrap_or(0)
    };
}
