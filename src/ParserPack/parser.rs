use ParserPack::Nodes::*;
use ParserPack::tree::*;
use TokenizerPack::tokenizer::Tokenizer;
use TokenizerPack::support::*;


type NodeResult = Result<Box<Node>, String>;
type TreeResult = Result<Tree, String>;

macro_rules! parse {
    ($self:ident, $func:ident, [$($x: path),*]) => ({
        let mut e = match $self.$func() {
            Ok(val) => val,
            Err(msg) => return Err(msg)
        };

        let mut t = $self.tokenizer.current.clone();
        while match t.token_type {
            $($x)|* => true,
            _ => false
        } {
            match $self.tokenizer.next() {
                Some(res) => match res {
                    Ok(_token) => {},
                    Err(msg) => return Err(msg)
                }
                None => { }
            };
            let right = match $self.$func() {
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
                Err(msg) => return Err(msg)
            }
            None => { return Err(String::from("Error: Not found tokens")); }
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
                Err(msg) => return Err(msg)
            }
            None => { }
        };
        match t.token_type {
            TokenType::TDouble => { return Ok(Box::new(ConstNode::new(t.value.as_double()))) },
            TokenType::TInt => { return Ok(Box::new(ConstNode::new(t.value.as_int()))) },
            TokenType::TId => { return Ok(Box::new(IdNode::new(t.value.as_string()))) },
            TokenType::TOp => {
                let e = self.parse_expr();

                let t = self.tokenizer.current.clone();
                match t.token_type {
                    TokenType::TCp => {},
                    _ => return Err(format!("Ошибка в ({}, {}): Missing closing bracket", t.coords.y, t.coords.x))
                }

                match self.tokenizer.next() {
                    Some(res) => match res {
                        Ok(_token) => {},
                        Err(msg) => return Err(msg)
                    }
                    None => { }
                };

                return e;
            } 
            _ => return Err(format!("Ошибка в ({}, {}): Missing operand", t.coords.y, t.coords.x))
        }
    }
}