use support::*;
use TokenizerPack::support::*;

pub fn missing_operand( x: i32, y: i32 ) -> CompilerErrors {
	let err = ParserErrors::MissingOperand{ x, y };
    CompilerErrors::ParserError{ err }
}

pub fn expected_token( x: i32, y: i32, token_type: TokenType ) -> CompilerErrors {
    let token = match token_type {
        TokenType::TSemicolom => { ";".to_string() },
        TokenType::TColon     => { ":".to_string() },
        TokenType::TComma     => { ",".to_string() },
        TokenType::TObr       => { "[".to_string() },
        TokenType::TOp        => { "(".to_string() },
        TokenType::TCbr       => { "]".to_string() },
        TokenType::TCp        => { ")".to_string() },
        TokenType::TPoint     => { ".".to_string() },
        TokenType::TBegin     => { "begin".to_string() },
        TokenType::TEnd       => { "end".to_string() },
        TokenType::TThen      => { "then".to_string() },
        TokenType::TDo        => { "do".to_string() },
        TokenType::TOf        => { "of".to_string() },
        TokenType::TId        => { "идентификатор".to_string() },
        _                     => { "".to_string() }
    };
    let err = ParserErrors::ExpectedToken{ x, y, token };
    CompilerErrors::ParserError{ err }
}

macro_rules! parse_bin {
    ($self:ident, $next_func:ident, [$($var: path),*]) => ({
        let mut e = try!( $self.$next_func() );

        let mut t = $self.tokenizer.current.clone();

        while match t.token_type {
            $($var)|* => true,
            _ => false
        } {
            try!( $self.tokenizer.my_next() );
            let right = try!( $self.$next_func() );

            let old_e = e;
            e = Box::new(BinNode::new(t));

            e.add_child(right);
            e.add_child(old_e);
            t = $self.tokenizer.current.clone();
        }

        Ok(e)
    })
}

macro_rules! parse {
    ($self:ident, $curr_t: expr, [$($var: path => $next_func:block),*]) => ({
        let curr_t = $curr_t;

        match curr_t.token_type {
            $($var => {
                Some($next_func)
            },)*
            _ => None
        }
    })
}

macro_rules! parse_simple {
    ($self:ident, $curr_t: expr, [ $var: path => $next_func:block ], $experted_func:ident) => ({
        let curr_t = $curr_t;
        match parse!($self, curr_t, [$var => $next_func]) {
            Some(res) => res,
            None => { return Err($experted_func(curr_t.coords.x, curr_t.coords.y, $var)); }
        }
    })
}

macro_rules! break_if {
    ($($part: ident).* == [$($var: path),*]) => ({
        if true_if!($($part).* == [$($var),*]) { break }
    })
}

macro_rules! true_if {
    ($($part: ident).* == [$($var: path),*]) => ({
        match $($part).* {
            $($var)|* => true,
            _ => { false }
        }
    })
}


macro_rules! check_token {
    ($self:ident, $var: path $(,$opt: path)*) => ({
        let t = $self.tokenizer.current.clone();
        try!( $self.tokenizer.my_next() );
        match t.token_type {
            $var => {},
            $($opt => {},)*
            _ => { return Err(expected_token(t.coords.x, t.coords.y, $var)); }
        }
    })
}