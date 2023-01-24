use phf::{phf_map, Map};
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
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
     #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.into()
    }
    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
     #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.into()
    }

    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
     #[wasm_bindgen(getter)]
    pub fn alias(&self) -> String {
        self.alias.into()
    }

    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
     #[wasm_bindgen(getter)]
    pub fn age(&self) -> String {
        self.age.into()
    }

    // #[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter))]
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


pub const ADLM: ScriptCode = ScriptCode {
    name: "Adlam",
    code: "Adlm",
    numeric: 166,
    alias: "Adlam",
    age: "9.0",
    date: "2016-12-05",

};


pub const AFAK: ScriptCode = ScriptCode {
    name: "Afaka",
    code: "Afak",
    numeric: 439,
    alias: "",
    age: "",
    date: "2010-12-21",

};


pub const AGHB: ScriptCode = ScriptCode {
    name: "Caucasian Albanian",
    code: "Aghb",
    numeric: 239,
    alias: "Caucasian_Albanian",
    age: "7.0",
    date: "2014-11-15",

};


pub const AHOM: ScriptCode = ScriptCode {
    name: "Ahom, Tai Ahom",
    code: "Ahom",
    numeric: 338,
    alias: "Ahom",
    age: "8.0",
    date: "2015-07-07",

};


pub const ARAB: ScriptCode = ScriptCode {
    name: "Arabic",
    code: "Arab",
    numeric: 160,
    alias: "Arabic",
    age: "1.1",
    date: "2004-05-01",

};


pub const ARAN: ScriptCode = ScriptCode {
    name: "Arabic (Nastaliq variant)",
    code: "Aran",
    numeric: 161,
    alias: "",
    age: "1.1",
    date: "2014-11-15",

};


pub const ARMI: ScriptCode = ScriptCode {
    name: "Imperial Aramaic",
    code: "Armi",
    numeric: 124,
    alias: "Imperial_Aramaic",
    age: "5.2",
    date: "2009-06-01",

};


pub const ARMN: ScriptCode = ScriptCode {
    name: "Armenian",
    code: "Armn",
    numeric: 230,
    alias: "Armenian",
    age: "1.1",
    date: "2004-05-01",

};


pub const AVST: ScriptCode = ScriptCode {
    name: "Avestan",
    code: "Avst",
    numeric: 134,
    alias: "Avestan",
    age: "5.2",
    date: "2009-06-01",

};


pub const BALI: ScriptCode = ScriptCode {
    name: "Balinese",
    code: "Bali",
    numeric: 360,
    alias: "Balinese",
    age: "5.0",
    date: "2006-10-10",

};


pub const BAMU: ScriptCode = ScriptCode {
    name: "Bamum",
    code: "Bamu",
    numeric: 435,
    alias: "Bamum",
    age: "5.2",
    date: "2009-06-01",

};


pub const BASS: ScriptCode = ScriptCode {
    name: "Bassa Vah",
    code: "Bass",
    numeric: 259,
    alias: "Bassa_Vah",
    age: "7.0",
    date: "2014-11-15",

};


pub const BATK: ScriptCode = ScriptCode {
    name: "Batak",
    code: "Batk",
    numeric: 365,
    alias: "Batak",
    age: "6.0",
    date: "2010-07-23",

};


pub const BENG: ScriptCode = ScriptCode {
    name: "Bengali (Bangla)",
    code: "Beng",
    numeric: 325,
    alias: "Bengali",
    age: "1.1",
    date: "2016-12-05",

};


pub const BHKS: ScriptCode = ScriptCode {
    name: "Bhaiksuki",
    code: "Bhks",
    numeric: 334,
    alias: "Bhaiksuki",
    age: "9.0",
    date: "2016-12-05",

};


pub const BLIS: ScriptCode = ScriptCode {
    name: "Blissymbols",
    code: "Blis",
    numeric: 550,
    alias: "",
    age: "",
    date: "2004-05-01",

};


pub const BOPO: ScriptCode = ScriptCode {
    name: "Bopomofo",
    code: "Bopo",
    numeric: 285,
    alias: "Bopomofo",
    age: "1.1",
    date: "2004-05-01",

};


pub const BRAH: ScriptCode = ScriptCode {
    name: "Brahmi",
    code: "Brah",
    numeric: 300,
    alias: "Brahmi",
    age: "6.0",
    date: "2010-07-23",

};


pub const BRAI: ScriptCode = ScriptCode {
    name: "Braille",
    code: "Brai",
    numeric: 570,
    alias: "Braille",
    age: "3.0",
    date: "2004-05-01",

};


pub const BUGI: ScriptCode = ScriptCode {
    name: "Buginese",
    code: "Bugi",
    numeric: 367,
    alias: "Buginese",
    age: "4.1",
    date: "2006-06-21",

};


pub const BUHD: ScriptCode = ScriptCode {
    name: "Buhid",
    code: "Buhd",
    numeric: 372,
    alias: "Buhid",
    age: "3.2",
    date: "2004-05-01",

};


pub const CAKM: ScriptCode = ScriptCode {
    name: "Chakma",
    code: "Cakm",
    numeric: 349,
    alias: "Chakma",
    age: "6.1",
    date: "2012-02-06",

};


pub const CANS: ScriptCode = ScriptCode {
    name: "Unified Canadian Aboriginal Syllabics",
    code: "Cans",
    numeric: 440,
    alias: "Canadian_Aboriginal",
    age: "3.0",
    date: "2004-05-29",

};


pub const CARI: ScriptCode = ScriptCode {
    name: "Carian",
    code: "Cari",
    numeric: 201,
    alias: "Carian",
    age: "5.1",
    date: "2007-07-02",

};


pub const CHAM: ScriptCode = ScriptCode {
    name: "Cham",
    code: "Cham",
    numeric: 358,
    alias: "Cham",
    age: "5.1",
    date: "2009-11-11",

};


pub const CHER: ScriptCode = ScriptCode {
    name: "Cherokee",
    code: "Cher",
    numeric: 445,
    alias: "Cherokee",
    age: "3.0",
    date: "2004-05-01",

};


pub const CHRS: ScriptCode = ScriptCode {
    name: "Chorasmian",
    code: "Chrs",
    numeric: 109,
    alias: "Chorasmian",
    age: "13.0",
    date: "2019-08-19",

};


pub const CIRT: ScriptCode = ScriptCode {
    name: "Cirth",
    code: "Cirt",
    numeric: 291,
    alias: "",
    age: "",
    date: "2004-05-01",

};


pub const COPT: ScriptCode = ScriptCode {
    name: "Coptic",
    code: "Copt",
    numeric: 204,
    alias: "Coptic",
    age: "4.1",
    date: "2006-06-21",

};


pub const CPMN: ScriptCode = ScriptCode {
    name: "Cypro-Minoan",
    code: "Cpmn",
    numeric: 402,
    alias: "Cypro_Minoan",
    age: "14.0",
    date: "2017-07-26",

};


pub const CPRT: ScriptCode = ScriptCode {
    name: "Cypriot syllabary",
    code: "Cprt",
    numeric: 403,
    alias: "Cypriot",
    age: "4.0",
    date: "2017-07-26",

};


pub const CYRL: ScriptCode = ScriptCode {
    name: "Cyrillic",
    code: "Cyrl",
    numeric: 220,
    alias: "Cyrillic",
    age: "1.1",
    date: "2004-05-01",

};


pub const CYRS: ScriptCode = ScriptCode {
    name: "Cyrillic (Old Church Slavonic variant)",
    code: "Cyrs",
    numeric: 221,
    alias: "",
    age: "1.1",
    date: "2004-05-01",

};


pub const DEVA: ScriptCode = ScriptCode {
    name: "Devanagari (Nagari)",
    code: "Deva",
    numeric: 315,
    alias: "Devanagari",
    age: "1.1",
    date: "2004-05-01",

};


pub const DIAK: ScriptCode = ScriptCode {
    name: "Dives Akuru",
    code: "Diak",
    numeric: 342,
    alias: "Dives_Akuru",
    age: "13.0",
    date: "2019-08-19",

};


pub const DOGR: ScriptCode = ScriptCode {
    name: "Dogra",
    code: "Dogr",
    numeric: 328,
    alias: "Dogra",
    age: "11.0",
    date: "2016-12-05",

};


pub const DSRT: ScriptCode = ScriptCode {
    name: "Deseret (Mormon)",
    code: "Dsrt",
    numeric: 250,
    alias: "Deseret",
    age: "3.1",
    date: "2004-05-01",

};


pub const DUPL: ScriptCode = ScriptCode {
    name: "Duployan shorthand, Duployan stenography",
    code: "Dupl",
    numeric: 755,
    alias: "Duployan",
    age: "7.0",
    date: "2014-11-15",

};


pub const EGYD: ScriptCode = ScriptCode {
    name: "Egyptian demotic",
    code: "Egyd",
    numeric: 070,
    alias: "",
    age: "",
    date: "2004-05-01",

};


pub const EGYH: ScriptCode = ScriptCode {
    name: "Egyptian hieratic",
    code: "Egyh",
    numeric: 060,
    alias: "",
    age: "5.2",
    date: "2004-05-01",

};


pub const EGYP: ScriptCode = ScriptCode {
    name: "Egyptian hieroglyphs",
    code: "Egyp",
    numeric: 050,
    alias: "Egyptian_Hieroglyphs",
    age: "5.2",
    date: "2009-06-01",

};


pub const ELBA: ScriptCode = ScriptCode {
    name: "Elbasan",
    code: "Elba",
    numeric: 226,
    alias: "Elbasan",
    age: "7.0",
    date: "2014-11-15",

};


