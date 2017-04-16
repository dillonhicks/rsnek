//use phf;
//
//use token::Tag;
//
//static PYTHON36_KEYWORDS: phf::Set<&'static str> = phf_set! {
//        "False",
//        "None",
//        "True",
//        "and",
//        "as",
//        "assert",
//        "break",
//        "class",
//        "continue",
//        "def",
//        "del",
//        "elif",
//        "else",
//        "except",
//        "finally",
//        "for",
//        "from",
//        "global",
//        "if",
//        "import",
//        "in",
//        "is",
//        "lambda",
//        "nonlocal",
//        "not",
//        "or",
//        "pass",
//        "raise",
//        "return",
//        "try",
//        "while",
//        "with",
//        "yield"
//};
//
//pub fn is_keyword(keyword: &str) -> bool {
//    PYTHON36_KEYWORDS.contains(keyword)
//}
//
