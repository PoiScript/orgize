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
        $src.find('\n').unwrap_or_else(|| $src.len())
    };
    ($src:expr, $from:expr) => {
        $src[$from..]
            .find('\n')
            .map(|i| i + $from)
            .unwrap_or_else(|| $src.len())
    };
}

#[macro_export]
macro_rules! until {
    ($src:expr, $until:tt) => {{
        let mut pos = 0;
        loop {
            if pos >= $src.len() {
                break None;
            }

            if $until == $src.as_bytes()[pos] {
                break Some(pos);
            } else {
                pos += 1;
            }
        }
    }};
    ($src:expr, $until:expr) => {{
        let mut pos = 0;
        loop {
            if pos >= $src.len() {
                break None;
            }

            if $until($src.as_bytes()[pos]) {
                break Some(pos);
            } else {
                pos += 1;
            }
        }
    }};
}

#[macro_export]
macro_rules! until_while {
    ($src:expr, $start:expr, $until:tt, $while:expr) => {{
        let mut pos = $start;
        loop {
            if pos >= $src.len() {
                break None;
            } else if $until == $src.as_bytes()[pos] {
                break Some(pos);
            } else if $while($src.as_bytes()[pos]) {
                pos += 1;
                continue;
            } else {
                break None;
            }
        }
    }};
    ($src:expr, $start:expr, $until:expr, $while:expr) => {{
        let mut pos = $start;
        loop {
            if pos >= $src.len() {
                break None;
            } else if $until($src.as_bytes()[pos]) {
                break Some(pos);
            } else if $while($src.as_bytes()[pos]) {
                pos += 1;
                continue;
            } else {
                break None;
            }
        }
    }};
}

#[macro_export]
macro_rules! cond_eq {
    ($s:ident, $i:expr, $p:expr) => {
        if $i >= $s.len() {
            return None;
        } else {
            $s.as_bytes()[$i] == $p
        }
    };
}

#[macro_export]
macro_rules! starts_with {
    ($s:ident, $p:expr) => {
        if !$s.starts_with($p) {
            return None;
        }
    };
}

#[macro_export]
macro_rules! skip_space {
    ($src:ident) => {
        until!($src, |c| c != b' ' && c != b'\t').unwrap_or(0)
    };
    ($src:ident, $from:expr) => {
        until!($src[$from..], |c| c != b' ' && c != b'\t').unwrap_or(0) + $from
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

#[macro_export]
macro_rules! parse_fail {
    ($ty:ident, $src:expr) => {
        assert_eq!($ty::parse($src), None);
    };
}

#[macro_export]
macro_rules! parse_succ {
    ($ty:ident, $src:expr, $($field:ident : $value:expr),* ) => {
        assert_eq!(
            $ty::parse($src),
            Some((
                $ty {
                    $( $field : $value ),*
                },
                $src.len()
            )),
        );
    };
}

#[macro_export]
macro_rules! lines {
    ($src:ident) => {
        memchr::memchr_iter(b'\n', $src.as_bytes())
            .map(|i| i + 1)
            .chain(std::iter::once($src.len()))
    };
}