pub const ELYM: ScriptCode = ScriptCode {
    name: "Elymaic",
    code: "Elym",
    numeric: 128,
    alias: "Elymaic",
    age: "12.0",
    date: "2018-08-26",

};


pub const ETHI: ScriptCode = ScriptCode {
    name: "Ethiopic (Geʻez)",
    code: "Ethi",
    numeric: 430,
    alias: "Ethiopic",
    age: "3.0",
    date: "2004-10-25",

};


pub const GEOK: ScriptCode = ScriptCode {
    name: "Khutsuri (Asomtavruli and Nuskhuri)",
    code: "Geok",
    numeric: 241,
    alias: "Georgian",
    age: "1.1",
    date: "2012-10-16",

};


pub const GEOR: ScriptCode = ScriptCode {
    name: "Georgian (Mkhedruli and Mtavruli)",
    code: "Geor",
    numeric: 240,
    alias: "Georgian",
    age: "1.1",
    date: "2016-12-05",

};


pub const GLAG: ScriptCode = ScriptCode {
    name: "Glagolitic",
    code: "Glag",
    numeric: 225,
    alias: "Glagolitic",
    age: "4.1",
    date: "2006-06-21",

};


pub const GONG: ScriptCode = ScriptCode {
    name: "Gunjala Gondi",
    code: "Gong",
    numeric: 312,
    alias: "Gunjala_Gondi",
    age: "11.0",
    date: "2016-12-05",

};


pub const GONM: ScriptCode = ScriptCode {
    name: "Masaram Gondi",
    code: "Gonm",
    numeric: 313,
    alias: "Masaram_Gondi",
    age: "10.0",
    date: "2017-07-26",

};


pub const GOTH: ScriptCode = ScriptCode {
    name: "Gothic",
    code: "Goth",
    numeric: 206,
    alias: "Gothic",
    age: "3.1",
    date: "2004-05-01",

};


pub const GRAN: ScriptCode = ScriptCode {
    name: "Grantha",
    code: "Gran",
    numeric: 343,
    alias: "Grantha",
    age: "7.0",
    date: "2014-11-15",

};


pub const GREK: ScriptCode = ScriptCode {
    name: "Greek",
    code: "Grek",
    numeric: 200,
    alias: "Greek",
    age: "1.1",
    date: "2004-05-01",

};


pub const GUJR: ScriptCode = ScriptCode {
    name: "Gujarati",
    code: "Gujr",
    numeric: 320,
    alias: "Gujarati",
    age: "1.1",
    date: "2004-05-01",

};


pub const GURU: ScriptCode = ScriptCode {
    name: "Gurmukhi",
    code: "Guru",
    numeric: 310,
    alias: "Gurmukhi",
    age: "1.1",
    date: "2004-05-01",

};


pub const HANB: ScriptCode = ScriptCode {
    name: "Han with Bopomofo (alias for Han + Bopomofo)",
    code: "Hanb",
    numeric: 503,
    alias: "",
    age: "1.1",
    date: "2016-01-19",

};


pub const HANG: ScriptCode = ScriptCode {
    name: "Hangul (Hangŭl, Hangeul)",
    code: "Hang",
    numeric: 286,
    alias: "Hangul",
    age: "1.1",
    date: "2004-05-29",

};


pub const HANI: ScriptCode = ScriptCode {
    name: "Han (Hanzi, Kanji, Hanja)",
    code: "Hani",
    numeric: 500,
    alias: "Han",
    age: "1.1",
    date: "2009-02-23",

};


pub const HANO: ScriptCode = ScriptCode {
    name: "Hanunoo (Hanunóo)",
    code: "Hano",
    numeric: 371,
    alias: "Hanunoo",
    age: "3.2",
    date: "2004-05-29",

};


pub const HANS: ScriptCode = ScriptCode {
    name: "Han (Simplified variant)",
    code: "Hans",
    numeric: 501,
    alias: "",
    age: "1.1",
    date: "2004-05-29",

};


pub const HANT: ScriptCode = ScriptCode {
    name: "Han (Traditional variant)",
    code: "Hant",
    numeric: 502,
    alias: "",
    age: "1.1",
    date: "2004-05-29",

};


pub const HATR: ScriptCode = ScriptCode {
    name: "Hatran",
    code: "Hatr",
    numeric: 127,
    alias: "Hatran",
    age: "8.0",
    date: "2015-07-07",

};


pub const HEBR: ScriptCode = ScriptCode {
    name: "Hebrew",
    code: "Hebr",
    numeric: 125,
    alias: "Hebrew",
    age: "1.1",
    date: "2004-05-01",

};


pub const HIRA: ScriptCode = ScriptCode {
    name: "Hiragana",
    code: "Hira",
    numeric: 410,
    alias: "Hiragana",
    age: "1.1",
    date: "2004-05-01",

};


pub const HLUW: ScriptCode = ScriptCode {
    name: "Anatolian Hieroglyphs (Luwian Hieroglyphs, Hittite Hieroglyphs)",
    code: "Hluw",
    numeric: 080,
    alias: "Anatolian_Hieroglyphs",
    age: "8.0",
    date: "2015-07-07",

};


pub const HMNG: ScriptCode = ScriptCode {
    name: "Pahawh Hmong",
    code: "Hmng",
    numeric: 450,
    alias: "Pahawh_Hmong",
    age: "7.0",
    date: "2014-11-15",

};


pub const HMNP: ScriptCode = ScriptCode {
    name: "Nyiakeng Puachue Hmong",
    code: "Hmnp",
    numeric: 451,
    alias: "Nyiakeng_Puachue_Hmong",
    age: "12.0",
    date: "2017-07-26",

};


pub const HRKT: ScriptCode = ScriptCode {
    name: "Japanese syllabaries (alias for Hiragana + Katakana)",
    code: "Hrkt",
    numeric: 412,
    alias: "Katakana_Or_Hiragana",
    age: "1.1",
    date: "2011-06-21",

};


pub const HUNG: ScriptCode = ScriptCode {
    name: "Old Hungarian (Hungarian Runic)",
    code: "Hung",
    numeric: 176,
    alias: "Old_Hungarian",
    age: "8.0",
    date: "2015-07-07",

};


pub const INDS: ScriptCode = ScriptCode {
    name: "Indus (Harappan)",
    code: "Inds",
    numeric: 610,
    alias: "",
    age: "",
    date: "2004-05-01",

};


pub const ITAL: ScriptCode = ScriptCode {
    name: "Old Italic (Etruscan, Oscan, etc.)",
    code: "Ital",
    numeric: 210,
    alias: "Old_Italic",
    age: "3.1",
    date: "2004-05-29",

};


pub const JAMO: ScriptCode = ScriptCode {
    name: "Jamo (alias for Jamo subset of Hangul)",
    code: "Jamo",
    numeric: 284,
    alias: "",
    age: "1.1",
    date: "2016-01-19",

};


pub const JAVA: ScriptCode = ScriptCode {
    name: "Javanese",
    code: "Java",
    numeric: 361,
    alias: "Javanese",
    age: "5.2",
    date: "2009-06-01",

};


pub const JPAN: ScriptCode = ScriptCode {
    name: "Japanese (alias for Han + Hiragana + Katakana)",
    code: "Jpan",
    numeric: 413,
    alias: "",
    age: "1.1",
    date: "2006-06-21",

};


pub const JURC: ScriptCode = ScriptCode {
    name: "Jurchen",
    code: "Jurc",
    numeric: 510,
    alias: "",
    age: "",
    date: "2010-12-21",

};


pub const KALI: ScriptCode = ScriptCode {
    name: "Kayah Li",
    code: "Kali",
    numeric: 357,
    alias: "Kayah_Li",
    age: "5.1",
    date: "2007-07-02",

};


pub const KANA: ScriptCode = ScriptCode {
    name: "Katakana",
    code: "Kana",
    numeric: 411,
    alias: "Katakana",
    age: "1.1",
    date: "2004-05-01",

};


pub const KAWI: ScriptCode = ScriptCode {
    name: "Kawi",
    code: "Kawi",
    numeric: 368,
    alias: "",
    age: "",
    date: "2021-12-03",

};


pub const KHAR: ScriptCode = ScriptCode {
    name: "Kharoshthi",
    code: "Khar",
    numeric: 305,
    alias: "Kharoshthi",
    age: "4.1",
    date: "2006-06-21",

};


pub const KHMR: ScriptCode = ScriptCode {
    name: "Khmer",
    code: "Khmr",
    numeric: 355,
    alias: "Khmer",
    age: "3.0",
    date: "2004-05-29",

};


pub const KHOJ: ScriptCode = ScriptCode {
    name: "Khojki",
    code: "Khoj",
    numeric: 322,
    alias: "Khojki",
    age: "7.0",
    date: "2014-11-15",

};


pub const KITL: ScriptCode = ScriptCode {
    name: "Khitan large script",
    code: "Kitl",
    numeric: 505,
    alias: "",
    age: "",
    date: "2015-07-15",

};


pub const KITS: ScriptCode = ScriptCode {
    name: "Khitan small script",
    code: "Kits",
    numeric: 288,
    alias: "Khitan_Small_Script",
    age: "13.0",
    date: "2015-07-15",

};


pub const KNDA: ScriptCode = ScriptCode {
    name: "Kannada",
    code: "Knda",
    numeric: 345,
    alias: "Kannada",
    age: "1.1",
    date: "2004-05-29",

};


pub const KORE: ScriptCode = ScriptCode {
    name: "Korean (alias for Hangul + Han)",
    code: "Kore",
    numeric: 287,
    alias: "",
    age: "1.1",
    date: "2007-06-13",

};


pub const KPEL: ScriptCode = ScriptCode {
    name: "Kpelle",
    code: "Kpel",
    numeric: 436,
    alias: "",
    age: "",
    date: "2010-03-26",

};


