use std::{fmt, marker::PhantomData, str::FromStr};

/// Provides methods on `&str` for parsing.
///
/// Everything is expected to succeed, so many of this trait's methods will panic on failure.
///
/// `T: FromStrUnwrap` requires `T: FromStr, <T as FromStr>::Err: Debug`.
pub trait Parse {
    fn parse_uw<T: FromStrUnwrap>(&self) -> T;
    fn idx(&self, index: usize) -> u8;
    fn ints_iter<T: FromStrUnwrap>(&self) -> Ints<T>;
    fn ints<const N: usize, T: FromStrUnwrap>(&self) -> [T; N];
    fn uints_iter<T: FromStrUnwrap>(&self) -> UInts<T>;
    fn uints<const N: usize, T: FromStrUnwrap>(&self) -> [T; N];
    fn try_between(&self, pre: &str, post: &str) -> Option<&str>;
    // fn try_between_many(&self, pre: &str, post: &[&str]) -> Option<&str>;
    fn as_parser(&self) -> Parser;
}

impl Parse for str {
    /// Short for `.parse::<T>().unwrap()`.
    ///
    /// Requires that `T: FromStr` and `<T as FromStr>::Err: Debug`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "-205";
    ///
    /// assert_eq!(s.parse_uw::<i32>(), -205);
    /// ```
    #[inline]
    #[track_caller]
    fn parse_uw<T: FromStrUnwrap>(&self) -> T {
        T::parse(self)
    }

    /// Returns the byte at the given index of `self`.
    ///
    /// Useful when `self` is an ASCII string slice.
    ///
    /// Panics when the index is at least the length of `self`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "hello, world!";
    ///
    /// assert_eq!(s.idx(0), b'h');
    /// assert_eq!(s.idx(7), b'w');
    /// ```
    #[inline]
    #[track_caller]
    fn idx(&self, index: usize) -> u8 {
        self.as_bytes()[index]
    }

    /// Returns an iterator over the signed integers in `self`, parsed into type `T`.
    ///
    /// Examples of signed integers include `"1"`, `"-2"` and `"+3"`, but not `"++4"`, `"-+5"` or `"--6"`.
    /// In the latter cases, all but the last sign will be ignored.
    ///
    /// `T` should generally be a signed integer type like `i32`. `T: FromStr` and `<T as FromStr>::Err: Debug` are required.
    ///
    /// The returned iterator will panic if it fails to parse an integer into `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "some signed integers: 15, -302 and +45.";
    /// let mut ints = s.ints_iter::<i32>();
    ///
    /// assert_eq!(ints.next(), Some(15));
    /// assert_eq!(ints.next(), Some(-302));
    /// assert_eq!(ints.next(), Some(45));
    /// assert_eq!(ints.next(), None);
    /// ```
    fn ints_iter<T: FromStrUnwrap>(&self) -> Ints<T> {
        Ints {
            s: self,
            _phantom: PhantomData,
        }
    }

    /// Returns an array of the first `N` signed integers in `self`, parsed into type `T`.
    ///
    /// Short for `.ints_iter::<T>().collect_n::<N>()`.
    ///
    /// Examples of signed integers include `"1"`, `"-2"` and `"+3"`, but not `"++4"`, `"-+5"` or `"--6"`.
    /// In the latter cases, all but the last sign will be ignored.
    ///
    /// `T` should generally be a signed integer type like `i32`. `T: FromStr` and `<T as FromStr>::Err: Debug` are required.
    ///
    /// Panics if the iterator yields less than `N` items, or if it fails to parse an integer into `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "some signed integers: 15, -302 and +45.";
    ///
    /// assert_eq!(s.ints::<3, i32>(), [15, -302, 45]);
    /// ```
    #[inline]
    #[track_caller]
    fn ints<const N: usize, T: FromStrUnwrap>(&self) -> [T; N] {
        self.ints_iter().collect_n()
    }

