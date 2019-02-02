use memchr::{memchr_iter, Memchr};
use std::iter::{once, Chain, Once};

pub struct Lines<'a> {
    src: &'a str,
    iter: Chain<Memchr<'a>, Once<usize>>,
    start: usize,
    pre_cont_end: usize,
}

impl<'a> Lines<'a> {
    pub fn new(src: &'a str) -> Lines<'a> {
        Lines {
            src,
            iter: memchr_iter(b'\n', &src.as_bytes()).chain(once(src.len())),
            start: 0,
            pre_cont_end: 0,
        }
    }
}

impl<'a> Iterator for Lines<'a> {
    type Item = (usize, usize, &'a str);

    #[inline]
    fn next(&mut self) -> Option<(usize, usize, &'a str)> {
        self.iter.next().map(|i| {
            let (line, cont_end) = if i != self.src.len() && self.src.as_bytes()[i - 1] == b'\r' {
                (&self.src[self.start..i - 1], i - 1)
            } else {
                (&self.src[self.start..i], i)
            };
            self.start = if i != self.src.len() { i + 1 } else { i };
            (cont_end, self.start, line)
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[test]
fn lines() {
    let mut lines = Lines::new("foo\r\nbar\n\nbaz\n");

    assert_eq!(Some((3, 5, "foo")), lines.next());
    assert_eq!(Some((8, 9, "bar")), lines.next());
    assert_eq!(Some((9, 10, "")), lines.next());
    assert_eq!(Some((13, 14, "baz")), lines.next());
    assert_eq!(Some((14, 14, "")), lines.next());
    assert_eq!(None, lines.next());
}