pub const KTHI: ScriptCode = ScriptCode {
    name: "Kaithi",
    code: "Kthi",
    numeric: 317,
    alias: "Kaithi",
    age: "5.2",
    date: "2009-06-01",

};


pub const LANA: ScriptCode = ScriptCode {
    name: "Tai Tham (Lanna)",
    code: "Lana",
    numeric: 351,
    alias: "Tai_Tham",
    age: "5.2",
    date: "2009-06-01",

};


pub const LAOO: ScriptCode = ScriptCode {
    name: "Lao",
    code: "Laoo",
    numeric: 356,
    alias: "Lao",
    age: "1.1",
    date: "2004-05-01",

};


pub const LATF: ScriptCode = ScriptCode {
    name: "Latin (Fraktur variant)",
    code: "Latf",
    numeric: 217,
    alias: "",
    age: "1.1",
    date: "2004-05-01",

};


pub const LATG: ScriptCode = ScriptCode {
    name: "Latin (Gaelic variant)",
    code: "Latg",
    numeric: 216,
    alias: "",
    age: "1.1",
    date: "2004-05-01",

};


pub const LATN: ScriptCode = ScriptCode {
    name: "Latin",
    code: "Latn",
    numeric: 215,
    alias: "Latin",
    age: "1.1",
    date: "2004-05-01",

};


pub const LEKE: ScriptCode = ScriptCode {
    name: "Leke",
    code: "Leke",
    numeric: 364,
    alias: "",
    age: "",
    date: "2015-07-07",

};


pub const LEPC: ScriptCode = ScriptCode {
    name: "Lepcha (Róng)",
    code: "Lepc",
    numeric: 335,
    alias: "Lepcha",
    age: "5.1",
    date: "2007-07-02",

};


pub const LIMB: ScriptCode = ScriptCode {
    name: "Limbu",
    code: "Limb",
    numeric: 336,
    alias: "Limbu",
    age: "4.0",
    date: "2004-05-29",

};


pub const LINA: ScriptCode = ScriptCode {
    name: "Linear A",
    code: "Lina",
    numeric: 400,
    alias: "Linear_A",
    age: "7.0",
    date: "2014-11-15",

};


pub const LINB: ScriptCode = ScriptCode {
    name: "Linear B",
    code: "Linb",
    numeric: 401,
    alias: "Linear_B",
    age: "4.0",
    date: "2004-05-29",

};


pub const LISU: ScriptCode = ScriptCode {
    name: "Lisu (Fraser)",
    code: "Lisu",
    numeric: 399,
    alias: "Lisu",
    age: "5.2",
    date: "2009-06-01",

};


pub const LOMA: ScriptCode = ScriptCode {
    name: "Loma",
    code: "Loma",
    numeric: 437,
    alias: "",
    age: "",
    date: "2010-03-26",

};


pub const LYCI: ScriptCode = ScriptCode {
    name: "Lycian",
    code: "Lyci",
    numeric: 202,
    alias: "Lycian",
    age: "5.1",
    date: "2007-07-02",

};


pub const LYDI: ScriptCode = ScriptCode {
    name: "Lydian",
    code: "Lydi",
    numeric: 116,
    alias: "Lydian",
    age: "5.1",
    date: "2007-07-02",

};


pub const MAHJ: ScriptCode = ScriptCode {
    name: "Mahajani",
    code: "Mahj",
    numeric: 314,
    alias: "Mahajani",
    age: "7.0",
    date: "2014-11-15",

};


pub const MAKA: ScriptCode = ScriptCode {
    name: "Makasar",
    code: "Maka",
    numeric: 366,
    alias: "Makasar",
    age: "11.0",
    date: "2016-12-05",

};


pub const MAND: ScriptCode = ScriptCode {
    name: "Mandaic, Mandaean",
    code: "Mand",
    numeric: 140,
    alias: "Mandaic",
    age: "6.0",
    date: "2010-07-23",

};


pub const MANI: ScriptCode = ScriptCode {
    name: "Manichaean",
    code: "Mani",
    numeric: 139,
    alias: "Manichaean",
    age: "7.0",
    date: "2014-11-15",

};


pub const MARC: ScriptCode = ScriptCode {
    name: "Marchen",
    code: "Marc",
    numeric: 332,
    alias: "Marchen",
    age: "9.0",
    date: "2016-12-05",

};


pub const MAYA: ScriptCode = ScriptCode {
    name: "Mayan hieroglyphs",
    code: "Maya",
    numeric: 090,
    alias: "",
    age: "",
    date: "2004-05-01",

};


pub const MEDF: ScriptCode = ScriptCode {
    name: "Medefaidrin (Oberi Okaime, Oberi Ɔkaimɛ)",
    code: "Medf",
    numeric: 265,
    alias: "Medefaidrin",
    age: "11.0",
    date: "2016-12-05",

};


pub const MEND: ScriptCode = ScriptCode {
    name: "Mende Kikakui",
    code: "Mend",
    numeric: 438,
    alias: "Mende_Kikakui",
    age: "7.0",
    date: "2014-11-15",

};


pub const MERC: ScriptCode = ScriptCode {
    name: "Meroitic Cursive",
    code: "Merc",
    numeric: 101,
    alias: "Meroitic_Cursive",
    age: "6.1",
    date: "2012-02-06",

};


pub const MERO: ScriptCode = ScriptCode {
    name: "Meroitic Hieroglyphs",
    code: "Mero",
    numeric: 100,
    alias: "Meroitic_Hieroglyphs",
    age: "6.1",
    date: "2012-02-06",

};


pub const MLYM: ScriptCode = ScriptCode {
    name: "Malayalam",
    code: "Mlym",
    numeric: 347,
    alias: "Malayalam",
    age: "1.1",
    date: "2004-05-01",

};


pub const MODI: ScriptCode = ScriptCode {
    name: "Modi, Moḍī",
    code: "Modi",
    numeric: 324,
    alias: "Modi",
    age: "7.0",
    date: "2014-11-15",

};


pub const MONG: ScriptCode = ScriptCode {
    name: "Mongolian",
    code: "Mong",
    numeric: 145,
    alias: "Mongolian",
    age: "3.0",
    date: "2004-05-01",

};


pub const MOON: ScriptCode = ScriptCode {
    name: "Moon (Moon code, Moon script, Moon type)",
    code: "Moon",
    numeric: 218,
    alias: "",
    age: "",
    date: "2006-12-11",

};


pub const MROO: ScriptCode = ScriptCode {
    name: "Mro, Mru",
    code: "Mroo",
    numeric: 264,
    alias: "Mro",
    age: "7.0",
    date: "2016-12-05",

};


pub const MTEI: ScriptCode = ScriptCode {
    name: "Meitei Mayek (Meithei, Meetei)",
    code: "Mtei",
    numeric: 337,
    alias: "Meetei_Mayek",
    age: "5.2",
    date: "2009-06-01",

};


pub const MULT: ScriptCode = ScriptCode {
    name: "Multani",
    code: "Mult",
    numeric: 323,
    alias: "Multani",
    age: "8.0",
    date: "2015-07-07",

};


pub const MYMR: ScriptCode = ScriptCode {
    name: "Myanmar (Burmese)",
    code: "Mymr",
    numeric: 350,
    alias: "Myanmar",
    age: "3.0",
    date: "2004-05-01",

};


pub const NAGM: ScriptCode = ScriptCode {
    name: "Nag Mundari",
    code: "Nagm",
    numeric: 295,
    alias: "",
    age: "",
    date: "2021-12-03",

};


pub const NAND: ScriptCode = ScriptCode {
    name: "Nandinagari",
    code: "Nand",
    numeric: 311,
    alias: "Nandinagari",
    age: "12.0",
    date: "2018-08-26",

};


pub const NARB: ScriptCode = ScriptCode {
    name: "Old North Arabian (Ancient North Arabian)",
    code: "Narb",
    numeric: 106,
    alias: "Old_North_Arabian",
    age: "7.0",
    date: "2014-11-15",

};


pub const NBAT: ScriptCode = ScriptCode {
    name: "Nabataean",
    code: "Nbat",
    numeric: 159,
    alias: "Nabataean",
    age: "7.0",
    date: "2014-11-15",

};


pub const NEWA: ScriptCode = ScriptCode {
    name: "Newa, Newar, Newari, Nepāla lipi",
    code: "Newa",
    numeric: 333,
    alias: "Newa",
    age: "9.0",
    date: "2016-12-05",

};


pub const NKDB: ScriptCode = ScriptCode {
    name: "Naxi Dongba (na²¹ɕi³³ to³³ba²¹, Nakhi Tomba)",
    code: "Nkdb",
    numeric: 085,
    alias: "",
    age: "",
    date: "2017-07-26",

};


pub const NKGB: ScriptCode = ScriptCode {
    name: "Naxi Geba (na²¹ɕi³³ gʌ²¹ba²¹, 'Na-'Khi ²Ggŏ-¹baw, Nakhi Geba)",
    code: "Nkgb",
    numeric: 420,
    alias: "",
    age: "",
    date: "2017-07-26",

};


pub const NKOO: ScriptCode = ScriptCode {
    name: "N’Ko",
    code: "Nkoo",
    numeric: 165,
    alias: "Nko",
    age: "5.0",
    date: "2006-10-10",

};


pub const NSHU: ScriptCode = ScriptCode {
    name: "Nüshu",
    code: "Nshu",
    numeric: 499,
    alias: "Nushu",
    age: "10.0",
    date: "2017-07-26",

};


