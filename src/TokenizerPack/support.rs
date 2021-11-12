use std::f64;
use std::i64;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, name_by_field)]
pub enum TokenType {
    TColon,
    TDouble,
    THex,
    TMinus,
    TPlus,
    TId,
    TPoint,
    TInt,
    TSemicolom,
    TComma,
    TAssign,
    TRange,
    TMul,
    TShare,
    TLiteral,
    TString,

    TMinAssign,
    TMulAssign,
    TShareAssign,
    TPlsAssign,

    TOp,
    TCp,
    TObr,
    TCbr,

    TGe,
    TLe,
    TGt,
    TLt,
    TEq,
    TNe,

    TExp,
    TKeyword,

    TDog,
    TLid,
    TOctal,
    TGrill,
    TChar,
    THexChar,
    TOctalChar,
    TBin,
    TBinChar,
    TEof,

    TAnd,
    TArray,
    TBegin,
    TCase,
    TConst,
    TDiv,
    TDo,
    TDownto,
    TElse,
    TEnd,
    TFile,
    TFor,
    TFunction,
    TGoto,
    TIf,
    TIn,
    TLabel,
    TMod,
    TNil,
    TNot,
    TOf,
    TOr,
    TPacked,
    TProcedure,
    TProgram,
    TRecord,
    TRepeat,
    TSet,
    TThen,
    TTo,
    TType,
    TUntil,
    TVar,
    TWhile,
    TWith,
    TIntegerType,
    TDoubleType,
    TCharType,
    TForward,
    TContinue,
    TBreak,
    TWriteln,
}

