# rust_iso/iso15924

A rust crate providing ISO 15924 support.

## What is ISO 15924

ISO 15924, Codes for the representation of names of scripts, is an international standard defining codes for writing systems or scripts (a "set of graphic characters used for the written form of one or more languages"). Each script is given both a four-letter code and a numeric code.[1]

Where possible the codes are derived from ISO 639-2, where the name of a script and the name of a language using the script are identical (example: Gujarātī ISO 639 guj, ISO 15924 Gujr). Preference is given to the 639-2 Bibliographical codes, which is different from the otherwise often preferred use of the Terminological codes.[1]

4-letter ISO 15924 codes are incorporated into the IANA Language Subtag Registry for IETF language tags and so can be used in file formats that make use of such language tags. For example, they can be used in HTML and XML to help Web browsers determine which typeface to use for foreign text. This way one could differentiate, for example, between Serbian written in the Cyrillic (sr-Cyrl) or Latin (sr-Latn) script, or mark romanized or transliterated text as such.

> _-- [Wikipedia](https://en.wikipedia.org/wiki/ISO_15924)_

## Installing

```sh
[dependencies]
rust_iso15924 = "0.0.1"
```

## License

rust-iso/rust_iso15924 is licensed under the Apache-2.0 license.

## Using

See [using](https://crates.io/crates/rust_iso15924) section of the documentation.

Quick guide:

```rust
let script = rust_iso15924::from_code("Adlm");
assert_eq!(166, script.unwrap().numeric);

let script = rust_iso15924::from_numeric(439);
assert_eq!("Afak", script.unwrap().code);

let script = rust_iso15924::from_numeric_str("439");
assert_eq!("Afak", script.unwrap().code);

println!("{:?}", rust_iso15924::ALL);
println!("{:?}", rust_iso15924::ALL_MAP);
println!("{:?}", rust_iso15924::NUMERIC_MAP);
println!("{:?}", rust_iso15924::ALL_NAME);
println!("{:?}", rust_iso15924::ALL_CODE);
println!("{:?}", rust_iso15924::ALL_NUMERIC);
println!("{:?}", rust_iso15924::ALL_NUMERIC_STR);
```

Data sample:

```rust
pub struct LanguageCode<'a> {
    ///ISO Language Name
    pub name: &'static str,
    ///639-1
    pub code: &'static str,
    ///639-2/T
    pub code_2t: &'static str,
    ///639-2/B
    pub code_2b: &'static str,
    //639-3 Macrolanguage
    pub code_3: &'static str,

    pub individual_languages: &'a [IndividualLanguages],
}

#[derive(Debug, Copy, Clone)]
pub struct IndividualLanguages {
    ///Name
    pub name: &'static str,
    ///Code
    pub code: &'static str,
}
```

## Source(s)

- https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
- https://en.wikipedia.org/wiki/ISO_639_macrolanguage
- https://www.rfc-editor.org/rfc/bcp/bcp47.txt
- http://www.iana.org/assignments/language-subtag-registry/language-subtag-registry
- https://www.alchemysoftware.com/livedocs/ezscript/Topics/Catalyst/Language.htm
- [Codes for the representation of names of scripts](https://en.wikipedia.org/wiki/ISO_15924)
- https://www.zhihu.com/question/21980689
- https://en.wikipedia.org/wiki/Language_code
- https://iso639-3.sil.org/code_tables/download_tables
- http://unicode.org/iso15924/