pub const OGAM: ScriptCode = ScriptCode {
    name: "Ogham",
    code: "Ogam",
    numeric: 212,
    alias: "Ogham",
    age: "3.0",
    date: "2004-05-01",

};


pub const OLCK: ScriptCode = ScriptCode {
    name: "Ol Chiki (Ol Cemet’, Ol, Santali)",
    code: "Olck",
    numeric: 261,
    alias: "Ol_Chiki",
    age: "5.1",
    date: "2007-07-02",

};


pub const ORKH: ScriptCode = ScriptCode {
    name: "Old Turkic, Orkhon Runic",
    code: "Orkh",
    numeric: 175,
    alias: "Old_Turkic",
    age: "5.2",
    date: "2009-06-01",

};


pub const ORYA: ScriptCode = ScriptCode {
    name: "Oriya (Odia)",
    code: "Orya",
    numeric: 327,
    alias: "Oriya",
    age: "1.1",
    date: "2016-12-05",

};


pub const OSGE: ScriptCode = ScriptCode {
    name: "Osage",
    code: "Osge",
    numeric: 219,
    alias: "Osage",
    age: "9.0",
    date: "2016-12-05",

};


pub const OSMA: ScriptCode = ScriptCode {
    name: "Osmanya",
    code: "Osma",
    numeric: 260,
    alias: "Osmanya",
    age: "4.0",
    date: "2004-05-01",

};


pub const OUGR: ScriptCode = ScriptCode {
    name: "Old Uyghur",
    code: "Ougr",
    numeric: 143,
    alias: "Old_Uyghur",
    age: "14.0",
    date: "2021-01-25",

};


pub const PALM: ScriptCode = ScriptCode {
    name: "Palmyrene",
    code: "Palm",
    numeric: 126,
    alias: "Palmyrene",
    age: "7.0",
    date: "2014-11-15",

};


pub const PAUC: ScriptCode = ScriptCode {
    name: "Pau Cin Hau",
    code: "Pauc",
    numeric: 263,
    alias: "Pau_Cin_Hau",
    age: "7.0",
    date: "2014-11-15",

};


pub const PCUN: ScriptCode = ScriptCode {
    name: "Proto-Cuneiform",
    code: "Pcun",
    numeric: 015,
    alias: "",
    age: "",
    date: "2021-01-25",

};


pub const PELM: ScriptCode = ScriptCode {
    name: "Proto-Elamite",
    code: "Pelm",
    numeric: 016,
    alias: "",
    age: "",
    date: "2021-01-25",

};


pub const PERM: ScriptCode = ScriptCode {
    name: "Old Permic",
    code: "Perm",
    numeric: 227,
    alias: "Old_Permic",
    age: "7.0",
    date: "2014-11-15",

};


pub const PHAG: ScriptCode = ScriptCode {
    name: "Phags-pa",
    code: "Phag",
    numeric: 331,
    alias: "Phags_Pa",
    age: "5.0",
    date: "2006-10-10",

};


pub const PHLI: ScriptCode = ScriptCode {
    name: "Inscriptional Pahlavi",
    code: "Phli",
    numeric: 131,
    alias: "Inscriptional_Pahlavi",
    age: "5.2",
    date: "2009-06-01",

};


pub const PHLP: ScriptCode = ScriptCode {
    name: "Psalter Pahlavi",
    code: "Phlp",
    numeric: 132,
    alias: "Psalter_Pahlavi",
    age: "7.0",
    date: "2014-11-15",

};


pub const PHLV: ScriptCode = ScriptCode {
    name: "Book Pahlavi",
    code: "Phlv",
    numeric: 133,
    alias: "",
    age: "",
    date: "2007-07-15",

};


pub const PHNX: ScriptCode = ScriptCode {
    name: "Phoenician",
    code: "Phnx",
    numeric: 115,
    alias: "Phoenician",
    age: "5.0",
    date: "2006-10-10",

};


pub const PLRD: ScriptCode = ScriptCode {
    name: "Miao (Pollard)",
    code: "Plrd",
    numeric: 282,
    alias: "Miao",
    age: "6.1",
    date: "2012-02-06",

};


pub const PIQD: ScriptCode = ScriptCode {
    name: "Klingon (KLI pIqaD)",
    code: "Piqd",
    numeric: 293,
    alias: "",
    age: "",
    date: "2015-12-16",

};


pub const PRTI: ScriptCode = ScriptCode {
    name: "Inscriptional Parthian",
    code: "Prti",
    numeric: 130,
    alias: "Inscriptional_Parthian",
    age: "5.2",
    date: "2009-06-01",

};


pub const PSIN: ScriptCode = ScriptCode {
    name: "Proto-Sinaitic",
    code: "Psin",
    numeric: 103,
    alias: "",
    age: "",
    date: "2021-01-25",

};


pub const QAAA: ScriptCode = ScriptCode {
    name: "Reserved for private use (start)",
    code: "Qaaa",
    numeric: 900,
    alias: "",
    age: "",
    date: "2004-05-29",

};


pub const QABX: ScriptCode = ScriptCode {
    name: "Reserved for private use (end)",
    code: "Qabx",
    numeric: 949,
    alias: "",
    age: "",
    date: "2004-05-29",

};


pub const RANJ: ScriptCode = ScriptCode {
    name: "Ranjana",
    code: "Ranj",
    numeric: 303,
    alias: "",
    age: "",
    date: "2021-01-25",

};


pub const RJNG: ScriptCode = ScriptCode {
    name: "Rejang (Redjang, Kaganga)",
    code: "Rjng",
    numeric: 363,
    alias: "Rejang",
    age: "5.1",
    date: "2009-02-23",

};


pub const ROHG: ScriptCode = ScriptCode {
    name: "Hanifi Rohingya",
    code: "Rohg",
    numeric: 167,
    alias: "Hanifi_Rohingya",
    age: "11.0",
    date: "2017-11-21",

};


pub const RORO: ScriptCode = ScriptCode {
    name: "Rongorongo",
    code: "Roro",
    numeric: 620,
    alias: "",
    age: "",
    date: "2004-05-01",

};


pub const RUNR: ScriptCode = ScriptCode {
    name: "Runic",
    code: "Runr",
    numeric: 211,
    alias: "Runic",
    age: "3.0",
    date: "2004-05-01",

};


pub const SAMR: ScriptCode = ScriptCode {
    name: "Samaritan",
    code: "Samr",
    numeric: 123,
    alias: "Samaritan",
    age: "5.2",
    date: "2009-06-01",

};


pub const SARA: ScriptCode = ScriptCode {
    name: "Sarati",
    code: "Sara",
    numeric: 292,
    alias: "",
    age: "",
    date: "2004-05-29",

};


pub const SARB: ScriptCode = ScriptCode {
    name: "Old South Arabian",
    code: "Sarb",
    numeric: 105,
    alias: "Old_South_Arabian",
    age: "5.2",
    date: "2009-06-01",

};


pub const SAUR: ScriptCode = ScriptCode {
    name: "Saurashtra",
    code: "Saur",
    numeric: 344,
    alias: "Saurashtra",
    age: "5.1",
    date: "2007-07-02",

};


pub const SGNW: ScriptCode = ScriptCode {
    name: "SignWriting",
    code: "Sgnw",
    numeric: 095,
    alias: "SignWriting",
    age: "8.0",
    date: "2015-07-07",

};


pub const SHAW: ScriptCode = ScriptCode {
    name: "Shavian (Shaw)",
    code: "Shaw",
    numeric: 281,
    alias: "Shavian",
    age: "4.0",
    date: "2004-05-01",

};


pub const SHRD: ScriptCode = ScriptCode {
    name: "Sharada, Śāradā",
    code: "Shrd",
    numeric: 319,
    alias: "Sharada",
    age: "6.1",
    date: "2012-02-06",

};


pub const SHUI: ScriptCode = ScriptCode {
    name: "Shuishu",
    code: "Shui",
    numeric: 530,
    alias: "",
    age: "",
    date: "2017-07-26",

};


pub const SIDD: ScriptCode = ScriptCode {
    name: "Siddham, Siddhaṃ, Siddhamātṛkā",
    code: "Sidd",
    numeric: 302,
    alias: "Siddham",
    age: "7.0",
    date: "2014-11-15",

};


pub const SIND: ScriptCode = ScriptCode {
    name: "Khudawadi, Sindhi",
    code: "Sind",
    numeric: 318,
    alias: "Khudawadi",
    age: "7.0",
    date: "2014-11-15",

};


pub const SINH: ScriptCode = ScriptCode {
    name: "Sinhala",
    code: "Sinh",
    numeric: 348,
    alias: "Sinhala",
    age: "3.0",
    date: "2004-05-01",

};


pub const SOGD: ScriptCode = ScriptCode {
    name: "Sogdian",
    code: "Sogd",
    numeric: 141,
    alias: "Sogdian",
    age: "11.0",
    date: "2017-11-21",

};


pub const SOGO: ScriptCode = ScriptCode {
    name: "Old Sogdian",
    code: "Sogo",
    numeric: 142,
    alias: "Old_Sogdian",
    age: "11.0",
    date: "2017-11-21",

};


pub const SORA: ScriptCode = ScriptCode {
    name: "Sora Sompeng",
    code: "Sora",
    numeric: 398,
    alias: "Sora_Sompeng",
    age: "6.1",
    date: "2012-02-06",

};


pub const SOYO: ScriptCode = ScriptCode {
    name: "Soyombo",
    code: "Soyo",
    numeric: 329,
    alias: "Soyombo",
    age: "10.0",
    date: "2017-07-26",

};


pub const SUND: ScriptCode = ScriptCode {
    name: "Sundanese",
    code: "Sund",
    numeric: 362,
    alias: "Sundanese",
    age: "5.1",
    date: "2007-07-02",

};