lazy_static! {
    pub static ref TYPE_BY_STATE: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("int".to_string(),          TokenType::TInt);
        m.insert("double".to_string(),       TokenType::TDouble);
        m.insert("hex".to_string(),          TokenType::THex);
        m.insert("bin".to_string(),          TokenType::TBin);
        m.insert("minus".to_string(),        TokenType::TMinus);
        m.insert("plus".to_string(),         TokenType::TPlus);
        m.insert("id".to_string(),           TokenType::TId);
        m.insert("point".to_string(),        TokenType::TPoint);
        m.insert("colon".to_string(),        TokenType::TColon);
        m.insert("semicolon".to_string(),    TokenType::TSemicolom);
        m.insert("eq".to_string(),           TokenType::TEq);
        m.insert("assign".to_string(),       TokenType::TAssign);
        m.insert("pls_assign".to_string(),   TokenType::TPlsAssign);
        m.insert("min_assign".to_string(),   TokenType::TMinAssign);
        m.insert("mul_assign".to_string(),   TokenType::TMulAssign);
        m.insert("share_assign".to_string(), TokenType::TShareAssign);
        m.insert("comma".to_string(),        TokenType::TComma);
        m.insert("range".to_string(),        TokenType::TRange);
        m.insert("mul".to_string(),          TokenType::TMul);
        m.insert("share".to_string(),        TokenType::TShare);
        m.insert("literal".to_string(),      TokenType::TLiteral);
        m.insert("string".to_string(),       TokenType::TString);

        m.insert("op".to_string(),           TokenType::TOp);
        m.insert("cp".to_string(),           TokenType::TCp);
        m.insert("obr".to_string(),          TokenType::TObr);
        m.insert("cbr".to_string(),          TokenType::TCbr);

        m.insert("ge".to_string(),           TokenType::TGe);
        m.insert("gt".to_string(),           TokenType::TGt);
        m.insert("eq".to_string(),           TokenType::TEq);
        m.insert("le".to_string(),           TokenType::TLe);
        m.insert("lt".to_string(),           TokenType::TLt);
        m.insert("ne".to_string(),           TokenType::TNe);

        m.insert("exp".to_string(),          TokenType::TExp);
        m.insert("key_word".to_string(),     TokenType::TKeyword);

        m.insert("octal".to_string(),        TokenType::TOctal);
        m.insert("lid".to_string(),          TokenType::TLid);
        m.insert("dog".to_string(),          TokenType::TDog);
        m.insert("grill".to_string(),        TokenType::TGrill);
        m.insert("char".to_string(),         TokenType::TChar);
        m.insert("hex_char".to_string(),     TokenType::THexChar);
        m.insert("octal_char".to_string(),   TokenType::TOctalChar);
        m.insert("bin_char".to_string(),     TokenType::TBinChar);

        m.insert("and".to_string(),          TokenType::TAnd);
        m.insert("array".to_string(),        TokenType::TArray);
        m.insert("begin".to_string(),        TokenType::TBegin);
        m.insert("case".to_string(),         TokenType::TCase);
        m.insert("const".to_string(),        TokenType::TConst);
        m.insert("div".to_string(),          TokenType::TDiv);
        m.insert("do".to_string(),           TokenType::TDo);
        m.insert("downto".to_string(),       TokenType::TDownto);
        m.insert("else".to_string(),         TokenType::TElse);
        m.insert("end".to_string(),          TokenType::TEnd);
        m.insert("file".to_string(),         TokenType::TFile);
        m.insert("for".to_string(),          TokenType::TFor);
        m.insert("function".to_string(),     TokenType::TFunction);
        m.insert("goto".to_string(),         TokenType::TGoto);
        m.insert("if".to_string(),           TokenType::TIf);
        m.insert("in".to_string(),           TokenType::TIn);
        m.insert("label".to_string(),        TokenType::TLabel);
        m.insert("mod".to_string(),          TokenType::TMod);
        m.insert("nil".to_string(),          TokenType::TNil);
        m.insert("not".to_string(),          TokenType::TNot);
        m.insert("of".to_string(),           TokenType::TOf);
        m.insert("or".to_string(),           TokenType::TOr);
        m.insert("packed".to_string(),       TokenType::TPacked);
        m.insert("procedure".to_string(),    TokenType::TProcedure);
        m.insert("program".to_string(),      TokenType::TProgram);
        m.insert("record".to_string(),       TokenType::TRecord);
        m.insert("repeat".to_string(),       TokenType::TRepeat);
        m.insert("set".to_string(),          TokenType::TSet);
        m.insert("then".to_string(),         TokenType::TThen);
        m.insert("to".to_string(),           TokenType::TTo);
        m.insert("type".to_string(),         TokenType::TType);
        m.insert("until".to_string(),        TokenType::TUntil);
        m.insert("var".to_string(),          TokenType::TVar);
        m.insert("while".to_string(),        TokenType::TWhile);
        m.insert("with".to_string(),         TokenType::TWith);
        m.insert("forward".to_string(),      TokenType::TForward);

        m.insert("integer_type".to_string(), TokenType::TIntegerType);
        m.insert("double_type".to_string(),  TokenType::TDoubleType);
        m.insert("char_type".to_string(),    TokenType::TCharType);

        m.insert("continue".to_string(),     TokenType::TContinue);
        m.insert("break".to_string(),        TokenType::TBreak);

        m.insert("writeln".to_string(),      TokenType::TWriteln);

        m
    };
}

pub static KEY_WORDS: &'static [&'static str] = &[
    "and",
    "array",
    "begin",
    "case",
    "const",
    "div",
    "do",
    "downto",
    "else",
    "end",
    "file",
    "for",
    "function",
    "goto",
    "if",
    "in",
    "label",
    "mod",
    "nil",
    "not",
    "of",
    "or",
    "packed",
    "procedure",
    "program",
    "record",
    "repeat",
    "set",
    "then",
    "to",
    "type",
    "until",
    "var",
    "while",
    "with",
    "integer",
    "double",
    "char",
    "forward",
    "continue",
    "break",
    "writeln",
];

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub enum Value {
    Int { v: i64 },
    Double { v: f64 },
    Str { v: String },
}

impl Value {
    pub fn as_string(&self) -> String {
        match self {
            &Value::Str { ref v } => v.clone(),
            _ => "".to_string(),
        }
    }

    pub fn as_int(&self) -> i64 {
        match *self {
            Value::Int { v } => v,
            _ => 0,
        }
    }

    pub fn as_double(&self) -> f64 {
        match *self {
            Value::Double { v } => v,
            _ => 0.0,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Int { ref v } => write!(f, "{:<25}", v.clone()),
            &Value::Double { ref v } => write!(f, "{:<25}", v.clone()),
            &Value::Str { ref v } => write!(f, "{:<25}", v.clone()),
        }
    }
}