    /// Returns an iterator over the unsigned integers in `self`, parsed into `T`.
    ///
    /// Examples of unsigned integers include `"1"` and `"2"`, but not `"-3"` or `"+4"`.
    /// In the latter cases, the signs will be ignored.
    ///
    /// `T` should generally be an integer type like `u32`. `T: FromStr` and `<T as FromStr>::Err: Debug` are required.
    ///
    /// The returned iterator will panic if it fails to parse an integer into `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "some unsigned integers: 15, 302 and 45.";
    /// let mut ints = s.uints_iter::<u32>();
    ///
    /// assert_eq!(ints.next(), Some(15));
    /// assert_eq!(ints.next(), Some(302));
    /// assert_eq!(ints.next(), Some(45));
    /// assert_eq!(ints.next(), None);
    /// ```
    fn uints_iter<T: FromStrUnwrap>(&self) -> UInts<T> {
        UInts {
            s: self,
            _phantom: PhantomData,
        }
    }

    /// Returns an array of the first `N` unsigned integers in `self`, parsed into `T`.
    ///
    /// Short for `.uints_iter::<T>().collect_n::<N>()`.
    ///
    /// Examples of unsigned integers include `"1"` and `"2"`, but not `"-3"` or `"+4"`.
    /// In the latter cases, the signs will be ignored.
    ///
    /// `T` should generally be an integer type like `u32`. `T: FromStr` and `<T as FromStr>::Err: Debug` are required.
    ///
    /// Panics if the iterator yields less than `N` items, or if it fails to parse an integer into `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "some unsigned integers: 15, 302 and 45.";
    ///
    /// assert_eq!(s.uints::<3, u32>(), [15, 302, 45]);
    /// ```
    #[inline]
    #[track_caller]
    fn uints<const N: usize, T: FromStrUnwrap>(&self) -> [T; N] {
        self.uints_iter().collect_n()
    }

    /// Returns the string slice between `pre` and `post` in `self`.
    ///
    /// More specifically, finds the first occurrence of `pre` in `self`, or returns `None` if it does not occur.
    /// Then, finds the first occurrence of `post` after that, and returns the string slice between the two.
    /// If `post` does not occur after `pre`, returns the string slice starting after `pre` until the end of `self`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "ecl:gry pid:860033327 eyr:2020";
    ///
    /// assert_eq!(s.try_between("ecl:", " "), Some("gry"));
    /// assert_eq!(s.try_between("pid:", " "), Some("860033327"));
    /// assert_eq!(s.try_between("eyr:", " "), Some("2020"));
    /// assert_eq!(s.try_between("cid:", " "), None);
    /// ```
    fn try_between(&self, pre: &str, post: &str) -> Option<&str> {
        let start = self.find(pre)? + pre.len();
        let rest = &self[start..];
        let end = rest.find(post).unwrap_or(rest.len()) + start;
        Some(&self[start..end])
    }

    // /// Returns the string slice between `pre` and the first occurrence of an element of `post` in `self`.
    // ///
    // /// More specifically, finds the first occurrence of `pre` in `self`, or returns `None` if it does not occur.
    // /// Then, finds the first occurrence of each element of `post` after that, chooses the one that occurs first in `self`,
    // /// and returns the string slice between `pre` and the chosen element of `post`.
    // /// If no elements of `post` occur after `pre`, or if `pre` is empty, returns the string slice starting after `pre` until the end of `self`.
    // ///
    // /// If `pre` has only one element, this works the same as `Parse::between()`.
    // ///
    // /// # Examples
    // /// ```
    // /// use aoc::Parse;
    // ///
    // /// let s = "ecl:gry pid:860033327,eyr:2020";
    // ///
    // /// assert_eq!(s.try_between_many("ecl:", &[" ", ","]), Some("gry"));
    // /// assert_eq!(s.try_between_many("pid:", &[" ", ","]), Some("860033327"));
    // /// assert_eq!(s.try_between_many("eyr:", &[" ", ","]), Some("2020"));
    // /// assert_eq!(s.try_between_many("cid:", &[" ", ","]), None);
    // /// ```
    // fn try_between_many(&self, pre: &str, post: &[&str]) -> Option<&str> {
    //     let start = self.find(pre)? + pre.len();
    //     let rest = &self[start..];
    //     let mut end = self.len();
    //     for &post in post {
    //         if let Some(i) = rest.find(post) {
    //             end = end.min(i + start);
    //         }
    //     }
    //     Some(&self[start..end])
    // }