pub const SUNU: ScriptCode = ScriptCode {
    name: "Sunuwar",
    code: "Sunu",
    numeric: 274,
    alias: "",
    age: "",
    date: "2021-12-03",

};


pub const SYLO: ScriptCode = ScriptCode {
    name: "Syloti Nagri",
    code: "Sylo",
    numeric: 316,
    alias: "Syloti_Nagri",
    age: "4.1",
    date: "2006-06-21",

};


pub const SYRC: ScriptCode = ScriptCode {
    name: "Syriac",
    code: "Syrc",
    numeric: 135,
    alias: "Syriac",
    age: "3.0",
    date: "2004-05-01",

};


pub const SYRE: ScriptCode = ScriptCode {
    name: "Syriac (Estrangelo variant)",
    code: "Syre",
    numeric: 138,
    alias: "",
    age: "3.0",
    date: "2004-05-01",

};


pub const SYRJ: ScriptCode = ScriptCode {
    name: "Syriac (Western variant)",
    code: "Syrj",
    numeric: 137,
    alias: "",
    age: "3.0",
    date: "2004-05-01",

};


pub const SYRN: ScriptCode = ScriptCode {
    name: "Syriac (Eastern variant)",
    code: "Syrn",
    numeric: 136,
    alias: "",
    age: "3.0",
    date: "2004-05-01",

};


pub const TAGB: ScriptCode = ScriptCode {
    name: "Tagbanwa",
    code: "Tagb",
    numeric: 373,
    alias: "Tagbanwa",
    age: "3.2",
    date: "2004-05-01",

};


pub const TAKR: ScriptCode = ScriptCode {
    name: "Takri, Ṭākrī, Ṭāṅkrī",
    code: "Takr",
    numeric: 321,
    alias: "Takri",
    age: "6.1",
    date: "2012-02-06",

};


pub const TALE: ScriptCode = ScriptCode {
    name: "Tai Le",
    code: "Tale",
    numeric: 353,
    alias: "Tai_Le",
    age: "4.0",
    date: "2004-10-25",

};


pub const TALU: ScriptCode = ScriptCode {
    name: "New Tai Lue",
    code: "Talu",
    numeric: 354,
    alias: "New_Tai_Lue",
    age: "4.1",
    date: "2006-06-21",

};


pub const TAML: ScriptCode = ScriptCode {
    name: "Tamil",
    code: "Taml",
    numeric: 346,
    alias: "Tamil",
    age: "1.1",
    date: "2004-05-01",

};


pub const TANG: ScriptCode = ScriptCode {
    name: "Tangut",
    code: "Tang",
    numeric: 520,
    alias: "Tangut",
    age: "9.0",
    date: "2016-12-05",

};


pub const TAVT: ScriptCode = ScriptCode {
    name: "Tai Viet",
    code: "Tavt",
    numeric: 359,
    alias: "Tai_Viet",
    age: "5.2",
    date: "2009-06-01",

};


pub const TELU: ScriptCode = ScriptCode {
    name: "Telugu",
    code: "Telu",
    numeric: 340,
    alias: "Telugu",
    age: "1.1",
    date: "2004-05-01",

};


pub const TENG: ScriptCode = ScriptCode {
    name: "Tengwar",
    code: "Teng",
    numeric: 290,
    alias: "",
    age: "",
    date: "2004-05-01",

};


pub const TFNG: ScriptCode = ScriptCode {
    name: "Tifinagh (Berber)",
    code: "Tfng",
    numeric: 120,
    alias: "Tifinagh",
    age: "4.1",
    date: "2006-06-21",

};


pub const TGLG: ScriptCode = ScriptCode {
    name: "Tagalog (Baybayin, Alibata)",
    code: "Tglg",
    numeric: 370,
    alias: "Tagalog",
    age: "3.2",
    date: "2009-02-23",

};


pub const THAA: ScriptCode = ScriptCode {
    name: "Thaana",
    code: "Thaa",
    numeric: 170,
    alias: "Thaana",
    age: "3.0",
    date: "2004-05-01",

};


pub const THAI: ScriptCode = ScriptCode {
    name: "Thai",
    code: "Thai",
    numeric: 352,
    alias: "Thai",
    age: "1.1",
    date: "2004-05-01",

};


pub const TIBT: ScriptCode = ScriptCode {
    name: "Tibetan",
    code: "Tibt",
    numeric: 330,
    alias: "Tibetan",
    age: "2.0",
    date: "2004-05-01",

};


pub const TIRH: ScriptCode = ScriptCode {
    name: "Tirhuta",
    code: "Tirh",
    numeric: 326,
    alias: "Tirhuta",
    age: "7.0",
    date: "2014-11-15",

};


pub const TNSA: ScriptCode = ScriptCode {
    name: "Tangsa",
    code: "Tnsa",
    numeric: 275,
    alias: "Tangsa",
    age: "14.0",
    date: "2021-02-17",

};


pub const TOTO: ScriptCode = ScriptCode {
    name: "Toto",
    code: "Toto",
    numeric: 294,
    alias: "Toto",
    age: "14.0",
    date: "2020-04-16",

};


pub const UGAR: ScriptCode = ScriptCode {
    name: "Ugaritic",
    code: "Ugar",
    numeric: 040,
    alias: "Ugaritic",
    age: "4.0",
    date: "2004-05-01",

};


pub const VAII: ScriptCode = ScriptCode {
    name: "Vai",
    code: "Vaii",
    numeric: 470,
    alias: "Vai",
    age: "5.1",
    date: "2007-07-02",

};


pub const VISP: ScriptCode = ScriptCode {
    name: "Visible Speech",
    code: "Visp",
    numeric: 280,
    alias: "",
    age: "",
    date: "2004-05-01",

};


pub const VITH: ScriptCode = ScriptCode {
    name: "Vithkuqi",
    code: "Vith",
    numeric: 228,
    alias: "Vithkuqi",
    age: "14.0",
    date: "2021-02-17",

};


pub const WARA: ScriptCode = ScriptCode {
    name: "Warang Citi (Varang Kshiti)",
    code: "Wara",
    numeric: 262,
    alias: "Warang_Citi",
    age: "7.0",
    date: "2014-11-15",

};


pub const WCHO: ScriptCode = ScriptCode {
    name: "Wancho",
    code: "Wcho",
    numeric: 283,
    alias: "Wancho",
    age: "12.0",
    date: "2017-07-26",

};


pub const WOLE: ScriptCode = ScriptCode {
    name: "Woleai",
    code: "Wole",
    numeric: 480,
    alias: "",
    age: "",
    date: "2010-12-21",

};


pub const XPEO: ScriptCode = ScriptCode {
    name: "Old Persian",
    code: "Xpeo",
    numeric: 030,
    alias: "Old_Persian",
    age: "4.1",
    date: "2006-06-21",

};


pub const XSUX: ScriptCode = ScriptCode {
    name: "Cuneiform, Sumero-Akkadian",
    code: "Xsux",
    numeric: 020,
    alias: "Cuneiform",
    age: "5.0",
    date: "2006-10-10",

};


pub const YEZI: ScriptCode = ScriptCode {
    name: "Yezidi",
    code: "Yezi",
    numeric: 192,
    alias: "Yezidi",
    age: "13.0",
    date: "2019-08-19",

};


pub const YIII: ScriptCode = ScriptCode {
    name: "Yi",
    code: "Yiii",
    numeric: 460,
    alias: "Yi",
    age: "3.0",
    date: "2004-05-01",

};


pub const ZANB: ScriptCode = ScriptCode {
    name: "Zanabazar Square (Zanabazarin Dörböljin Useg, Xewtee Dörböljin Bicig, Horizontal Square Script)",
    code: "Zanb",
    numeric: 339,
    alias: "Zanabazar_Square",
    age: "10.0",
    date: "2017-07-26",

};


pub const ZINH: ScriptCode = ScriptCode {
    name: "Code for inherited script",
    code: "Zinh",
    numeric: 994,
    alias: "Inherited",
    age: "",
    date: "2009-02-23",

};


pub const ZMTH: ScriptCode = ScriptCode {
    name: "Mathematical notation",
    code: "Zmth",
    numeric: 995,
    alias: "",
    age: "3.2",
    date: "2007-11-26",

};


pub const ZSYE: ScriptCode = ScriptCode {
    name: "Symbols (Emoji variant)",
    code: "Zsye",
    numeric: 993,
    alias: "",
    age: "6.0",
    date: "2015-12-16",

};


pub const ZSYM: ScriptCode = ScriptCode {
    name: "Symbols",
    code: "Zsym",
    numeric: 996,
    alias: "",
    age: "1.1",
    date: "2007-11-26",

};


pub const ZXXX: ScriptCode = ScriptCode {
    name: "Code for unwritten documents",
    code: "Zxxx",
    numeric: 997,
    alias: "",
    age: "",
    date: "2011-06-21",

};


pub const ZYYY: ScriptCode = ScriptCode {
    name: "Code for undetermined script",
    code: "Zyyy",
    numeric: 998,
    alias: "Common",
    age: "",
    date: "2004-05-29",

};


pub const ZZZZ: ScriptCode = ScriptCode {
    name: "Code for uncoded script",
    code: "Zzzz",
    numeric: 999,
    alias: "Unknown",
    age: "",
    date: "2006-10-10",

};


