#!/usr/bin/env python3.9
import re
import sys
f = open(r"./15924.txt", "r+")

codes = []


def clean(s):
    s = s.replace('<tr><td>', '').strip()
    s = s.replace('</td></tr>', '').strip()
    s = s.replace('&nbsp;', '').strip()
    return s


for x in f.readlines():
    x = x.strip()
    ts = x.split('</td><td>')
    code = clean(ts[0])
    if code == "Code":
        continue
    numeric = clean(ts[1])
    name = clean(ts[2])
    alias = clean(ts[4])
    ts1 = ts[5].split('</td><td nowrap>')
    age = clean(ts1[0])
    date = clean(ts1[1])
    c = {"code": code, "numeric": numeric, "name": name,
         "alias": alias, "age": age, "date": date}
    codes.append(c)

prefix = """use phf::{phf_map, Map};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_code() {
        let l = crate::from_code("Adlm");
        print!("test_from_code result {:?}", l)
    }

    #[test]
    fn test_from_numeric() {
        let l = crate::from_numeric(360);
        print!("test_from_code result {:?}", l)
    }

    #[test]
    fn test_from_numeric_str() {
        let l = crate::from_numeric_str("239");
        print!("test_from_code result {:?}", l)
    }

    #[test]
    fn test_all() {
        println!("{:?}", crate::ALL);
        println!("{:?}", crate::ALL_MAP);
    }
}
//https://unicode.org/iso15924/iso15924-codes.html
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScriptCode {
    //English Name
     name: &'static str,
    //ISO number
    pub numeric: i32,
    //code
    code: &'static str,
    //alias
    alias: &'static str,
    //age
     age: &'static str,
    //date
    date: &'static str,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScriptCode {
    //English Name
    pub name: &'static str,
    //ISO number
    pub numeric: i32,
    //code
    pub code: &'static str,
    //alias
    pub alias: &'static str,
    //age
    pub age: &'static str,
    //date
    pub date: &'static str,
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ScriptCode {
     #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.into()
    }
     #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.into()
    }

     #[wasm_bindgen(getter)]
    pub fn alias(&self) -> String {
        self.alias.into()
    }

     #[wasm_bindgen(getter)]
    pub fn age(&self) -> String {
        self.age.into()
    }

    #[wasm_bindgen(getter)]
    pub fn date(&self) -> String {
        self.date.into()
    }
}


/// Returns the ScriptCode with the given Alpha4 code, if exists.
/// #Sample
/// ```
/// let script = rust_iso15924::from_code("Adlm");
/// assert_eq!(166, script.unwrap().numeric);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_code(alpha4: &str) -> Option<ScriptCode> {
    let up = alpha4.to_lowercase();
    ALL_MAP.get(&up).cloned()
}
/// Returns the ScriptCode with the given numeric , if exists.
// #Sample
/// ```
/// let script = rust_iso15924::from_numeric(439);
/// assert_eq!("Afak", script.unwrap().code);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_numeric(numeric: i32) -> Option<ScriptCode> {
    let k = format!("{:03}", numeric);
    NUMERIC_MAP.get(&k).cloned()
}

/// Returns the ScriptCode with the given numeric 3 length str, if exists.
// #Sample
/// ```
/// let script = rust_iso15924::from_numeric_str("439");
/// assert_eq!("Afak", script.unwrap().code);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_numeric_str(numeric: &str) -> Option<ScriptCode> {
    NUMERIC_MAP.get(numeric).cloned()
}
"""
print(prefix)


for x in codes:
    print("""
pub const %s: ScriptCode = ScriptCode {
    name: "%s",
    code: "%s",
    numeric: %s,
    alias: "%s",
    age: "%s",
    date: "%s",

};
""" % (x["code"].upper(), x["name"], x["code"], x["numeric"], x["alias"], x["age"], x["date"]))


print("""
///ScriptCode map with  alpha4  str Code key
pub const ALL_MAP: Map<&str, ScriptCode> = phf_map! {
    """)
for x in codes:
    print("\"%s\" => %s, " % (x["code"].lower(), x["code"].upper()))
print("""
};
""")

print("""
///ScriptCode map with  3 len numeric str Code key
pub const NUMERIC_MAP: Map<&str, ScriptCode> = phf_map! {
    """)
for x in codes:
    print("\"%s\" => %s, " % (x["numeric"], x["code"].upper()))
print("""
};
""")

print("""
///ALL names
pub const ALL_NAME: & [ & str] = &[
    """)
for x in codes:
    print("\"%s\"," % (x["name"]))
print("""
];
""")

print("""
///ALL codes
pub const ALL_CODE: & [ & str] = &[
    """)
for x in codes:
    print("\"%s\"," % (x["code"]))
print("""
];
""")


print("""
///ALL the ScriptCodes struct
pub const ALL_NUMERIC: & [i32] = &[
""")
for x in codes:
    print("%s," % (x["numeric"].upper()))
print("""
];
""")

print("""
///ALL the ScriptCodes struct
pub const ALL_NUMERIC_STR: & [ & str] = &[
""")
for x in codes:
    print("\"%s\"," % (x["numeric"].upper()))
print("""
];
""")

print("""
///ALL the ScriptCodes struct
pub const ALL: & [ScriptCode] = &[
""")
for x in codes:
    print("%s," % (x["code"].upper()))
print("""
];
""")
