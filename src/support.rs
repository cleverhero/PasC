use ParserPack::Types::support::*;

pub enum CompilerErrors {
    TokenizerError { err: TokenizerErrors },
    ParserError { err: ParserErrors },
    SemanticError { err: SemanticErrors }
}

pub enum TokenizerErrors {
    ErrorInInteger{ x: i32, y: i32 },
    ErrorInHex{ x: i32, y: i32 },
    ErrorInBin{ x: i32, y: i32 },
    ErrorInOctal{ x: i32, y: i32 },
    ErrorInDouble{ x: i32, y: i32 },
    ErrorInExp{ x: i32, y: i32 },
    UnknownCharCode{ x: i32, y: i32 },
    SimpleError{ x: i32, y: i32 }
}

pub enum ParserErrors {
	MissingOperand{ x: i32, y: i32 },
    ExpectedToken{ x: i32, y: i32, token: String },
}

#[derive(Debug)]
pub enum SemanticErrors {
    DuplicateIdentifier{ name: String },
    UnknownIdentifier{ name: String },
    NotAFunction{ name: String },
    UnknownOverride{ name: String, sign: String },
    CastError { this: String, other: String },
    ErrorInUnarOperation { name: String, op: UnarOperation },
    ErrorInBinOperation { left: String, right: String, op: BinOperation },
    ErrorInForwardDecl { name: String, sign: String },
    OtherError{ msg: String },
}

impl From<SemanticErrors> for CompilerErrors {
    fn from(err: SemanticErrors) -> Self {
        CompilerErrors::SemanticError{ err }
    }
}

impl From<ParserErrors> for CompilerErrors {
    fn from(err: ParserErrors) -> Self {
        CompilerErrors::ParserError{ err }
    }
}