pub enum CompilerErrors {
    TokenizerError { err: TokenizerErrors },
    ParserError { err: ParserErrors }
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
	EmptyFile{ x: i32, y: i32 },
	MissingOperand{ x: i32, y: i32 },
    ExpectedToken{ x: i32, y: i32, token: String },
}