///ScriptCode map with  alpha4  str Code key
pub const ALL_MAP: Map<&str, ScriptCode> = phf_map! {
    
"adlm" => ADLM, 
"afak" => AFAK, 
"aghb" => AGHB, 
"ahom" => AHOM, 
"arab" => ARAB, 
"aran" => ARAN, 
"armi" => ARMI, 
"armn" => ARMN, 
"avst" => AVST, 
"bali" => BALI, 
"bamu" => BAMU, 
"bass" => BASS, 
"batk" => BATK, 
"beng" => BENG, 
"bhks" => BHKS, 
"blis" => BLIS, 
"bopo" => BOPO, 
"brah" => BRAH, 
"brai" => BRAI, 
"bugi" => BUGI, 
"buhd" => BUHD, 
"cakm" => CAKM, 
"cans" => CANS, 
"cari" => CARI, 
"cham" => CHAM, 
"cher" => CHER, 
"chrs" => CHRS, 
"cirt" => CIRT, 
"copt" => COPT, 
"cpmn" => CPMN, 
"cprt" => CPRT, 
"cyrl" => CYRL, 
"cyrs" => CYRS, 
"deva" => DEVA, 
"diak" => DIAK, 
"dogr" => DOGR, 
"dsrt" => DSRT, 
"dupl" => DUPL, 
"egyd" => EGYD, 
"egyh" => EGYH, 
"egyp" => EGYP, 
"elba" => ELBA, 
"elym" => ELYM, 
"ethi" => ETHI, 
"geok" => GEOK, 
"geor" => GEOR, 
"glag" => GLAG, 
"gong" => GONG, 
"gonm" => GONM, 
"goth" => GOTH, 
"gran" => GRAN, 
"grek" => GREK, 
"gujr" => GUJR, 
"guru" => GURU, 
"hanb" => HANB, 
"hang" => HANG, 
"hani" => HANI, 
"hano" => HANO, 
"hans" => HANS, 
"hant" => HANT, 
"hatr" => HATR, 
"hebr" => HEBR, 
"hira" => HIRA, 
"hluw" => HLUW, 
"hmng" => HMNG, 
"hmnp" => HMNP, 
"hrkt" => HRKT, 
"hung" => HUNG, 
"inds" => INDS, 
"ital" => ITAL, 
"jamo" => JAMO, 
"java" => JAVA, 
"jpan" => JPAN, 
"jurc" => JURC, 
"kali" => KALI, 
"kana" => KANA, 
"kawi" => KAWI, 
"khar" => KHAR, 
"khmr" => KHMR, 
"khoj" => KHOJ, 
"kitl" => KITL, 
"kits" => KITS, 
"knda" => KNDA, 
"kore" => KORE, 
"kpel" => KPEL, 
"kthi" => KTHI, 
"lana" => LANA, 
"laoo" => LAOO, 
"latf" => LATF, 
"latg" => LATG, 
"latn" => LATN, 
"leke" => LEKE, 
"lepc" => LEPC, 
"limb" => LIMB, 
"lina" => LINA, 
"linb" => LINB, 
"lisu" => LISU, 
"loma" => LOMA, 
"lyci" => LYCI, 
"lydi" => LYDI, 
"mahj" => MAHJ, 
"maka" => MAKA, 
"mand" => MAND, 
"mani" => MANI, 
"marc" => MARC, 
"maya" => MAYA, 
"medf" => MEDF, 
"mend" => MEND, 
"merc" => MERC, 
"mero" => MERO, 
"mlym" => MLYM, 
"modi" => MODI, 
"mong" => MONG, 
"moon" => MOON, 
"mroo" => MROO, 
"mtei" => MTEI, 
"mult" => MULT, 
"mymr" => MYMR, 
"nagm" => NAGM, 
"nand" => NAND, 
"narb" => NARB, 
"nbat" => NBAT, 
"newa" => NEWA, 
"nkdb" => NKDB, 
"nkgb" => NKGB, 
"nkoo" => NKOO, 
"nshu" => NSHU, 
"ogam" => OGAM, 
"olck" => OLCK, 
"orkh" => ORKH, 
"orya" => ORYA, 
"osge" => OSGE, 
"osma" => OSMA, 
"ougr" => OUGR, 
"palm" => PALM, 
"pauc" => PAUC, 
"pcun" => PCUN, 
"pelm" => PELM, 
"perm" => PERM, 
"phag" => PHAG, 
"phli" => PHLI, 
"phlp" => PHLP, 
"phlv" => PHLV, 
"phnx" => PHNX, 
"plrd" => PLRD, 
"piqd" => PIQD, 
"prti" => PRTI, 
"psin" => PSIN, 
"qaaa" => QAAA, 
"qabx" => QABX, 
"ranj" => RANJ, 
"rjng" => RJNG, 
"rohg" => ROHG, 
"roro" => RORO, 
"runr" => RUNR, 
"samr" => SAMR, 
"sara" => SARA, 
"sarb" => SARB, 
"saur" => SAUR, 
"sgnw" => SGNW, 
"shaw" => SHAW, 
"shrd" => SHRD, 
"shui" => SHUI, 
"sidd" => SIDD, 
"sind" => SIND, 
"sinh" => SINH, 
"sogd" => SOGD, 
"sogo" => SOGO, 
"sora" => SORA, 
"soyo" => SOYO, 
"sund" => SUND, 
"sunu" => SUNU, 
"sylo" => SYLO, 
"syrc" => SYRC, 
"syre" => SYRE, 
"syrj" => SYRJ, 
"syrn" => SYRN, 
"tagb" => TAGB, 
"takr" => TAKR, 
"tale" => TALE, 
"talu" => TALU, 
"taml" => TAML, 
"tang" => TANG, 
"tavt" => TAVT, 
"telu" => TELU, 
"teng" => TENG, 
"tfng" => TFNG, 
"tglg" => TGLG, 
"thaa" => THAA, 
"thai" => THAI, 
"tibt" => TIBT, 
"tirh" => TIRH, 
"tnsa" => TNSA, 
"toto" => TOTO, 
"ugar" => UGAR, 
"vaii" => VAII, 
"visp" => VISP, 
"vith" => VITH, 
"wara" => WARA, 
"wcho" => WCHO, 
"wole" => WOLE, 
"xpeo" => XPEO, 
"xsux" => XSUX, 
"yezi" => YEZI, 
"yiii" => YIII, 
"zanb" => ZANB, 
"zinh" => ZINH, 
"zmth" => ZMTH, 
"zsye" => ZSYE, 
"zsym" => ZSYM, 
"zxxx" => ZXXX, 
"zyyy" => ZYYY, 
"zzzz" => ZZZZ, 

};


///ScriptCode map with  3 len numeric str Code key
pub const NUMERIC_MAP: Map<&str, ScriptCode> = phf_map! {
    
"166" => ADLM, 
"439" => AFAK, 
"239" => AGHB, 
"338" => AHOM, 
"160" => ARAB, 
"161" => ARAN, 
"124" => ARMI, 
"230" => ARMN, 
"134" => AVST, 
"360" => BALI, 
"435" => BAMU, 
"259" => BASS, 
"365" => BATK, 
"325" => BENG, 
"334" => BHKS, 
"550" => BLIS, 
"285" => BOPO, 
"300" => BRAH, 
"570" => BRAI, 
"367" => BUGI, 
"372" => BUHD, 
"349" => CAKM, 
"440" => CANS, 
"201" => CARI, 
"358" => CHAM, 
"445" => CHER, 
"109" => CHRS, 
"291" => CIRT, 
"204" => COPT, 
"402" => CPMN, 
"403" => CPRT, 
"220" => CYRL, 
"221" => CYRS, 
"315" => DEVA, 
"342" => DIAK, 
"328" => DOGR, 
"250" => DSRT, 
"755" => DUPL, 
"070" => EGYD, 
"060" => EGYH, 
"050" => EGYP, 
"226" => ELBA, 
"128" => ELYM, 
"430" => ETHI, 
"241" => GEOK, 
"240" => GEOR, 
"225" => GLAG, 
"312" => GONG, 
"313" => GONM, 
"206" => GOTH, 
"343" => GRAN, 
"200" => GREK, 
"320" => GUJR, 
"310" => GURU, 
"503" => HANB, 
"286" => HANG, 
"500" => HANI, 
"371" => HANO, 
"501" => HANS, 
"502" => HANT, 
"127" => HATR, 
"125" => HEBR, 
"410" => HIRA, 
"080" => HLUW, 
"450" => HMNG, 
"451" => HMNP, 
"412" => HRKT, 
"176" => HUNG, 
"610" => INDS, 
"210" => ITAL, 
"284" => JAMO, 
"361" => JAVA, 
"413" => JPAN, 
"510" => JURC, 
"357" => KALI, 
"411" => KANA, 
"368" => KAWI, 
"305" => KHAR, 
"355" => KHMR, 
"322" => KHOJ, 
"505" => KITL, 
"288" => KITS, 
"345" => KNDA, 
"287" => KORE, 
"436" => KPEL, 
"317" => KTHI, 
"351" => LANA, 
"356" => LAOO, 
"217" => LATF, 
"216" => LATG, 
"215" => LATN, 
"364" => LEKE, 
"335" => LEPC, 
"336" => LIMB, 
"400" => LINA, 
"401" => LINB, 
"399" => LISU, 
"437" => LOMA, 
"202" => LYCI, 
"116" => LYDI, 
"314" => MAHJ, 
"366" => MAKA, 
"140" => MAND, 
"139" => MANI, 
"332" => MARC, 
"090" => MAYA, 
"265" => MEDF, 
"438" => MEND, 
"101" => MERC, 
"100" => MERO, 
"347" => MLYM, 
"324" => MODI, 
"145" => MONG, 
"218" => MOON, 
"264" => MROO, 
"337" => MTEI, 
"323" => MULT, 
"350" => MYMR, 
"295" => NAGM, 
"311" => NAND, 
"106" => NARB, 
"159" => NBAT, 
"333" => NEWA, 
"085" => NKDB, 
"420" => NKGB, 
"165" => NKOO, 
"499" => NSHU, 
"212" => OGAM, 
"261" => OLCK, 
"175" => ORKH, 
"327" => ORYA, 
"219" => OSGE, 
"260" => OSMA, 
"143" => OUGR, 
"126" => PALM, 
"263" => PAUC, 
"015" => PCUN, 
"016" => PELM, 
"227" => PERM, 
"331" => PHAG, 
"131" => PHLI, 
"132" => PHLP, 
"133" => PHLV, 
"115" => PHNX, 
"282" => PLRD, 
"293" => PIQD, 
"130" => PRTI, 
"103" => PSIN, 
"900" => QAAA, 
"949" => QABX, 
"303" => RANJ, 
"363" => RJNG, 
"167" => ROHG, 
"620" => RORO, 
"211" => RUNR, 
"123" => SAMR, 
"292" => SARA, 
"105" => SARB, 
"344" => SAUR, 
"095" => SGNW, 
"281" => SHAW, 
"319" => SHRD, 
"530" => SHUI, 
"302" => SIDD, 
"318" => SIND, 
"348" => SINH, 
"141" => SOGD, 
"142" => SOGO, 
"398" => SORA, 
"329" => SOYO, 
"362" => SUND, 
"274" => SUNU, 
"316" => SYLO, 
"135" => SYRC, 
"138" => SYRE, 
"137" => SYRJ, 
"136" => SYRN, 
"373" => TAGB, 
"321" => TAKR, 
"353" => TALE, 
"354" => TALU, 
"346" => TAML, 
"520" => TANG, 
"359" => TAVT, 
"340" => TELU, 
"290" => TENG, 
"120" => TFNG, 
"370" => TGLG, 
"170" => THAA, 
"352" => THAI, 
"330" => TIBT, 
"326" => TIRH, 
"275" => TNSA, 
"294" => TOTO, 
"040" => UGAR, 
"470" => VAII, 
"280" => VISP, 
"228" => VITH, 
"262" => WARA, 
"283" => WCHO, 
"480" => WOLE, 
"030" => XPEO, 
"020" => XSUX, 
"192" => YEZI, 
"460" => YIII, 
"339" => ZANB, 
"994" => ZINH, 
"995" => ZMTH, 
"993" => ZSYE, 
"996" => ZSYM, 
"997" => ZXXX, 
"998" => ZYYY, 
"999" => ZZZZ, 

};


