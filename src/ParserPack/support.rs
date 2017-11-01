use support::*;

pub fn missing_closing_bracket( x: i32, y: i32 ) -> CompilerErrors {
	let err = ParserErrors::MissingClosingBracket{ x, y };
    CompilerErrors::ParserError{ err }
}
pub fn missing_operand( x: i32, y: i32 ) -> CompilerErrors {
	let err = ParserErrors::MissingOperand{ x, y };
    CompilerErrors::ParserError{ err }
}

pub fn expected_id( x: i32, y: i32 ) -> CompilerErrors {
    let err = ParserErrors::ExpectedId{ x, y };
    CompilerErrors::ParserError{ err }
}

pub fn expected_semicolom( x: i32, y: i32 ) -> CompilerErrors {
    let err = ParserErrors::ExpectedSemicolom{ x, y };
    CompilerErrors::ParserError{ err }
}

pub fn expected_comma( x: i32, y: i32 ) -> CompilerErrors {
    let err = ParserErrors::ExpectedComma{ x, y };
    CompilerErrors::ParserError{ err }
}

pub fn expected_colon( x: i32, y: i32 ) -> CompilerErrors {
    let err = ParserErrors::ExpectedColon{ x, y };
    CompilerErrors::ParserError{ err }
}

pub fn expected_begin( x: i32, y: i32 ) -> CompilerErrors {
    let err = ParserErrors::ExpectedBegin{ x, y };
    CompilerErrors::ParserError{ err }
}

pub fn expected_point( x: i32, y: i32 ) -> CompilerErrors {
    let err = ParserErrors::ExpectedPoint{ x, y };
    CompilerErrors::ParserError{ err }
}

macro_rules! parse_bin {
    ($self:ident, $next_func:ident, [$($var: path),*]) => ({
        let mut e = match $self.$next_func() {
            Ok(val) => val,
            Err(err) => return Err(err)
        };

        let mut t = $self.tokenizer.current.clone();

        while match t.token_type {
            $($var)|* => true,
            _ => false
        } {
            try!( $self.tokenizer.my_next() );
            let right = match $self.$next_func() {
                Ok(val) => val,
                Err(err) => return Err(err)
            };

            let old_e = e;
            e = Box::new(BinNode::new(t));

            e.add_child(right);
            e.add_child(old_e);
            t = $self.tokenizer.current.clone();
        }

        Ok(e)
    })
}

macro_rules! before_parse {
    ($self:ident, $func: ident, [$($var: path => $next_func:ident),*]) => ({
        let t = $self.tokenizer.current.clone();
        try!( $self.tokenizer.my_next() );

        match t.token_type {
            $($var => $self.$next_func(&t),)*
            _ => return Err($func(t.coords.x, t.coords.y))
        }
    });
    ($self:ident, $default: expr, [$($var: path => $next_func:ident),*]) => ({
        let t = $self.tokenizer.current.clone();
        try!( $self.tokenizer.my_next() );

        match t.token_type {
            $($var => $self.$next_func(&t),)*
            _ => return Ok($default)
        }
    })
}

macro_rules! behind_parse {
    ($self:ident, $t: expr, [$($var: path => $next_func:ident),*]) => ({
        let t = $self.tokenizer.current.clone();

        match t.token_type {
            $($var => Some($self.$next_func(&$t)),)*
            _ => {None}
        }
    })
}

macro_rules! break_if {
    ($($part: ident).* = [$($var: path),*]) => ({
        match $($part).* {
            $($var)|* => break,
            _ => {}
        }
    })
}

macro_rules! get_err_or_none {
    ($($part: ident).* $func:pat) => ({
        match $($part).* {
            Some(res) => match res {
                Ok(_token) => {},
                Err(err) => return Err(CompilerErrors::TokenizerError{err})
            }
            None => { }
        };     
    })
}