    /// Returns a struct for gradually parsing data from `self` from left to right.
    ///
    /// Each time a method is called on the struct, the processed portion of the string is "consumed",
    /// and future method calls will only consider the remainder of the string.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "move 10 from 271 to 3";
    /// let mut parser = s.as_parser();
    ///
    /// assert_eq!(parser.between(" ", " "), "10");
    /// assert_eq!(parser.between(" ", " "), "271");
    /// assert_eq!(parser.after(" "), "3");
    /// ```
    /// Another way to do the same thing is:
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "move 10 from 271 to 3";
    /// let mut parser = s.as_parser().skip(5);
    ///
    /// assert_eq!(parser.before(" "), "10");
    /// parser.skip(5);
    /// assert_eq!(parser.before(" "), "271");
    /// parser.skip(3);
    /// assert_eq!(parser.rest(), "3");
    /// ```
    /// Or alternatively:
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "move 10 from 271 to 3";
    /// let mut parser = s.as_parser();
    ///
    /// assert_eq!(parser.between("move ", " "), "10");
    /// assert_eq!(parser.between("from ", " "), "271");
    /// assert_eq!(parser.after("to "), "3");
    /// ```
    #[inline]
    fn as_parser(&self) -> Parser {
        Parser::new(self)
    }
}

impl<S> Parse for S
where
    S: AsRef<str>,
{
    /// Short for `.parse::<T>().unwrap()`.
    ///
    /// Requires that `T: FromStr` and `<T as FromStr>::Err: Debug`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "-205";
    ///
    /// assert_eq!(s.parse_uw::<i32>(), -205);
    /// ```
    fn parse_uw<T: FromStrUnwrap>(&self) -> T {
        self.as_ref().parse_uw()
    }

    /// Returns the byte at the given index of `self`.
    ///
    /// Useful when `self` is an ASCII string slice.
    ///
    /// Panics when the index is at least the length of `self`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "hello, world!";
    ///
    /// assert_eq!(s.idx(0), b'h');
    /// assert_eq!(s.idx(7), b'w');
    /// ```
    fn idx(&self, index: usize) -> u8 {
        self.as_ref().idx(index)
    }

    /// Returns an iterator over the signed integers in `self`, parsed into type `T`.
    ///
    /// Examples of signed integers include `"1"`, `"-2"` and `"+3"`, but not `"++4"`, `"-+5"` or `"--6"`.
    /// In the latter cases, all but the last sign will be ignored.
    ///
    /// `T` should generally be a signed integer type like `i32`. `T: FromStr` and `<T as FromStr>::Err: Debug` are required.
    ///
    /// The returned iterator will panic if it fails to parse an integer into `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "some signed integers: 15, -302 and +45.";
    /// let mut ints = s.ints_iter::<i32>();
    ///
    /// assert_eq!(ints.next(), Some(15));
    /// assert_eq!(ints.next(), Some(-302));
    /// assert_eq!(ints.next(), Some(45));
    /// assert_eq!(ints.next(), None);
    /// ```
    fn ints_iter<T: FromStrUnwrap>(&self) -> Ints<T> {
        self.as_ref().ints_iter()
    }

    /// Returns an array of the first `N` signed integers in `self`, parsed into type `T`.
    ///
    /// Short for `.ints_iter::<T>().collect_n::<N>()`.
    ///
    /// Examples of signed integers include `"1"`, `"-2"` and `"+3"`, but not `"++4"`, `"-+5"` or `"--6"`.
    /// In the latter cases, all but the last sign will be ignored.
    ///
    /// `T` should generally be a signed integer type like `i32`. `T: FromStr` and `<T as FromStr>::Err: Debug` are required.
    ///
    /// Panics if the iterator yields less than `N` items, or if it fails to parse an integer into `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "some signed integers: 15, -302 and +45.";
    ///
    /// assert_eq!(s.ints::<3, i32>(), [15, -302, 45]);
    /// ```
    fn ints<const N: usize, T: FromStrUnwrap>(&self) -> [T; N] {
        self.as_ref().ints()
    }

    /// Returns an iterator over the unsigned integers in `self`, parsed into `T`.
    ///
    /// Examples of unsigned integers include `"1"` and `"2"`, but not `"-3"` or `"+4"`.
    /// In the latter cases, the signs will be ignored.
    ///
    /// `T` should generally be an integer type like `u32`. `T: FromStr` and `<T as FromStr>::Err: Debug` are required.
    ///
    /// The returned iterator will panic if it fails to parse an integer into `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "some unsigned integers: 15, 302 and 45.";
    /// let mut ints = s.uints_iter::<u32>();
    ///
    /// assert_eq!(ints.next(), Some(15));
    /// assert_eq!(ints.next(), Some(302));
    /// assert_eq!(ints.next(), Some(45));
    /// assert_eq!(ints.next(), None);
    /// ```
    fn uints_iter<T: FromStrUnwrap>(&self) -> UInts<T> {
        self.as_ref().uints_iter()
    }

    /// Returns an array of the first `N` unsigned integers in `self`, parsed into `T`.
    ///
    /// Short for `.uints_iter::<T>().collect_n::<N>()`.
    ///
    /// Examples of unsigned integers include `"1"` and `"2"`, but not `"-3"` or `"+4"`.
    /// In the latter cases, the signs will be ignored.
    ///
    /// `T` should generally be an integer type like `u32`. `T: FromStr` and `<T as FromStr>::Err: Debug` are required.
    ///
    /// Panics if the iterator yields less than `N` items, or if it fails to parse an integer into `T`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "some unsigned integers: 15, 302 and 45.";
    ///
    /// assert_eq!(s.uints::<3, u32>(), [15, 302, 45]);
    /// ```
    fn uints<const N: usize, T: FromStrUnwrap>(&self) -> [T; N] {
        self.as_ref().uints()
    }

    /// Returns the string slice between `pre` and `post` in `self`.
    ///
    /// More specifically, finds the first occurrence of `pre` in `self`, or returns `None` if it does not occur.
    /// Then, finds the first occurrence of `post` after that, and returns the string slice between the two.
    /// If `post` does not occur after `pre`, returns the string slice starting after `pre` until the end of `self`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "ecl:gry pid:860033327 eyr:2020";
    ///
    /// assert_eq!(s.try_between("ecl:", " "), Some("gry"));
    /// assert_eq!(s.try_between("pid:", " "), Some("860033327"));
    /// assert_eq!(s.try_between("eyr:", " "), Some("2020"));
    /// assert_eq!(s.try_between("cid:", " "), None);
    /// ```
    fn try_between(&self, pre: &str, post: &str) -> Option<&str> {
        self.as_ref().try_between(pre, post)
    }

    /// Returns a struct for gradually parsing data from `self` from left to right.
    ///
    /// Each time a method is called on the struct, the processed portion of the string is "consumed",
    /// and future method calls will only consider the remainder of the string.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "move 10 from 271 to 3";
    /// let mut parser = s.as_parser();
    ///
    /// assert_eq!(parser.between(" ", " "), "10");
    /// assert_eq!(parser.between(" ", " "), "271");
    /// assert_eq!(parser.after(" "), "3");
    /// ```
    /// Another way to do the same thing is:
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "move 10 from 271 to 3";
    /// let mut parser = s.as_parser().skip(5);
    ///
    /// assert_eq!(parser.before(" "), "10");
    /// parser.skip(5);
    /// assert_eq!(parser.before(" "), "271");
    /// parser.skip(3);
    /// assert_eq!(parser.rest(), "3");
    /// ```
    /// Or alternatively:
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "move 10 from 271 to 3";
    /// let mut parser = s.as_parser();
    ///
    /// assert_eq!(parser.between("move ", " "), "10");
    /// assert_eq!(parser.between("from ", " "), "271");
    /// assert_eq!(parser.after("to "), "3");
    /// ```
    fn as_parser(&self) -> Parser {
        self.as_ref().as_parser()
    }
}

