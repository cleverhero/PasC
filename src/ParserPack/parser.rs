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
        Parser { tokenizer }
    } 

    pub fn parse(&mut self) -> TreeResult {
        try!(self.tokenizer.my_next());
        Ok(Tree::new( try!(self.parse_program()) ))
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
        match before_parse!(self, [TokenType::TProgram => parse_header_id]) {
            Some(node) => node,
            None => Ok(Box::new(IdNode::new("No name".to_string())))
        }
    }

    fn parse_header_id(&mut self, _t: &Token) -> NodeResult {
        let e = before_parse!(self, expected_token, TokenType::TId => parse_id);
        check_token!(self, TokenType::TSemicolom);
        e
    } 

    fn parse_block(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Block".to_string());
        let child = try!( self.parse_declarations() );
        e.add_child(child);

        let child = before_parse!(self, expected_token, TokenType::TBegin => parse_statements);
        e.add_child( try!(child) );
        
        Ok(Box::new(e))
    }

    fn parse_declarations(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Declarations".to_string());

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type = [TokenType::TBegin]);

            let child = match before_parse!(self, [TokenType::TVar => parse_var_declaration_list]) {
                Some(res) => try!(res),
                None => { return Err(expected_token(t.coords.x, t.coords.y, TokenType::TSemicolom)); }
            };
            e.add_child(child);
        }
        Ok(Box::new(e))
    }

    fn parse_var_declaration_list(&mut self, _t: &Token) -> NodeResult {
        let mut e = ProgramNode::new("var_declaration".to_string());
        let child = before_parse!(self, expected_token, TokenType::TId => parse_var_declaration);
        e.add_child( try!(child) );
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type = [TokenType::TBegin, TokenType::TVar]);
        
            let child = before_parse!(self, expected_token, TokenType::TId => parse_var_declaration);
            e.add_child( try!(child) );
        }

        Ok(Box::new(e))
    }

    fn parse_var_declaration(&mut self, t: &Token) -> NodeResult {
        let mut e = ProgramNode::new(t.value.as_string());
        check_token!(self, TokenType::TColon);

        let child = self.parse_type();  
        e.add_child( try!(child) );
        
        match before_parse!(self, [TokenType::TEq => parse_simple_expr]) {
            Some(res) => e.add_child( try!(res) ),
            None => { }
        };

        check_token!(self, TokenType::TSemicolom);

        Ok(Box::new(e))
    }

    fn parse_type(&mut self) -> NodeResult {
        before_parse!(self, expected_token, TokenType::TId => parse_id)
    }

    fn parse_statements(&mut self, _t: &Token) -> NodeResult {
        let mut e = ProgramNode::new("Statements".to_string());

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type = [TokenType::TEnd, TokenType::TEof]);
            
            let child = match before_parse!(self, [TokenType::TId  => parse_simple_stmt]) {
                Some(res) => try!(res),
                None => { return Err(missing_operand(t.coords.x, t.coords.y)); }
            };

            check_token!(self, TokenType::TSemicolom);
            e.add_child( child );
        }

        try!(self.tokenizer.my_next());
        check_token!(self, TokenType::TPoint);

        Ok(Box::new(e))
    }

    fn parse_simple_stmt(&mut self, t: &Token) -> NodeResult {
        match behind_parse!(self, t, [TokenType::TOp     => parse_func_call,
                                      TokenType::TAssign => parse_assign]) {
            Some(node) => Ok(try!(node)),
            None => Err(expected_token(t.coords.x, t.coords.y, TokenType::TSemicolom))
        }
    }

    fn parse_assign(&mut self, t: &Token) -> NodeResult {
        let op = self.tokenizer.current.clone();
        let mut e = ProgramNode::new(op.text.to_string());//Хранить токен
        e.add_child(Box::new(IdNode::new(t.value.as_string())));

        try!(self.tokenizer.my_next());
        e.add_child( try!(self.parse_simple_expr(&t)) );

        return Ok(Box::new(e));
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
        let t = self.tokenizer.current.clone();

        match before_parse!(self, [TokenType::TInt    => parse_int,
                                   TokenType::TDouble => parse_double,
                                   TokenType::TId     => parse_id,
                                   TokenType::TOp     => parse_op_in_expr,
                                   TokenType::TPlus   => parse_unary,
                                   TokenType::TMinus  => parse_unary,
                                   TokenType::TNot    => parse_unary]) {

            Some(res) => Ok( try!(res) ),
            None => { return Err(missing_operand(t.coords.x, t.coords.y)); }
        }
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
            Some(node) => return Ok(try!(node)),
            None => {}
        };
        match behind_parse!(self, t, [TokenType::TObr => parse_array_element]) {
            None => {},
            Some(node) => return Ok(try!(node))
        };
        Ok(Box::new(IdNode::new(t.value.as_string())))
    }

    fn parse_array_element(&mut self, _t: &Token) -> NodeResult {
        let mut e = ProgramNode::new("array".to_string());

        let t = self.tokenizer.after.clone();
        try!(self.tokenizer.my_next());
        
        match t.token_type {
            TokenType::TCbr => { },
            _ => { e.add_child( try!( self.parse_simple_expr(&t) )) }
        }
        
        check_token!(self, TokenType::TCbr);
        match behind_parse!(self, t, [TokenType::TObr => parse_array_element]) {
            None => {},
            Some(node) => e.add_child(try!(node))
        };

        Ok(Box::new(e))
    }

    fn parse_func_call(&mut self, t: &Token) -> NodeResult {
        let mut e = Box::new(ProgramNode::new(t.value.as_string()));

        let t = self.tokenizer.after.clone();
        
        match t.token_type {
            TokenType::TCp => { try!(self.tokenizer.my_next()); },
            _ => { e.add_child( try!( self.parse_arg_list() )) }
        }

        try!(self.tokenizer.my_next());
        match behind_parse!(self, t, [TokenType::TObr => parse_array_element]) {
            None => {},
            Some(node) => e.add_child(try!(node))
        };
        
        Ok(e)
    }

    fn parse_arg_list(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Arguments list".to_string());

        let child = before_parse!(self, expected_token, TokenType::TOp => parse_simple_expr);
        e.add_child(try!(child));
        
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type = [TokenType::TCp]);

            let child = before_parse!(self, expected_token, TokenType::TComma => parse_simple_expr);
            e.add_child(try!(child));
        }

        Ok(Box::new(e))
    }

    fn parse_op_in_expr(&mut self, _t: &Token) -> NodeResult { 
        let e = self.parse_expr();
        check_token!(self, TokenType::TCp);
        return e;
    }
}