///ALL names
pub const ALL_NAME: & [ & str] = &[
    
"Adlam",
"Afaka",
"Caucasian Albanian",
"Ahom, Tai Ahom",
"Arabic",
"Arabic (Nastaliq variant)",
"Imperial Aramaic",
"Armenian",
"Avestan",
"Balinese",
"Bamum",
"Bassa Vah",
"Batak",
"Bengali (Bangla)",
"Bhaiksuki",
"Blissymbols",
"Bopomofo",
"Brahmi",
"Braille",
"Buginese",
"Buhid",
"Chakma",
"Unified Canadian Aboriginal Syllabics",
"Carian",
"Cham",
"Cherokee",
"Chorasmian",
"Cirth",
"Coptic",
"Cypro-Minoan",
"Cypriot syllabary",
"Cyrillic",
"Cyrillic (Old Church Slavonic variant)",
"Devanagari (Nagari)",
"Dives Akuru",
"Dogra",
"Deseret (Mormon)",
"Duployan shorthand, Duployan stenography",
"Egyptian demotic",
"Egyptian hieratic",
"Egyptian hieroglyphs",
"Elbasan",
"Elymaic",
"Ethiopic (Geʻez)",
"Khutsuri (Asomtavruli and Nuskhuri)",
"Georgian (Mkhedruli and Mtavruli)",
"Glagolitic",
"Gunjala Gondi",
"Masaram Gondi",
"Gothic",
"Grantha",
"Greek",
"Gujarati",
"Gurmukhi",
"Han with Bopomofo (alias for Han + Bopomofo)",
"Hangul (Hangŭl, Hangeul)",
"Han (Hanzi, Kanji, Hanja)",
"Hanunoo (Hanunóo)",
"Han (Simplified variant)",
"Han (Traditional variant)",
"Hatran",
"Hebrew",
"Hiragana",
"Anatolian Hieroglyphs (Luwian Hieroglyphs, Hittite Hieroglyphs)",
"Pahawh Hmong",
"Nyiakeng Puachue Hmong",
"Japanese syllabaries (alias for Hiragana + Katakana)",
"Old Hungarian (Hungarian Runic)",
"Indus (Harappan)",
"Old Italic (Etruscan, Oscan, etc.)",
"Jamo (alias for Jamo subset of Hangul)",
"Javanese",
"Japanese (alias for Han + Hiragana + Katakana)",
"Jurchen",
"Kayah Li",
"Katakana",
"Kawi",
"Kharoshthi",
"Khmer",
"Khojki",
"Khitan large script",
"Khitan small script",
"Kannada",
"Korean (alias for Hangul + Han)",
"Kpelle",
"Kaithi",
"Tai Tham (Lanna)",
"Lao",
"Latin (Fraktur variant)",
"Latin (Gaelic variant)",
"Latin",
"Leke",
"Lepcha (Róng)",
"Limbu",
"Linear A",
"Linear B",
"Lisu (Fraser)",
"Loma",
"Lycian",
"Lydian",
"Mahajani",
"Makasar",
"Mandaic, Mandaean",
"Manichaean",
"Marchen",
"Mayan hieroglyphs",
"Medefaidrin (Oberi Okaime, Oberi Ɔkaimɛ)",
"Mende Kikakui",
"Meroitic Cursive",
"Meroitic Hieroglyphs",
"Malayalam",
"Modi, Moḍī",
"Mongolian",
"Moon (Moon code, Moon script, Moon type)",
"Mro, Mru",
"Meitei Mayek (Meithei, Meetei)",
"Multani",
"Myanmar (Burmese)",
"Nag Mundari",
"Nandinagari",
"Old North Arabian (Ancient North Arabian)",
"Nabataean",
"Newa, Newar, Newari, Nepāla lipi",
"Naxi Dongba (na²¹ɕi³³ to³³ba²¹, Nakhi Tomba)",
"Naxi Geba (na²¹ɕi³³ gʌ²¹ba²¹, 'Na-'Khi ²Ggŏ-¹baw, Nakhi Geba)",
"N’Ko",
"Nüshu",
"Ogham",
"Ol Chiki (Ol Cemet’, Ol, Santali)",
"Old Turkic, Orkhon Runic",
"Oriya (Odia)",
"Osage",
"Osmanya",
"Old Uyghur",
"Palmyrene",
"Pau Cin Hau",
"Proto-Cuneiform",
"Proto-Elamite",
"Old Permic",
"Phags-pa",
"Inscriptional Pahlavi",
"Psalter Pahlavi",
"Book Pahlavi",
"Phoenician",
"Miao (Pollard)",
"Klingon (KLI pIqaD)",
"Inscriptional Parthian",
"Proto-Sinaitic",
"Reserved for private use (start)",
"Reserved for private use (end)",
"Ranjana",
"Rejang (Redjang, Kaganga)",
"Hanifi Rohingya",
"Rongorongo",
"Runic",
"Samaritan",
"Sarati",
"Old South Arabian",
"Saurashtra",
"SignWriting",
"Shavian (Shaw)",
"Sharada, Śāradā",
"Shuishu",
"Siddham, Siddhaṃ, Siddhamātṛkā",
"Khudawadi, Sindhi",
"Sinhala",
"Sogdian",
"Old Sogdian",
"Sora Sompeng",
"Soyombo",
"Sundanese",
"Sunuwar",
"Syloti Nagri",
"Syriac",
"Syriac (Estrangelo variant)",
"Syriac (Western variant)",
"Syriac (Eastern variant)",
"Tagbanwa",
"Takri, Ṭākrī, Ṭāṅkrī",
"Tai Le",
"New Tai Lue",
"Tamil",
"Tangut",
"Tai Viet",
"Telugu",
"Tengwar",
"Tifinagh (Berber)",
"Tagalog (Baybayin, Alibata)",
"Thaana",
"Thai",
"Tibetan",
"Tirhuta",
"Tangsa",
"Toto",
"Ugaritic",
"Vai",
"Visible Speech",
"Vithkuqi",
"Warang Citi (Varang Kshiti)",
"Wancho",
"Woleai",
"Old Persian",
"Cuneiform, Sumero-Akkadian",
"Yezidi",
"Yi",
"Zanabazar Square (Zanabazarin Dörböljin Useg, Xewtee Dörböljin Bicig, Horizontal Square Script)",
"Code for inherited script",
"Mathematical notation",
"Symbols (Emoji variant)",
"Symbols",
"Code for unwritten documents",
"Code for undetermined script",
"Code for uncoded script",

];


