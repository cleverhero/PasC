use support::*;

pub fn missing_operand(x: i32, y: i32) -> CompilerErrors {
    let err = ParserErrors::MissingOperand { x, y };
    CompilerErrors::ParserError { err }
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
            let new_e = try!(BinNode::new(t, old_e, right));

            e = Rc::new( new_e );
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
            None => { return Err($self.$experted_func(curr_t.coords.x, curr_t.coords.y, $var)); }
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
            _ => { return Err($self.expected_token(t.coords.x, t.coords.y, $var)); }
        }
    })
}
