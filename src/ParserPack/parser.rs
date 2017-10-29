use ParserPack::Nodes::*;
use ParserPack::tree::*;
use TokenizerPack::tokenizer::Tokenizer;
use TokenizerPack::support::*;
use support::*;


type NodeResult = Result<Box<Node>, CompilerErrors>;
type TreeResult = Result<Tree, CompilerErrors>;

macro_rules! parse {
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
                Err(msg) => return Err(msg)
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

pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Parser {
        Parser {
            tokenizer
        }
    } 

    pub fn parse(&mut self) -> TreeResult {
        match self.tokenizer.next() {
            Some(res) => match res {
                Ok(_token) => {},
                Err(err) => return Err(CompilerErrors::TokenizerError{err})
            }
            None => {
                let err = ParserErrors::EmptyFile{ x: 0, y: 0 };
                return Err(CompilerErrors::ParserError{ err });
            }
        };

        match self.parse_expr() {
            Ok(node) => return Ok(Tree::new(node)),
            Err(msg) => return Err(msg)
        }  
    } 

    fn parse_expr(&mut self) -> NodeResult {
        parse!(self, parse_term, [TokenType::TPlus, TokenType::TMinus])
    }

    fn parse_term(&mut self) -> NodeResult {
        parse!(self, parse_factor, [TokenType::TMul, TokenType::TShare])
    }

    fn parse_factor(&mut self) -> NodeResult {
        let t = self.tokenizer.current.clone();
        match self.tokenizer.next() {
            Some(res) => match res {
                Ok(_token) => {},
                Err(err) => return Err(CompilerErrors::TokenizerError{err})
            }
            None => { }
        };
        match t.token_type {
            TokenType::TDouble => { return Ok(Box::new(ConstNode::new(t.value.as_double()))) },
            TokenType::TInt    => { return Ok(Box::new(ConstNode::new(t.value.as_int()))) },
            TokenType::TId     => { return Ok(Box::new(IdNode::new(t.value.as_string()))) },
            TokenType::TOp     => {
                let e = self.parse_expr();

                let t = self.tokenizer.current.clone();
                match t.token_type {
                    TokenType::TCp => {},
                    _ => {
                        let err = ParserErrors::MissingClosingBracket{ x: t.coords.clone().x, y: t.coords.clone().y };
                        return Err(CompilerErrors::ParserError{ err });
                    }
                }

                match self.tokenizer.next() {
                    Some(res) => match res {
                        Ok(_token) => {},
                        Err(err) => return Err(CompilerErrors::TokenizerError{err})
                    }
                    None => { }
                };

                return e;
            } 
            _ => {
                let err = ParserErrors::MissingOperand{ x: t.coords.clone().x, y: t.coords.clone().y };
                return Err(CompilerErrors::ParserError{ err });
            }
        }
    }
}