pub trait FromStrUnwrap {
    fn parse(s: &str) -> Self;
}

impl<T> FromStrUnwrap for T
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    #[inline(always)]
    #[track_caller]
    fn parse(s: &str) -> Self {
        s.parse().unwrap()
    }
}

/// An iterator over the signed integers in a `&str`.
///
/// Panics if it fails to parse an integer into `T`.
#[derive(Clone, Copy, Debug)]
pub struct Ints<'a, T> {
    s: &'a str,
    _phantom: PhantomData<T>,
}

impl<'a, T: FromStrUnwrap> Iterator for Ints<'a, T> {
    type Item = T;

    #[track_caller]
    fn next(&mut self) -> Option<Self::Item> {
        fn is_digit_or_sign(ch: u8) -> bool {
            ch.is_ascii_digit() || ch == b'-' || ch == b'+'
        }

        let (s, mut i) = (self.s, 0);
        loop {
            while i < s.len() && !is_digit_or_sign(s.idx(i)) {
                i += 1;
            }
            if i >= s.len() {
                return None;
            }
            let mut j = i + 1;
            while j < s.len() && s.idx(j).is_ascii_digit() {
                j += 1;
            }
            if !s.idx(j - 1).is_ascii_digit() {
                i = j;
                continue;
            }
            self.s = &s[j..];
            return Some(s[i..j].parse_uw());
        }
    }
}

