// known-bug: #113333
// edition: 2021

#![feature(c_str_literals)]

fn main() {
    c"\0";
    // FIXME(c_str_literals): This should be `null characters in C string`

    c"\u{00}";
    // FIXME(c_str_literals): This should be `null characters in C string`

    c" ";
    // FIXME(c_str_literals): This should be `null characters in C string`

    c"\x00";
    // FIXME(c_str_literals): This should be `null characters in C string`

    cr" ";
    // FIXME(c_str_literals): This should be `null characters in C string`
}

macro_rules! empty {
    ($($tt:tt)*) => {};
}

#[cfg(FALSE)]
fn test() {
    c"\0";
    c"\u{00}";
    c" ";
    c"\x00";
    cr" ";

    empty!(c"\0");
    empty!(c"\u{00}");
    empty!(c" ");
    empty!(c"\x00");
    empty!(cr" ");
}
