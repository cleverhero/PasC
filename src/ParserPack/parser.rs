use ParserPack::Nodes::*;
use ParserPack::tree::*;
use TokenizerPack::tokenizer::Tokenizer;
use TokenizerPack::support::*;
use TokenizerPack::token::*;
use ParserPack::support::*;
use support::*;

type NodeResult = Result<Box<Node>, CompilerErrors>;
type TreeResult = Result<Tree, CompilerErrors>;

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
            None => { }
        };

        match self.parse_simple_expr() {
            Ok(node) => return Ok(Tree::new(node)),
            Err(msg) => return Err(msg)
        }  
    } 

    fn parse_simple_expr(&mut self) -> NodeResult {
        parse_bin!(self, parse_expr, [TokenType::TGe, TokenType::TGt,
                                      TokenType::TEq, TokenType::TLe,
                                      TokenType::TLt, TokenType::TNe])
    }

    fn parse_expr(&mut self) -> NodeResult {
        parse_bin!(self, parse_term, [TokenType::TPlus, TokenType::TMinus, TokenType::TOr])
    }

    fn parse_term(&mut self) -> NodeResult {
        parse_bin!(self, parse_factor, [TokenType::TMul, TokenType::TShare, TokenType::TDiv, TokenType::TMod, TokenType::TAnd])
    }

    fn parse_factor(&mut self) -> NodeResult {
        before_parse!(self, missing_closing_bracket, [TokenType::TInt    => parse_int,
                                                      TokenType::TDouble => parse_double,
                                                      TokenType::TId     => parse_id_in_expr,
                                                      TokenType::TOp     => parse_op_in_expr,
                                                      TokenType::TPlus   => parse_tern_plus,
                                                      TokenType::TMinus  => parse_tern_minus,
                                                      TokenType::TNot    => parse_not])
    }

    fn parse_not(&mut self, t: &Token) -> NodeResult { 
        let children = match self.parse_factor()  {
            Ok(val) => val,
            Err(err) => return Err(err)
        };

        let e = TernarOpNode::new(t.clone(), children);
        Ok(Box::new(e)) 
    }

    fn parse_tern_plus(&mut self, t: &Token) -> NodeResult { 
        let children = match self.parse_factor()  {
            Ok(val) => val,
            Err(err) => return Err(err)
        };

        let e = TernarOpNode::new(t.clone(), children);
        Ok(Box::new(e)) 
    }
    fn parse_tern_minus(&mut self, t: &Token) -> NodeResult { 
        let children = match self.parse_factor()  {
            Ok(val) => val,
            Err(err) => return Err(err)
        };

        let e = TernarOpNode::new(t.clone(), children);
        Ok(Box::new(e)) 
    }
    fn parse_double(&mut self, t: &Token) -> NodeResult { 
        Ok(Box::new(ConstNode::new(t.value.as_double()))) 
    }
    fn parse_int(&mut self, t: &Token) -> NodeResult { 
        Ok(Box::new(ConstNode::new(t.value.as_int()))) 
    }
    fn parse_id_in_expr(&mut self, t: &Token) -> NodeResult { 
        Ok(Box::new(IdNode::new(t.value.as_string()))) 
    }
    fn parse_op_in_expr(&mut self, _t: &Token) -> NodeResult { 
        let e = self.parse_expr();

        let t = self.tokenizer.current.clone();
        match t.token_type {
            TokenType::TCp => {},
            _ => return Err(missing_operand(t.coords.x, t.coords.y))
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

}