/// An iterator over the unsigned integers in a `&str`.
///
/// Panics if it fails to parse an integer into `T`.
#[derive(Clone, Copy, Debug)]
pub struct UInts<'a, T> {
    s: &'a str,
    _phantom: PhantomData<T>,
}

impl<'a, T: FromStrUnwrap> Iterator for UInts<'a, T> {
    type Item = T;

    #[track_caller]
    fn next(&mut self) -> Option<Self::Item> {
        let (s, mut i) = (self.s, 0);
        while i < s.len() && !s.idx(i).is_ascii_digit() {
            i += 1;
        }
        if i >= s.len() {
            return None;
        }
        let mut j = i + 1;
        while j < s.len() && s.idx(j).is_ascii_digit() {
            j += 1;
        }
        self.s = &s[j..];
        Some(s[i..j].parse_uw())
    }
}

/// Provides methods on iterators to reduce allocations and `.unwrap()` calls when success is assumed.
pub trait IterUnwrap {
    type Item;

    fn next_uw(&mut self) -> Self::Item;
    fn collect_n<const N: usize>(&mut self) -> [Self::Item; N];
}

impl<I> IterUnwrap for I
where
    I: Iterator,
{
    type Item = <I as Iterator>::Item;

    /// Short for `.next().unwrap()`.
    ///
    /// # Examples
    /// ```
    /// use aoc::IterUnwrap;
    ///
    /// let mut iter = [1, 2, 3].into_iter();
    ///
    /// assert_eq!(iter.next_uw(), 1);
    /// assert_eq!(iter.next_uw(), 2);
    /// assert_eq!(iter.next_uw(), 3);
    /// ```
    #[inline]
    #[track_caller]
    fn next_uw(&mut self) -> Self::Item {
        self.next().unwrap()
    }

    /// Collects the next `N` items yielded by the iterator into an array.
    ///
    /// Panics if the iterator yields less than `N` items.
    ///
    /// # Examples
    /// ```
    /// use aoc::IterUnwrap;
    ///
    /// assert_eq!("hello, world!".chars().collect_n::<5>(), ['h', 'e', 'l', 'l', 'o']);
    /// ```
    #[track_caller]
    fn collect_n<const N: usize>(&mut self) -> [Self::Item; N] {
        let arr = [(); N].map(|_| self.next());
        for res in &arr {
            if res.is_none() {
                panic!("not enough elements in the iterator to fill the size `N` array")
            }
        }
        arr.map(|x| x.unwrap())
    }
}

