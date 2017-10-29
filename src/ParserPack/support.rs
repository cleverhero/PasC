use support::*;

pub fn missing_closing_bracket( x: i32, y: i32 ) -> CompilerErrors {
	let err = ParserErrors::MissingClosingBracket{ x, y };
    CompilerErrors::ParserError{ err }
}
pub fn missing_operand( x: i32, y: i32 ) -> CompilerErrors {
	let err = ParserErrors::MissingOperand{ x, y };
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
            match $self.tokenizer.next() {
                Some(res) => match res {
                    Ok(_token) => {},
                    Err(err) => return Err(CompilerErrors::TokenizerError{err})
                }
                None => { }
            };
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
        match $self.tokenizer.next() {
            Some(res) => match res {
                Ok(_token) => {},
                Err(err) => return Err(CompilerErrors::TokenizerError{err})
            }
            None => { }
        };

        match t.token_type {
            $($var => $self.$next_func(&t),)*
            _ => return Err($func(t.coords.x, t.coords.y))
        }
    })
}