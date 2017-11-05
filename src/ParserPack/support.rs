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
        TokenType::TCbr       => { "]".to_string() },
        TokenType::TCp        => { ")".to_string() },
        TokenType::TPoint     => { ".".to_string() },
        TokenType::TBegin     => { "begin".to_string() },
        TokenType::TEnd       => { "end".to_string() },
        TokenType::TId        => { "идентификатор".to_string() },
        _                     => { "".to_string() }
    };
    let err = ParserErrors::ExpectedToken{ x, y, token };
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
    ($self:ident, [$($var: path => $next_func:ident),*]) => ({
        let t = $self.tokenizer.current.clone();
        
        match t.token_type {
            $($var => {
                try!( $self.tokenizer.my_next() );
                Some($self.$next_func(&t))
            },)*
            _ => None
        }
    });
    ($self:ident, $experted_func:ident, $var: path => $next_func:ident) => ({
        let t = $self.tokenizer.current.clone();
        match before_parse!($self, [$var => $next_func]) {
            Some(res) => res,
            None => { return Err($experted_func(t.coords.x, t.coords.y, $var)); }
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

macro_rules! check_token {
    ($self:ident, $var: path) => ({
        let t = $self.tokenizer.current.clone();
        try!( $self.tokenizer.my_next() );
        match t.token_type {
            $var => {},
            _ => { return Err(expected_token(t.coords.x, t.coords.y, $var)); }
        }
    })
}