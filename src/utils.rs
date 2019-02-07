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
macro_rules! eol {
    ($src:expr) => {
        memchr::memchr(b'\n', $src.as_bytes()).unwrap_or_else(|| $src.len())
    };
    ($src:expr, $from:expr) => {
        memchr::memchr(b'\n', $src.as_bytes()[$from..])
            .map(|i| i + $from)
            .unwrap_or_else(|| $src.len())
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

#[macro_export]
macro_rules! skip_empty_line {
    ($src:ident, $from:expr) => {{
        let mut pos = $from;
        loop {
            if pos >= $src.len() || $src.as_bytes()[pos] != b'\n' {
                break pos;
            } else {
                pos += 1;
            }
        }
    }};
}
