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
        check_token!(self, TokenType::TPoint);
        
        Ok(Box::new(e))
    }

    fn parse_header(&mut self) -> NodeResult {
        let t = self.tokenizer.current.clone();

        match parse!(self, &t, [ TokenType::TProgram => {
            try!(self.tokenizer.my_next()); 
            self.parse_header_id(&t) 
        } ]) {
            Some(node) => node,
            None => Ok(Box::new(IdNode::new("No name".to_string())))
        }
    }

    fn parse_header_id(&mut self, _t: &Token) -> NodeResult {
        let t = try!( self.tokenizer.get_and_next() );
        let e = parse_simple!(self, &t, [ TokenType::TId => { self.parse_id(&t) } ], expected_token);

        check_token!(self, TokenType::TSemicolom);
        e
    } 

    fn parse_block(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Block".to_string());
        let child = try!( self.parse_declarations() );
        e.add_child(child);

        let t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &t, [TokenType::TBegin => { self.parse_statements(&t) } ], expected_token);
        e.add_child( try!(child) );
        
        Ok(Box::new(e))
    }

    fn parse_declarations(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Declarations".to_string());

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TBegin]);
            try!(self.tokenizer.my_next());
            let child = match parse!(self, &t, [ TokenType::TVar       => { self.parse_var_declaration_list(&t, "var_declaration".to_string()) },
                                                 TokenType::TConst     => { self.parse_const_declaration_list(&t) },
                                                 TokenType::TType      => { self.parse_type_declaration_list(&t) },
                                                 TokenType::TFunction  => { self.parse_function_declaration(&t) },
                                                 TokenType::TProcedure => { self.parse_procedure_declaration(&t) } ]) {
                Some(res) => try!(res),
                None => { return Err(expected_token(t.coords.x, t.coords.y, TokenType::TSemicolom)); }
            };
            e.add_child(child);
        }
        Ok(Box::new(e))
    }

    fn parse_type_declaration_list(&mut self, _t: &Token) -> NodeResult {
        let mut e = ProgramNode::new("Type declarations".to_string());

        let t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_type_declaration(&t) } ], expected_token);
        e.add_child( try!(child) );

        check_token!(self, TokenType::TSemicolom);

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [ TokenType::TBegin, TokenType::TType, TokenType::TVar, 
                                        TokenType::TConst, TokenType::TFunction, TokenType::TProcedure]);
            try!(self.tokenizer.my_next());

            let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_type_declaration(&t) } ], expected_token);
            e.add_child( try!(child) );

            check_token!(self, TokenType::TSemicolom);
        }

        Ok( Box::new(e) )
    }

    fn parse_type_declaration(&mut self, t: &Token) -> NodeResult {
        let mut e = ProgramNode::new(t.value.to_string());

        check_token!(self, TokenType::TEq);

        let child = self.parse_type();
        e.add_child( try!(child) );

        Ok( Box::new(e) )
    }

    fn parse_procedure_declaration(&mut self, _t: &Token) -> NodeResult {
        let func_name = try!( self.tokenizer.get_and_next() );
        let mut e = ProgramNode::new(func_name.value.to_string());

        let t = self.tokenizer.current.clone();
        let argument_list = match parse!(self, &t, [ TokenType::TOp => { 
                                                try!(self.tokenizer.my_next());
                                                self.parse_decl_arg_list(&t) 
                                            },
                                            TokenType::TSemicolom => { 
                                                let e = ProgramNode::new("Arguments list".to_string());
                                                Ok (Box::new(e) as Box<Node> )
                                            } ]) {
            Some(res) => try!(res),
            None => { return Err(expected_token(t.coords.x, t.coords.y, TokenType::TSemicolom)); }
        };
        check_token!(self, TokenType::TSemicolom);

        let t = self.tokenizer.current.clone();
        let body = match parse!(self, &t,  [ TokenType::TBegin   => { self.parse_block() },
                                             TokenType::TForward => { 
                                                 e = ProgramNode::new("Forward procedure".to_string());
                                                 self.parse_forward()
                                             } ]) {
            Some(res) => try!(res),
            None => { return Err(expected_token(t.coords.x, t.coords.y, TokenType::TBegin)); }
        };
        check_token!(self, TokenType::TSemicolom);

        e.add_child( argument_list );
        e.add_child( body );
        
        Ok(Box::new(e))
    }

    fn parse_function_declaration(&mut self, _t: &Token) -> NodeResult {
        let func_name = try!( self.tokenizer.get_and_next() );
        let mut e = ProgramNode::new(func_name.value.to_string());

        let t = self.tokenizer.current.clone();
        let argument_list = match parse!(self, &t, [ TokenType::TOp    => { 
                                                         try!(self.tokenizer.my_next());
                                                         self.parse_decl_arg_list(&t) 
                                                     },
                                                     TokenType::TColon => { 
                                                         let e = ProgramNode::new("Arguments list".to_string());
                                                         Ok (Box::new(e) as Box<Node> )
                                                     } ]) {
            Some(res) => try!(res),
            None => { return Err(expected_token(t.coords.x, t.coords.y, TokenType::TColon)); }
        };
        check_token!(self, TokenType::TColon);

        let out_type = try!( self.parse_type() );  
        check_token!(self, TokenType::TSemicolom);

        let t = self.tokenizer.current.clone();
        let body = match parse!(self, &t,  [ TokenType::TBegin   => { self.parse_block() },
                                             TokenType::TForward => { 
                                                 e = ProgramNode::new("Forward function".to_string());
                                                 self.parse_forward()
                                             } ]) {
            Some(res) => try!(res),
            None => { return Err(expected_token(t.coords.x, t.coords.y, TokenType::TBegin)); }
        };
        check_token!(self, TokenType::TSemicolom);

        e.add_child( argument_list );
        e.add_child( out_type );
        e.add_child( body );
        
        Ok(Box::new(e))
    }

    fn parse_forward(&mut self) -> NodeResult {
        try!(self.tokenizer.my_next());
        Ok( Box::new(ProgramNode::new("Forward".to_string())) )
    }

    fn parse_decl_arg_list(&mut self, _t: &Token) -> NodeResult {
        let mut e = ProgramNode::new("Arguments list".to_string());
        
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TCp]);
            try!(self.tokenizer.my_next());

            let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_var_declaration(&t) } ], expected_token);
            e.add_child(try!(child));

            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TCp]);

            check_token!(self, TokenType::TSemicolom);
        }
        try!(self.tokenizer.my_next());

        Ok(Box::new(e))
    }

    fn parse_const_declaration_list(&mut self, _t: &Token) -> NodeResult {
        let mut e = ProgramNode::new("const_declaration".to_string());

        let t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_const_declaration(&t) } ], expected_token);
        e.add_child( try!(child) );

        check_token!(self, TokenType::TSemicolom);
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [ TokenType::TBegin, TokenType::TType, TokenType::TVar, 
                                        TokenType::TConst, TokenType::TFunction, TokenType::TProcedure]);
            try!(self.tokenizer.my_next());

            let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_const_declaration(&t) } ], expected_token);
            e.add_child( try!(child) );

            check_token!(self, TokenType::TSemicolom);
        }

        Ok(Box::new(e))
    }

    fn parse_var_declaration_list(&mut self, _t: &Token, node_name: String) -> NodeResult {
        let mut e = ProgramNode::new(node_name.clone());

        let t = try!( self.tokenizer.get_and_next() );
        println!("{}", t);
        let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_var_declaration(&t) } ], expected_token);
        e.add_child( try!(child) );

        check_token!(self, TokenType::TSemicolom);
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [ TokenType::TBegin, TokenType::TType, TokenType::TVar, 
                                        TokenType::TConst, TokenType::TFunction, TokenType::TProcedure]);

            if node_name == "Record" { break_if!(t.token_type == [ TokenType::TEnd]); }

            try!(self.tokenizer.my_next());

            let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_var_declaration(&t) } ], expected_token);
            e.add_child( try!(child) );

            check_token!(self, TokenType::TSemicolom);
        }

        println!("{}", e);
        Ok(Box::new(e))
    }

    fn parse_const_declaration(&mut self, t: &Token) -> NodeResult {
        let mut e = ProgramNode::new(t.value.as_string());

        let t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &t, [ TokenType::TEq => { self.parse_simple_expr(&t) } ], expected_token);
        e.add_child( try!(child) );

        Ok(Box::new(e))
    }

    fn parse_var_declaration(&mut self, t: &Token) -> NodeResult {
        let mut e = ProgramNode::new(t.value.as_string());
        check_token!(self, TokenType::TColon);

        let child = self.parse_type();  
        e.add_child( try!(child) );

        let t = self.tokenizer.current.clone();
        match parse!(self, &t, [TokenType::TEq => { 
            try!(self.tokenizer.my_next());
            self.parse_simple_expr(&t) 
        } ]) {
            Some(res) => e.add_child( try!(res) ),
            None => { }
        };

        Ok(Box::new(e))
    }

    fn parse_type(&mut self) -> NodeResult {
        let t = try!( self.tokenizer.get_and_next() );

        match parse!(self, &t, [ TokenType::TId          => { self.parse_id(&t) },
                                 TokenType::TRecord      => { self.parse_record(&t) },
                                 TokenType::TIntegerType => { self.parse_id(&t) },
                                 TokenType::TDoubleType  => { self.parse_id(&t) },
                                 TokenType::TCharType    => { self.parse_id(&t) }  ]) {
            Some(res) => Ok(try!(res)),
            None => { Err(missing_operand(t.coords.x, t.coords.y)) }
        }
    }

    fn parse_record(&mut self, t: &Token) -> NodeResult {
        let e = try!( self.parse_var_declaration_list(t, "Record".to_string()) );

        check_token!(self, TokenType::TEnd);

        Ok( e )
    }

    fn parse_statements(&mut self, _t: &Token) -> NodeResult {
        let mut e = ProgramNode::new("Statements".to_string());

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TEnd, TokenType::TEof]);

            let child = match parse!(self, &t, [ TokenType::TId     => { self.parse_simple_stmt(&t) },
                                                 TokenType::TFor    => { self.parse_for(&t) },
                                                 TokenType::TIf     => { self.parse_if(&t) },
                                                 TokenType::TWhile  => { self.parse_while(&t) },
                                                 TokenType::TRepeat => { self.parse_repeat(&t) },
                                                 TokenType::TBegin  => { 
                                                    try!( self.tokenizer.my_next() ); 
                                                    self.parse_statements(&t) 
                                                } ]) {
                Some(res) => try!(res),
                None => { return Err(missing_operand(t.coords.x, t.coords.y)); }
            };


            check_token!(self, TokenType::TSemicolom);
            e.add_child( child );
        }

        try!(self.tokenizer.my_next());

        Ok(Box::new(e))
    }

    fn parse_simple_stmt(&mut self, _t: &Token) -> NodeResult {   
        let curr_t = try!( self.tokenizer.get_and_next() );
        let targer = try!(self.parse_id(&curr_t)); 

        let curr_t = self.tokenizer.current.clone(); 
        match parse!(self, &curr_t, [ TokenType::TAssign      => { self.parse_assign(targer) },
                                      TokenType::TPlsAssign   => { self.parse_assign(targer) },
                                      TokenType::TMinAssign   => { self.parse_assign(targer) },
                                      TokenType::TMulAssign   => { self.parse_assign(targer) },
                                      TokenType::TShareAssign => { self.parse_assign(targer) },
                                      TokenType::TSemicolom   => { Ok( targer ) }              ]) {
            Some(node) => Ok(try!(node)),
            None => Err(expected_token(curr_t.coords.x, curr_t.coords.y, TokenType::TSemicolom))
        }
    }

    fn parse_assign(&mut self, target: Box<Node>) -> NodeResult {
        let op = self.tokenizer.current.clone();
        let mut e = ProgramNode::new(op.text.to_string()); //Хранить токен
        e.add_child(target);

        let t = self.tokenizer.current.clone(); 
        try!(self.tokenizer.my_next());
        e.add_child( try!(self.parse_simple_expr(&t)) );

        return Ok(Box::new(e));
    }

    fn parse_for(&mut self, _t: &Token) -> NodeResult {
        try!( self.tokenizer.my_next() );
        let mut e = ProgramNode::new("For".to_string());

        let t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_simple_id(&t) } ], expected_token);
        e.add_child(try!(child));

        check_token!(self, TokenType::TAssign);

        let child = self.parse_expr();
        e.add_child(try!(child));

        check_token!(self, TokenType::TTo);

        let child = self.parse_expr();
        e.add_child(try!(child));

        check_token!(self, TokenType::TDo);

        let t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &t, [ TokenType::TBegin => { self.parse_statements(&t) } ], expected_token);
        e.add_child( try!(child) );

        Ok(Box::new(e))
    }

    fn parse_repeat(&mut self, t: &Token) -> NodeResult {
        try!( self.tokenizer.my_next() );
        let mut e = ProgramNode::new("Repeat".to_string());

        let curr_t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &curr_t, [ TokenType::TBegin => { self.parse_statements(&curr_t) } ], expected_token);
        e.add_child( try!(child) );

        check_token!(self, TokenType::TSemicolom);
        check_token!(self, TokenType::TUntil);

        let child = self.parse_simple_expr(t);
        e.add_child(try!(child));

        Ok(Box::new(e))
    }

    fn parse_while(&mut self, t: &Token) -> NodeResult {
        try!( self.tokenizer.my_next() );
        let mut e = ProgramNode::new("While".to_string());

        let child = self.parse_simple_expr(t);
        e.add_child(try!(child));

        check_token!(self, TokenType::TDo);

        let curr_t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &curr_t, [ TokenType::TBegin => { self.parse_statements(&curr_t) } ], expected_token);
        e.add_child( try!(child) );

        Ok(Box::new(e))
    }

    fn parse_if(&mut self, t: &Token) -> NodeResult {
        try!( self.tokenizer.my_next() );
        let mut e = ProgramNode::new("If".to_string());

        let child = self.parse_simple_expr(t);
        e.add_child(try!(child));

        check_token!(self, TokenType::TThen);

        let curr_t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &curr_t, [ TokenType::TBegin => { self.parse_statements(&curr_t) } ], expected_token);
        e.add_child( try!(child) );
 
        let curr_t = self.tokenizer.current.clone();
        match parse!(self, &curr_t, [TokenType::TElse => {
            try!(self.tokenizer.my_next());
            self.parse_stmts(&curr_t) 
        } ]) {
            Some(res) => e.add_child( try!(res) ),
            None => { }
        };

        Ok(Box::new(e))
    }

    fn parse_stmts(&mut self, _t: &Token) -> NodeResult {
        let curr_t = try!( self.tokenizer.get_and_next() );
        parse_simple!(self, &curr_t, [ TokenType::TBegin => { self.parse_statements(&curr_t) } ], expected_token)
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
        let t = try!( self.tokenizer.get_and_next() ); 
        match parse!(self, &t, [ TokenType::TInt    => { self.parse_int(&t) },
                                 TokenType::TDouble => { self.parse_double(&t) },
                                 TokenType::TId     => { self.parse_id(&t) },
                                 TokenType::TOp     => { self.parse_op_in_expr(&t) },
                                 TokenType::TPlus   => { self.parse_unary(&t) },
                                 TokenType::TMinus  => { self.parse_unary(&t) },
                                 TokenType::TNot    => { self.parse_unary(&t) }]) {

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
        let curr_t = self.tokenizer.current.clone();  
        match parse!(self, &curr_t, [ TokenType::TOp  => { self.parse_func_call(t) },
                                      TokenType::TObr => { self.parse_array_element(t.value.as_string()) } ]) {
            Some(node) => return Ok(try!(node)),
            None => {}
        };

        self.parse_simple_id(t)      
    }
    fn parse_simple_id(&mut self, t: &Token) -> NodeResult { 
        Ok(Box::new(IdNode::new(t.value.as_string())))
    }

    fn parse_array_element(&mut self, array_name: String) -> NodeResult {
        let mut e = ProgramNode::new(array_name);

        try!(self.tokenizer.my_next());
        let t = self.tokenizer.current.clone();
        
        match t.token_type {
            TokenType::TCbr => { },
            _ => { e.add_child( try!( self.parse_index_list() )) }
        }

        let curr_t = self.tokenizer.current.clone();
        match parse!(self, &curr_t, [TokenType::TObr => { self.parse_array_element("Array element".to_string()) } ]) {
            Some(node) => e.add_child(try!(node)),
            None => {}
        };

        Ok(Box::new(e))
    }

    fn parse_index_list(&mut self) -> NodeResult {
        let t = self.tokenizer.current.clone();
        let mut e = Box::new( ProgramNode::new("Index".to_string()) );

        let index = try!( self.parse_simple_expr(&t) );
        e.add_child(index);

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TCbr]);
            try!(self.tokenizer.my_next());

            let index = try!( self.parse_simple_expr(&t) );
            e.add_child(index);
        }

        check_token!(self, TokenType::TCbr);

        Ok(e)
    }

    fn parse_func_call(&mut self, t: &Token) -> NodeResult {
        let mut e = Box::new(ProgramNode::new(t.value.as_string()));
        let t = self.tokenizer.after.clone();
        
        match t.token_type {
            TokenType::TCp => { try!(self.tokenizer.my_next()); },
            _ => { e.add_child( try!( self.parse_arg_list() )) }
        }

        try!(self.tokenizer.my_next());
        let curr_t = self.tokenizer.current.clone();
        match parse!(self, &curr_t, [TokenType::TObr => { self.parse_array_element("Array".to_string()) } ]) {
            Some(node) => e.add_child(try!(node)),
            None => {}
        };
        
        Ok(e)
    }

    fn parse_arg_list(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Arguments list".to_string());

        let t = try!( self.tokenizer.get_and_next() ); 
        let child = parse_simple!(self, &t, [TokenType::TOp => { self.parse_simple_expr(&t) } ], expected_token);
        e.add_child(try!(child));
        
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TCp]);
            try!(self.tokenizer.my_next());

            let child = parse_simple!(self, &t, [TokenType::TComma => { self.parse_simple_expr(&t) } ], expected_token);
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