/// A struct for gradually parsing data from a string from left to right.
///
/// Each time a method is called, the processed portion of the string is "consumed",
/// and future method calls will only consider the remainder of the string.
///
/// # Examples
/// ```
/// use aoc::Parse;
///
/// let s = "move 10 from 271 to 3";
/// let mut parser = s.as_parser();
///
/// assert_eq!(parser.between(" ", " "), "10");
/// assert_eq!(parser.between(" ", " "), "271");
/// assert_eq!(parser.after(" "), "3");
/// ```
/// Another way to do the same thing is:
/// ```
/// use aoc::Parse;
///
/// let s = "move 10 from 271 to 3";
/// let mut parser = s.as_parser().skip(5);
///
/// assert_eq!(parser.before(" "), "10");
/// parser.skip(5);
/// assert_eq!(parser.before(" "), "271");
/// parser.skip(3);
/// assert_eq!(parser.rest(), "3");
/// ```
/// Or alternatively:
/// ```
/// use aoc::Parse;
///
/// let s = "move 10 from 271 to 3";
/// let mut parser = s.as_parser();
///
/// assert_eq!(parser.between("move ", " "), "10");
/// assert_eq!(parser.between("from ", " "), "271");
/// assert_eq!(parser.after("to "), "3");
/// ```
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    inner: &'a str,
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` from the given `&str`.
    #[inline]
    pub fn new(s: &'a str) -> Self {
        Self { inner: s }
    }

    /// Skips past the next `n` bytes (ASCII characters) of the string.
    ///
    /// Future method calls on `self` will work on the remainder of the string.
    ///
    /// Both mutates `self` and returns a copy of `self` after the mutation.
    ///
    /// Panics if the string has less than `n` bytes left.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "12345foo1234567bar";
    /// let mut parser = s.as_parser().skip(5);
    ///
    /// assert_eq!(parser.take(3), "foo");
    /// parser.skip(7);
    /// assert_eq!(parser.rest(), "bar");
    /// ```
    #[inline]
    #[track_caller]
    pub fn skip(&mut self, n: usize) -> Self {
        self.inner = &self.inner[n..];
        self.clone()
    }

    /// Returns the next `n` bytes (ASCII characters) of the string.
    ///
    /// Future method calls on `self` will then work on the remainder of the string.
    ///
    /// Panics if the string has less than `n` bytes left.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "12345foo1234567bar.";
    /// let mut parser = s.as_parser().skip(5);
    ///
    /// assert_eq!(parser.take(3), "foo");
    /// parser.skip(7);
    /// assert_eq!(parser.take(3), "bar");
    /// ```
    #[track_caller]
    pub fn take(&mut self, n: usize) -> &str {
        let first = &self.inner[..n];
        self.inner = &self.inner[n..];
        first
    }

    /// Returns the remainder of the string, consuming `self`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "hello, world!";
    /// let mut parser = s.as_parser();
    ///
    /// parser.skip(7);
    /// assert_eq!(parser.rest(), "world!");
    /// ```
    #[inline]
    pub fn rest(self) -> &'a str {
        self.inner
    }

    /// Returns the slice of the string before the first occurrence of `suffix`.
    ///
    /// Future method calls on `self` will then work on the remainder of the string after `suffix`.
    ///
    /// Panics if `suffix` is not contained in the remainder of the string.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "move 10 from 271 to 3";
    /// let mut parser = s.as_parser().skip(5);
    ///
    /// assert_eq!(parser.before(" "), "10");
    /// parser.skip(5);
    /// assert_eq!(parser.before(" "), "271");
    /// parser.skip(3);
    /// assert_eq!(parser.rest(), "3");
    /// ```
    #[track_caller]
    pub fn before(&mut self, suffix: &str) -> &'a str {
        let (before, after) = self
            .inner
            .split_once(suffix)
            .expect("`suffix` should be contained in the string");
        self.inner = after;
        before
    }

    /// Returns the slice of the string after the first occurrence of `prefix`, consuming `self`.
    ///
    /// Panics if `prefix` is not contained in the remainder of the string.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "move 10 from 271 to 3";
    /// let mut parser = s.as_parser();
    ///
    /// assert_eq!(parser.between(" ", " "), "10");
    /// assert_eq!(parser.between(" ", " "), "271");
    /// assert_eq!(parser.after(" "), "3");
    /// ```
    #[track_caller]
    pub fn after(self, prefix: &str) -> &'a str {
        let i = self
            .inner
            .find(prefix)
            .expect("`prefix` should be contained in the string")
            + prefix.len();
        &self.inner[i..]
    }

    /// Returns the slice of the string after the first occurrence of `prefix`, and before the next occurrence of `suffix`.
    ///
    /// Future method calls on `self` will then work on the remainder of the string after `suffix`.
    ///
    /// # Examples
    /// ```
    /// use aoc::Parse;
    ///
    /// let s = "move 10 from 271 to 3";
    /// let mut parser = s.as_parser();
    ///
    /// assert_eq!(parser.between("move ", " "), "10");
    /// assert_eq!(parser.between("from ", " "), "271");
    /// assert_eq!(parser.after("to "), "3");
    /// ```
    #[track_caller]
    pub fn between(&mut self, prefix: &str, suffix: &str) -> &'a str {
        *self = Self {
            inner: self.clone().after(prefix),
        };
        self.before(suffix)
    }
}
