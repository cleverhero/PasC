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
        try!(self.tokenizer.my_next());
        Ok(Tree::new( try!(self.parse_expr()) ))
    } 

    fn parse_program(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Program".to_string());
        let child = try!( self.parse_header() );
        e.add_child(child);

        let child = try!( self.parse_block() );
        e.add_child(child);
        
        Ok(Box::new(e))
    }

    fn parse_header(&mut self) -> NodeResult {
        before_parse!(self, Box::new(IdNode::new("No name".to_string())), [TokenType::TProgram => parse_header_id])
    }

    fn parse_header_id(&mut self, _t: &Token) -> NodeResult {
        let e = before_parse!(self, expected_semicolom, [TokenType::TId => parse_id]);
        try!(self.check_semicolon() );
        e
    }

    fn parse_block(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Block".to_string());
        let child = try!( self.parse_declarations() );
        e.add_child(child);

        let child = before_parse!(self, expected_begin, [TokenType::TBegin => parse_statements]);
        let child = try!( child );
        e.add_child(child);
        
        Ok(Box::new(e))
    }

    fn parse_declarations(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Declarations".to_string());

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type = [TokenType::TBegin]);
        
            let res = before_parse!(self, expected_semicolom, [TokenType::TVar => parse_var_declaration_list]); 
            e.add_child(try!(res));
        }
        Ok(Box::new(e))
    }

    fn parse_var_declaration_list(&mut self, _t: &Token) -> NodeResult {
        let mut e = ProgramNode::new("var_declaration".to_string());

        let child = before_parse!(self, expected_id, [TokenType::TId => parse_var_declaration]); 
        e.add_child(try!(child));

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type = [TokenType::TBegin, TokenType::TVar]);
        
            let child = before_parse!(self, expected_id, [TokenType::TId => parse_var_declaration]);  
            e.add_child(try!(child));
        }

        Ok(Box::new(e))
    }

    fn parse_var_declaration(&mut self, t: &Token) -> NodeResult {
        let mut e = ProgramNode::new(t.value.as_string());
        try!(self.check_colon());

        let child = self.parse_type();  
        e.add_child( try!(child) );
        
        let child = before_parse!(self, expected_semicolom, [TokenType::TEq => parse_simple_expr,
                                                             TokenType::TSemicolom => fake]);
        let child = try!(child);
        if child.get_value() != "#" { 
            e.add_child( child ); 
            try!(self.check_semicolon());
        }

        Ok(Box::new(e))
    }

    fn parse_type(&mut self) -> NodeResult {
        before_parse!(self, expected_id, [TokenType::TId => parse_id])
    }

    fn parse_statements(&mut self, _t: &Token) -> NodeResult {
        let e = ProgramNode::new("Statements".to_string());

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type = [TokenType::TEnd]);
        
        }

        Ok(Box::new(e))
    }

    fn parse_simple_expr(&mut self, _t: &Token) -> NodeResult {
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
        before_parse!(self, missing_operand, [TokenType::TInt    => parse_int,
                                              TokenType::TDouble => parse_double,
                                              TokenType::TId     => parse_id,
                                              TokenType::TOp     => parse_op_in_expr,
                                              TokenType::TPlus   => parse_unary,
                                              TokenType::TMinus  => parse_unary,
                                              TokenType::TNot    => parse_unary])
    }

    fn parse_unary(&mut self, t: &Token) -> NodeResult { 
        let child = try!( self.parse_factor() );
        Ok(Box::new( UnaryOpNode::new(t.clone(), child) ))
    }
    fn parse_double(&mut self, t: &Token) -> NodeResult { 
        Ok(Box::new(ConstNode::new(t.value.as_double()))) 
    }
    fn parse_int(&mut self, t: &Token) -> NodeResult { 
        Ok(Box::new(ConstNode::new(t.value.as_int()))) 
    }
    fn parse_id(&mut self, t: &Token) -> NodeResult { 
        match behind_parse!(self, t, [TokenType::TOp => parse_func_call]) {
            None => Ok(Box::new(IdNode::new(t.value.as_string()))),
            Some(node) => node
        }
    }

    fn parse_func_call(&mut self, t: &Token) -> NodeResult {
        let mut e = Box::new(ProgramNode::new(t.value.as_string()));

        let t = self.tokenizer.after.clone();
        match t.token_type {
            TokenType::TCp => { try!(self.tokenizer.my_next()); },
            _ => { e.add_child( try!( self.parse_arg_list() )) }
        }
        
        try!(self.tokenizer.my_next());
        
        Ok(e)
    }

    fn parse_arg_list(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Arguments list".to_string());
        
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type = [TokenType::TCp]);

            let child = before_parse!(self, expected_comma, [TokenType::TOp    => parse_simple_expr,
                                                             TokenType::TComma => parse_simple_expr]);  
            e.add_child(try!(child));
        }

        Ok(Box::new(e))
    }

    fn parse_op_in_expr(&mut self, _t: &Token) -> NodeResult { 
        let e = self.parse_expr();
        before_parse!(self, missing_closing_bracket, [TokenType::TCp => fake]);
        return e;
    }

    fn check_semicolon(&mut self) -> NodeResult {
        before_parse!(self, expected_semicolom, [TokenType::TSemicolom => fake])
    }

    fn check_colon(&mut self) -> NodeResult {
        before_parse!(self, expected_colon, [TokenType::TColon => fake])
    }

    fn check_comma(&mut self) -> NodeResult {
        before_parse!(self, expected_comma, [TokenType::TComma => fake])
    }

    fn fake(&mut self, _t: &Token) -> NodeResult {
        Ok(Box::new(IdNode::new("#".to_string()))) 
    }
}