///ALL codes
pub const ALL_CODE: & [ & str] = &[
    
"Adlm",
"Afak",
"Aghb",
"Ahom",
"Arab",
"Aran",
"Armi",
"Armn",
"Avst",
"Bali",
"Bamu",
"Bass",
"Batk",
"Beng",
"Bhks",
"Blis",
"Bopo",
"Brah",
"Brai",
"Bugi",
"Buhd",
"Cakm",
"Cans",
"Cari",
"Cham",
"Cher",
"Chrs",
"Cirt",
"Copt",
"Cpmn",
"Cprt",
"Cyrl",
"Cyrs",
"Deva",
"Diak",
"Dogr",
"Dsrt",
"Dupl",
"Egyd",
"Egyh",
"Egyp",
"Elba",
"Elym",
"Ethi",
"Geok",
"Geor",
"Glag",
"Gong",
"Gonm",
"Goth",
"Gran",
"Grek",
"Gujr",
"Guru",
"Hanb",
"Hang",
"Hani",
"Hano",
"Hans",
"Hant",
"Hatr",
"Hebr",
"Hira",
"Hluw",
"Hmng",
"Hmnp",
"Hrkt",
"Hung",
"Inds",
"Ital",
"Jamo",
"Java",
"Jpan",
"Jurc",
"Kali",
"Kana",
"Kawi",
"Khar",
"Khmr",
"Khoj",
"Kitl",
"Kits",
"Knda",
"Kore",
"Kpel",
"Kthi",
"Lana",
"Laoo",
"Latf",
"Latg",
"Latn",
"Leke",
"Lepc",
"Limb",
"Lina",
"Linb",
"Lisu",
"Loma",
"Lyci",
"Lydi",
"Mahj",
"Maka",
"Mand",
"Mani",
"Marc",
"Maya",
"Medf",
"Mend",
"Merc",
"Mero",
"Mlym",
"Modi",
"Mong",
"Moon",
"Mroo",
"Mtei",
"Mult",
"Mymr",
"Nagm",
"Nand",
"Narb",
"Nbat",
"Newa",
"Nkdb",
"Nkgb",
"Nkoo",
"Nshu",
"Ogam",
"Olck",
"Orkh",
"Orya",
"Osge",
"Osma",
"Ougr",
"Palm",
"Pauc",
"Pcun",
"Pelm",
"Perm",
"Phag",
"Phli",
"Phlp",
"Phlv",
"Phnx",
"Plrd",
"Piqd",
"Prti",
"Psin",
"Qaaa",
"Qabx",
"Ranj",
"Rjng",
"Rohg",
"Roro",
"Runr",
"Samr",
"Sara",
"Sarb",
"Saur",
"Sgnw",
"Shaw",
"Shrd",
"Shui",
"Sidd",
"Sind",
"Sinh",
"Sogd",
"Sogo",
"Sora",
"Soyo",
"Sund",
"Sunu",
"Sylo",
"Syrc",
"Syre",
"Syrj",
"Syrn",
"Tagb",
"Takr",
"Tale",
"Talu",
"Taml",
"Tang",
"Tavt",
"Telu",
"Teng",
"Tfng",
"Tglg",
"Thaa",
"Thai",
"Tibt",
"Tirh",
"Tnsa",
"Toto",
"Ugar",
"Vaii",
"Visp",
"Vith",
"Wara",
"Wcho",
"Wole",
"Xpeo",
"Xsux",
"Yezi",
"Yiii",
"Zanb",
"Zinh",
"Zmth",
"Zsye",
"Zsym",
"Zxxx",
"Zyyy",
"Zzzz",

];


///ALL the ScriptCodes struct
pub const ALL_NUMERIC: & [i32] = &[

166,
439,
239,
338,
160,
161,
124,
230,
134,
360,
435,
259,
365,
325,
334,
550,
285,
300,
570,
367,
372,
349,
440,
201,
358,
445,
109,
291,
204,
402,
403,
220,
221,
315,
342,
328,
250,
755,
070,
060,
050,
226,
128,
430,
241,
240,
225,
312,
313,
206,
343,
200,
320,
310,
503,
286,
500,
371,
501,
502,
127,
125,
410,
080,
450,
451,
412,
176,
610,
210,
284,
361,
413,
510,
357,
411,
368,
305,
355,
322,
505,
288,
345,
287,
436,
317,
351,
356,
217,
216,
215,
364,
335,
336,
400,
401,
399,
437,
202,
116,
314,
366,
140,
139,
332,
090,
265,
438,
101,
100,
347,
324,
145,
218,
264,
337,
323,
350,
295,
311,
106,
159,
333,
085,
420,
165,
499,
212,
261,
175,
327,
219,
260,
143,
126,
263,
015,
016,
227,
331,
131,
132,
133,
115,
282,
293,
130,
103,
900,
949,
303,
363,
167,
620,
211,
123,
292,
105,
344,
095,
281,
319,
530,
302,
318,
348,
141,
142,
398,
329,
362,
274,
316,
135,
138,
137,
136,
373,
321,
353,
354,
346,
520,
359,
340,
290,
120,
370,
170,
352,
330,
326,
275,
294,
040,
470,
280,
228,
262,
283,
480,
030,
020,
192,
460,
339,
994,
995,
993,
996,
997,
998,
999,

];


///ALL the ScriptCodes struct
pub const ALL_NUMERIC_STR: & [ & str] = &[

"166",
"439",
"239",
"338",
"160",
"161",
"124",
"230",
"134",
"360",
"435",
"259",
"365",
"325",
"334",
"550",
"285",
"300",
"570",
"367",
"372",
"349",
"440",
"201",
"358",
"445",
"109",
"291",
"204",
"402",
"403",
"220",
"221",
"315",
"342",
"328",
"250",
"755",
"070",
"060",
"050",
"226",
"128",
"430",
"241",
"240",
"225",
"312",
"313",
"206",
"343",
"200",
"320",
"310",
"503",
"286",
"500",
"371",
"501",
"502",
"127",
"125",
"410",
"080",
"450",
"451",
"412",
"176",
"610",
"210",
"284",
"361",
"413",
"510",
"357",
"411",
"368",
"305",
"355",
"322",
"505",
"288",
"345",
"287",
"436",
"317",
"351",
"356",
"217",
"216",
"215",
"364",
"335",
"336",
"400",
"401",
"399",
"437",
"202",
"116",
"314",
"366",
"140",
"139",
"332",
"090",
"265",
"438",
"101",
"100",
"347",
"324",
"145",
"218",
"264",
"337",
"323",
"350",
"295",
"311",
"106",
"159",
"333",
"085",
"420",
"165",
"499",
"212",
"261",
"175",
"327",
"219",
"260",
"143",
"126",
"263",
"015",
"016",
"227",
"331",
"131",
"132",
"133",
"115",
"282",
"293",
"130",
"103",
"900",
"949",
"303",
"363",
"167",
"620",
"211",
"123",
"292",
"105",
"344",
"095",
"281",
"319",
"530",
"302",
"318",
"348",
"141",
"142",
"398",
"329",
"362",
"274",
"316",
"135",
"138",
"137",
"136",
"373",
"321",
"353",
"354",
"346",
"520",
"359",
"340",
"290",
"120",
"370",
"170",
"352",
"330",
"326",
"275",
"294",
"040",
"470",
"280",
"228",
"262",
"283",
"480",
"030",
"020",
"192",
"460",
"339",
"994",
"995",
"993",
"996",
"997",
"998",
"999",

];


///ALL the ScriptCodes struct
pub const ALL: & [ScriptCode] = &[

ADLM,
AFAK,
AGHB,
AHOM,
ARAB,
ARAN,
ARMI,
ARMN,
AVST,
BALI,
BAMU,
BASS,
BATK,
BENG,
BHKS,
BLIS,
BOPO,
BRAH,
BRAI,
BUGI,
BUHD,
CAKM,
CANS,
CARI,
CHAM,
CHER,
CHRS,
CIRT,
COPT,
CPMN,
CPRT,
CYRL,
CYRS,
DEVA,
DIAK,
DOGR,
DSRT,
DUPL,
EGYD,
EGYH,
EGYP,
ELBA,
ELYM,
ETHI,
GEOK,
GEOR,
GLAG,
GONG,
GONM,
GOTH,
GRAN,
GREK,
GUJR,
GURU,
HANB,
HANG,
HANI,
HANO,
HANS,
HANT,
HATR,
HEBR,
HIRA,
HLUW,
HMNG,
HMNP,
HRKT,
HUNG,
INDS,
ITAL,
JAMO,
JAVA,
JPAN,
JURC,
KALI,
KANA,
KAWI,
KHAR,
KHMR,
KHOJ,
KITL,
KITS,
KNDA,
KORE,
KPEL,
KTHI,
LANA,
LAOO,
LATF,
LATG,
LATN,
LEKE,
LEPC,
LIMB,
LINA,
LINB,
LISU,
LOMA,
LYCI,
LYDI,
MAHJ,
MAKA,
MAND,
MANI,
MARC,
MAYA,
MEDF,
MEND,
MERC,
MERO,
MLYM,
MODI,
MONG,
MOON,
MROO,
MTEI,
MULT,
MYMR,
NAGM,
NAND,
NARB,
NBAT,
NEWA,
NKDB,
NKGB,
NKOO,
NSHU,
OGAM,
OLCK,
ORKH,
ORYA,
OSGE,
OSMA,
OUGR,
PALM,
PAUC,
PCUN,
PELM,
PERM,
PHAG,
PHLI,
PHLP,
PHLV,
PHNX,
PLRD,
PIQD,
PRTI,
PSIN,
QAAA,
QABX,
RANJ,
RJNG,
ROHG,
RORO,
RUNR,
SAMR,
SARA,
SARB,
SAUR,
SGNW,
SHAW,
SHRD,
SHUI,
SIDD,
SIND,
SINH,
SOGD,
SOGO,
SORA,
SOYO,
SUND,
SUNU,
SYLO,
SYRC,
SYRE,
SYRJ,
SYRN,
TAGB,
TAKR,
TALE,
TALU,
TAML,
TANG,
TAVT,
TELU,
TENG,
TFNG,
TGLG,
THAA,
THAI,
TIBT,
TIRH,
TNSA,
TOTO,
UGAR,
VAII,
VISP,
VITH,
WARA,
WCHO,
WOLE,
XPEO,
XSUX,
YEZI,
YIII,
ZANB,
ZINH,
ZMTH,
ZSYE,
ZSYM,
ZXXX,
ZYYY,
ZZZZ,

];

