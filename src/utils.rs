//! Utils macros

#[macro_export]
macro_rules! expect {
    ($src:ident, $index:expr, $expect:tt) => {
        $src.as_bytes().get($index).filter(|&b| b == &$expect)
    };
    ($src:ident, $index:expr, $expect:expr) => {
        // $src.as_bytes().get($index).filter($expect)
        if $index >= $src.len() || !$expect($src.as_bytes()[$index]) {
            return None;
        }
    };
}

#[macro_export]
macro_rules! eol {
    ($src:expr) => {
        $src.find('\n').unwrap_or($src.len())
    };
    ($src:expr, $from:expr) => {
        $src[$from..]
            .find('\n')
            .map(|i| i + $from)
            .unwrap_or($src.len())
    };
}

#[macro_export]
macro_rules! until {
    ($src:expr, $until:tt) => {{
        let mut pos = 0;
        while pos < $src.len() {
            if $until == $src.as_bytes()[pos] {
                break;
            }
            pos += 1;
        }
        if pos == $src.len() {
            None
        } else {
            Some(pos)
        }
    }};
    ($src:expr, $until:expr) => {{
        let mut pos = 0;
        while pos < $src.len() {
            if $until($src.as_bytes()[pos]) {
                break;
            }
            pos += 1;
        }
        if pos == $src.len() {
            None
        } else {
            Some(pos)
        }
    }};
}

#[macro_export]
macro_rules! until_while {
    ($src:expr, $start:expr, $until:tt, $while:expr) => {{
        let mut pos = $start;
        while pos < $src.len() {
            // println!("pos {} char {} ", pos, $src.as_bytes()[pos] as char,);
            if $until == $src.as_bytes()[pos] {
                break;
            } else if $while($src.as_bytes()[pos]) {
                pos += 1;
                continue;
            } else {
                return None;
            }
        }
        if pos == $src.len() {
            return None;
        } else {
            pos
        }
    }};
    ($src:expr, $start:expr, $until:expr, $while:expr) => {{
        let mut pos = $start;
        while pos < $src.len() {
            // println!("pos {} char {}", pos, $src.as_bytes()[pos] as char);
            if $until($src.as_bytes()[pos]) {
                break;
            } else if $while($src.as_bytes()[pos]) {
                pos += 1;
                continue;
            } else {
                return None;
            }
        }
        if pos == $src.len() {
            return None;
        } else {
            pos
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
macro_rules! position {
    ($s:ident, $i:expr, $p:expr) => {
        match $s[$i..].chars().position($p) {
            Some(x) => x + $i,
            None => return None,
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
    ($src:ident, $from:expr) => {
        until!($src[$from..], |c| c != b' ').unwrap_or(0) + $from
    };
}

#[macro_export]
macro_rules! skip_empty_line {
    ($src:ident, $from:expr) => {{
        let mut pos = $from;
        while pos < $src.len() {
            if $src.as_bytes()[pos] != b'\n' {
                break;
            }
            pos += 1;
        }
        pos
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
