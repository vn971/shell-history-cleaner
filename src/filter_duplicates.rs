#!/bin/sh -eu
#![allow(unused_attribute)] /*
binName="`basename "$0" .rs`"
if [ ! "$binName" -nt "$0" ]; then
    rustc "$0"
fi
exec ./"$binName" "$@"
!# */

use std::os;
use std::collections::HashMap;


fn main() {

    assert!(os::args().tail().head().is_none(),
            "We don't know what to do with command-line arguments. Exiting.");

    let max_occ = os::getenv("maximum_occurences")
        .flat_map(|s| from_str(s.as_slice()))
        .unwrap_or_else(|| 1u);
    println!("max_occ: {}", max_occ);

    let mut map = HashMap::new();
    map.insert("a", 1u);
    map.insert("b", 2u);
    map.insert("c", 7u);

    let mut output = Vec::new();
    output.push(4i);

    // read file.
    // Reverse: either in-memory or while reading

}

// This method should probably be in the standard library
// I should probably ask rust people and PR this method if needed
trait OptionFlatMap<T> {
    fn flat_map<U>(&self, f: |&T| -> Option<U>) -> Option<U>;
}
impl<T> OptionFlatMap<T> for Option<T> {
    fn flat_map<U>(&self, f: |&T| -> Option<U>) -> Option<U> {
        match *self {
            Some(ref t) => f(t) ,
            None => None
        }
    }
}


// The awful fact about this script is that it's both valid Shell(sh) and valid Rust.
// Bash invokes compiling if neccessary,
// Rust does the real task of the script.
//
// This way of launching a script also keeps the command-line arguments,
// although we ignore them in our Rust code.
