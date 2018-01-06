use ParserPack::Nodes::*;
use std::rc::Rc;
use ParserPack::Types::*;
use ParserPack::tree::*;
use TokenizerPack::tokenizer::Tokenizer;
use TokenizerPack::support::*;
use TokenizerPack::token::*;
use ParserPack::support::*;
use support::*;
use SemanticPack::*;
use std::collections::HashMap;

type NodeResult = Result<Rc<Node>, CompilerErrors>;
type TypeResult = Result<Rc<Type>, CompilerErrors>;
type TreeResult = Result<Tree, CompilerErrors>;

pub struct Parser {
    pub tokenizer: Tokenizer,
    pub semantic_checker: SemanticChecker,

    last_unknownrecord_id: i32,
    last_unknownenum_id: i32,

    in_circle: bool,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Parser {
        Parser { 
            tokenizer, 
            semantic_checker: SemanticChecker::new(),
            last_unknownrecord_id: 0, 
            last_unknownenum_id:   0,
            in_circle:           false
        }
    } 

    pub fn parse(&mut self) -> TreeResult {
        try!(self.tokenizer.my_next());
        Ok(Tree::new( try!(self.parse_program()) ))
    } 

    fn parse_program(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Program".to_string());
        // let child = try!( self.parse_header() );
        // e.add_child(child);

        let child = try!( self.parse_block() );
        e.add_child(child);
        check_token!(self, TokenType::TPoint);

        try!( self.semantic_checker.remove_scope() );
        Ok( Rc::new(e) )
    }

    // fn parse_header(&mut self) -> NodeResult {
    //     let t = self.tokenizer.current.clone();

    //     match parse!(self, &t, [ TokenType::TProgram => {
    //         try!(self.tokenizer.my_next()); 
    //         self.parse_header_id(&t) 
    //     } ]) {
    //         Some(node) => node,
    //         None => Ok( Rc::new((IdNode::new("No name".to_string()))) )
    //     }
    // }

    // fn parse_header_id(&mut self, _t: &Token) -> NodeResult {
    //     let t = try!( self.tokenizer.get_and_next() );
    //     let e = parse_simple!(self, &t, [ TokenType::TId => { self.parse_id(&t) } ], expected_token);

    //     check_token!(self, TokenType::TSemicolom);
    //     e
    // } 

    fn parse_block(&mut self) -> NodeResult {
        let mut e = ProgramNode::new("Block".to_string());
        let child = try!( self.parse_declarations() );
        e.add_child(child);

        let t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &t, [TokenType::TBegin => { self.parse_statements(&t) } ], expected_token);
        e.add_child( try!(child) );
        
        Ok( Rc::new(e) )
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
                None => { return Err(self.expected_token(t.coords.x, t.coords.y, TokenType::TSemicolom)); }
            };
            e.add_child(child);
        }
        Ok( Rc::new(e) )
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

        Ok( Rc::new(e) )
    }

    fn parse_type_declaration(&mut self, t: &Token) -> NodeResult {
        check_token!(self, TokenType::TEq);
        let self_type = self.parse_type(t.text.clone());

        let e = Rc::new( TypedefNode::new(t.text.clone(), try!(self_type)) );
        try!( self.semantic_checker.create_typedef(e.clone(), 0) );

        Ok( e )
    }

    fn parse_procedure_declaration(&mut self, _t: &Token) -> NodeResult {
        let func_name = try!( self.tokenizer.get_and_next() ).text.to_string();

        self.semantic_checker.add_scope();
        let t = self.tokenizer.current.clone();
        let argument_list = match parse!(self, &t, [ TokenType::TOp => { 
                                                try!(self.tokenizer.my_next());
                                                self.parse_decl_arg_list() 
                                            },
                                            TokenType::TSemicolom => { 
                                                Ok( vec![] )
                                            } ]) {
            Some(res) => try!(res),
            None => { return Err(self.expected_token(t.coords.x, t.coords.y, TokenType::TSemicolom)); }
        };
        
        check_token!(self, TokenType::TSemicolom);
        let t = self.tokenizer.current.clone();
        let body = match parse!(self, &t,  [ TokenType::TForward => { self.parse_forward() } ]) {
            Some(res) => {
                try!(res);
                None
            },
            None => { 
                let e = Rc::new( DeclFunctionNode::new(func_name.clone(), argument_list.clone(), Rc::new( VoidType::new() ), None) );
                try!( self.semantic_checker.create_function( e.clone(), 1 ) );

                Some( try!(self.parse_block()) ) 
            }
        };
        check_token!(self, TokenType::TSemicolom); 

        try!( self.semantic_checker.remove_scope() ); 

        let e = Rc::new( DeclFunctionNode::new(func_name, argument_list, Rc::new( VoidType::new() ), body) );
        try!( self.semantic_checker.create_function( e.clone(), 0 ) );      
        Ok( e as Rc<Node> )
    }

    fn parse_function_declaration(&mut self, _t: &Token) -> NodeResult {
        let func_name = try!( self.tokenizer.get_and_next() ).text.to_string();

        self.semantic_checker.add_scope();
        let t = self.tokenizer.current.clone();
        let argument_list = match parse!(self, &t, [ TokenType::TOp    => { 
                                                         try!(self.tokenizer.my_next());
                                                         self.parse_decl_arg_list() 
                                                     },
                                                     TokenType::TColon => { Ok(vec![]) } ]) {
            Some(res) => try!(res),
            None => { return Err( self.expected_token(t.coords.x, t.coords.y, TokenType::TColon) ); }
        };
        
        check_token!(self, TokenType::TColon);
        let out_type = try!( self.parse_type("None".to_string()) );  
        check_token!(self, TokenType::TSemicolom);

        let t = self.tokenizer.current.clone();
        let body = match parse!(self, &t,  [ TokenType::TForward => { self.parse_forward() } ]) {
            Some(res) => {
                try!(res);
                None
            },
            None => { 
                let e = Rc::new( DeclFunctionNode::new(func_name.clone(), argument_list.clone(), out_type.clone(), None) );
                try!( self.semantic_checker.create_function( e.clone(), 1 ) );

                Some( try!(self.parse_block()) ) 
            }
        };
        check_token!(self, TokenType::TSemicolom);

        try!( self.semantic_checker.remove_scope() );

        let e = Rc::new( DeclFunctionNode::new(func_name, argument_list, out_type, body) );
        try!( self.semantic_checker.create_function( e.clone(), 0 ) );
        Ok( e as Rc<Node> )
    }

    fn parse_forward(&mut self) -> NodeResult {
        try!(self.tokenizer.my_next());
        Ok( Rc::new((ProgramNode::new("Forward".to_string()))) )
    }

    fn parse_decl_arg_list(&mut self) -> Result< Vec< Rc< Node > >, CompilerErrors> {
        let mut e: Vec< Rc< Node > > = vec![];
        
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TCp]);
            try!(self.tokenizer.my_next());

            let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_var_declaration(&t) } ], expected_token);
            e.push( try!(child) );

            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TCp]);

            check_token!(self, TokenType::TSemicolom);
        }
        try!(self.tokenizer.my_next());

        Ok( e )
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

        Ok( Rc::new(e) )
    }

    fn parse_var_declaration_list(&mut self, _t: &Token, node_name: String) -> NodeResult {
        let mut childs: Vec< Rc< Node > > = vec![];

        let t = try!( self.tokenizer.get_and_next() );
        let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_var_declaration(&t) } ], expected_token);
        childs.push( try!(child) );

        check_token!(self, TokenType::TSemicolom);
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [ TokenType::TBegin, TokenType::TType, TokenType::TVar, 
                                        TokenType::TConst, TokenType::TFunction, TokenType::TProcedure]);

            if node_name != "var_declaration" { break_if!(t.token_type == [ TokenType::TEnd]); }

            try!(self.tokenizer.my_next());

            let child = parse_simple!(self, &t, [ TokenType::TId => { self.parse_var_declaration(&t) } ], expected_token);
            childs.push( try!(child) );

            check_token!(self, TokenType::TSemicolom);
        }

        let e = if node_name != "var_declaration" {
            Rc::new( RecordNode::new(node_name, childs) ) as Rc<Node>
        }
        else { 
            Rc::new( DeclVarListNode::new(node_name, childs) ) as Rc<Node>
        };

        Ok( e )
    }

    fn parse_const_declaration(&mut self, t: &Token) -> NodeResult {
        check_token!(self, TokenType::TColon);
        let var_type = try!( self.parse_type("None".to_string()) );  

        check_token!(self, TokenType::TEq);
        let _res = var_type.parse_init_value(self);

        let e = Rc::new( DeclVarNode::new(t.value.as_string(), var_type) );

        try!( self.semantic_checker.create_var( e.clone(), 0 ) );
        Ok( e as Rc<Node> )
    }

    fn parse_var_declaration(&mut self, t: &Token) -> NodeResult {
        check_token!(self, TokenType::TColon);
        let var_type = try!( self.parse_type("None".to_string()) );  

        let curr_t = self.tokenizer.current.clone();
        match parse!(self, &curr_t, [TokenType::TEq => { 
            try!(self.tokenizer.my_next());
            var_type.parse_init_value(self) 
        } ]) {
            Some(res) => { try!(res); },
            None => { }
        }

        let e = Rc::new( DeclVarNode::new(t.value.as_string(), var_type) );
        try!( self.semantic_checker.create_var( e.clone(), 0 ) );
        Ok( e as Rc<Node>)
    }

    fn parse_type(&mut self, typename: String) -> TypeResult {
        let t = self.tokenizer.after.clone();
        match t.token_type {      
            TokenType::TRange | 
            TokenType::TPlus |
            TokenType::TMinus |
            TokenType::TMul |
            TokenType::TShare => { return self.parse_range() },    
            _ => {}
        }

        let t = try!( self.tokenizer.get_and_next() );
        
        match parse!(self, &t, [ TokenType::TIntegerType => { Ok( Rc::new(IntegerType::new(0)) as Rc<Type> ) },             
                                 TokenType::TDoubleType  => { Ok( Rc::new(DoubleType::new(0.0)) as Rc<Type> ) },              
                                 TokenType::TCharType    => { Ok( Rc::new(CharType::new(0)) as Rc<Type> ) },
                                 TokenType::TRecord      => { self.parse_record(&t, typename) },
                                 TokenType::TOp          => { self.parse_enum(&t, typename) },
                                 TokenType::TArray       => { self.parse_array(&t) }, 
                                 TokenType::TId          => { 
                                    let e = try!( self.semantic_checker.find_var(t.text.clone()) );
                                    match e.get_kind() {
                                        KindIdentifier::Typedef => {},
                                        _ => {return Err( CompilerErrors::from(SemanticErrors::OtherError{msg: "Ожидался тип".to_string()}) )}
                                    } 
                                    Ok( e.get_type().unwrap() )
                                 } ]) {             
            Some(res) => Ok(try!(res)),
            None => { Err( missing_operand(t.coords.x, t.coords.y) ) }
        }
    }

    fn parse_enum(&mut self, _t: &Token, name: String) -> TypeResult {
        let enum_name = if name == "None" {
            self.last_unknownenum_id += 1;
            "UnknownEnum #".to_string() + &self.last_unknownenum_id.to_string() 
        } 
        else { name };

        let mut var_list: Vec< String > = vec![];
        let mut value_map: HashMap< String, i64 > = HashMap::new();

        let t = self.tokenizer.current.clone();
        let name = t.text.clone();
        check_token!(self, TokenType::TId);
        var_list.push( t.value.as_string() );

        let t = self.tokenizer.current.clone();
        match t.token_type {
            TokenType::TEq => {
                try!(self.tokenizer.my_next());
                let expr = try!( self.parse_simple_expr() );
                let value = expr.get_type().unwrap().as_integer();
                match value {
                    Some(res) => { value_map.insert(name, res); },
                    None => { return Err( CompilerErrors::from(SemanticErrors::OtherError{msg: "Неверный тип выражения".to_string()}) ) }
                }
            },
            _ => {}
        }

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TCp]);
            check_token!(self, TokenType::TComma);

            let t = self.tokenizer.current.clone();
            check_token!(self, TokenType::TId);
            let name = t.text.clone();
            var_list.push( name.clone() );

            let t = self.tokenizer.current.clone();
            match t.token_type {
                TokenType::TEq => {
                    try!(self.tokenizer.my_next());
                    let expr = try!( self.parse_simple_expr() );
                    let value = expr.get_type().unwrap().as_integer();
                    match value {
                        Some(res) => { value_map.insert(name, res); },
                        None => { return Err( CompilerErrors::from(SemanticErrors::OtherError{msg: "Неверный тип выражения".to_string()}) ) }
                    }
                },
                _ => {}
            }
        }
        check_token!(self, TokenType::TCp);

        for i in 0..var_list.len() {
            let name = var_list[i].clone();
            let variant_type = Rc::new(EnumType::new( var_list.clone(), value_map.clone(), enum_name.clone(), i as i64 )) as Rc<Type>;
            let variant = Rc::new(DeclConstVarNode::new(name, variant_type)) as Rc<Node>;
            try!( self.semantic_checker.create_var( variant, 0 ) );
        }
        let e = EnumType::new( var_list.clone(), value_map.clone(), enum_name, 0 );

        Ok( Rc::new(e) )
    }

    fn parse_range(&mut self) -> TypeResult {
        let l = try!(self.parse_simple_expr()).get_type().unwrap();
        check_token!(self, TokenType::TRange);
        let r = try!(self.parse_simple_expr()).get_type().unwrap();

        
        match l.as_integer() {
            Some(l_value) => {
                let self_type = l.get_clone();
                match r.as_integer() {
                    Some(r_value) => { return Ok( Rc::new(try!(RangeType::new(l_value as i32, r_value as i32, self_type))) ) }
                    None => { }
                }
            },
            None => { }
        };

        match l.as_enum_without_name() {
            Some(l_value) => {
                let self_type = l.get_clone();
                match r.as_enum_without_name() {
                    Some(r_value) => { return Ok( Rc::new(try!(RangeType::new(l_value as i32, r_value as i32, self_type))) ) }
                    None => { }
                }
            },
            None => { }
        }

        match l.as_char() {
            Some(l_value) => {
                let self_type = l.get_clone();
                match r.as_char() {
                    Some(r_value) => { return Ok( Rc::new(try!(RangeType::new(l_value as i32, r_value as i32, self_type))) ) }
                    None => { }
                }
            },
            None => { }
        }

        return Err( CompilerErrors::from(SemanticErrors::OtherError{msg: format!("Нельзя создать range из {} и {}", l.as_str(), r.as_str())}) )
    }

    fn parse_record(&mut self, t: &Token, name: String) -> TypeResult {
        self.semantic_checker.add_scope();

        let record_name = if name == "None" {
            self.last_unknownrecord_id += 1;
            "UnknownRecord #".to_string() + &self.last_unknownrecord_id.to_string() 
        } 
        else { name };

        let e = try!( self.parse_var_declaration_list(t, record_name) );

        check_token!(self, TokenType::TEnd);
        try!( self.semantic_checker.remove_scope() );

        Ok( e.get_type().unwrap() )
    }

    fn parse_array(&mut self, _t: &Token) -> TypeResult {
        let t = self.tokenizer.current.clone();
        let index_type = match t.token_type {
            TokenType::TObr => { 
                try!(self.tokenizer.my_next());
                let res = try!( self.parse_type("None".to_string()) );
                check_token!(self, TokenType::TCbr);

                res
            },
            _ => { return Err( CompilerErrors::from(SemanticErrors::OtherError{msg: "НЕ ОШИБКА!!!".to_string()}) ) } //ДОДЕЛАТЬ!!!!!!!!!
        };

        check_token!(self, TokenType::TOf);

        let out_type = try!( self.parse_type("None".to_string()) );

        let res = try!(ArrayType::new(index_type.clone(), out_type.clone()));
        Ok( Rc::new(res) )
    }

    fn parse_statements(&mut self, _t: &Token) -> NodeResult {
        let mut e = ProgramNode::new("Statements".to_string());

        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TEnd, TokenType::TEof]);

            let child = match parse!(self, &t, [ TokenType::TId       => { self.parse_simple_stmt(&t) },
                                                 TokenType::TFor      => { self.parse_for(&t) },
                                                 TokenType::TIf       => { self.parse_if() },
                                                 TokenType::TWhile    => { self.parse_while() },
                                                 TokenType::TRepeat   => { self.parse_repeat() },
                                                 TokenType::TContinue => { self.parse_break_continue("Continue".to_string()) },
                                                 TokenType::TBreak    => { self.parse_break_continue("Break".to_string()) },
                                                 TokenType::TBegin    => { 
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

        Ok( Rc::new(e) )
    }

    fn parse_break_continue(&mut self, kind: String) -> NodeResult {
        if self.in_circle {
            try!( self.tokenizer.my_next() );
            Ok( Rc::new( ContinueBreakNode::new(kind) ) as Rc<Node> )
        }
        else {
            Err( CompilerErrors::from(SemanticErrors::OtherError{ msg: kind + " вызван не в цикле " }) ) 
        }
        
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
            None => Err(self.expected_token(curr_t.coords.x, curr_t.coords.y, TokenType::TSemicolom))
        }
    }

    fn parse_assign(&mut self, target: Rc< Node >) -> NodeResult {
        let op = self.tokenizer.current.clone();

        try!( self.tokenizer.my_next() );
        let object = try!(self.parse_simple_expr());

        let e = try!( AssignNode::new(op, target.clone(), object.clone()) );
        Ok( Rc::new(e) )
    }

    fn parse_for(&mut self, _t: &Token) -> NodeResult {
        try!( self.tokenizer.my_next() );
        

        let t = try!( self.tokenizer.get_and_next() );
        let id = parse_simple!(self, &t, [ TokenType::TId => { self.parse_simple_id(t.text.clone()) } ], expected_token);
        let id = try!( id );

        check_token!(self, TokenType::TAssign);

        let start = self.parse_simple_expr();
        let start = try!( start );

        check_token!(self, TokenType::TTo);

        let finish = self.parse_simple_expr();
        let finish = try!( finish );

        check_token!(self, TokenType::TDo);
        
        self.in_circle = true;

        let t = try!( self.tokenizer.get_and_next() );
        let block = parse_simple!(self, &t, [ TokenType::TBegin => { self.parse_statements(&t) } ], expected_token);
        let block =  try!( block );

        self.in_circle = false;

        let e = try!( ForNode::new(id, start, finish, block) );
        
        Ok( Rc::new(e) )
    }

    fn parse_repeat(&mut self) -> NodeResult {
        try!( self.tokenizer.my_next() );
        
        self.in_circle = true;

        let curr_t = try!( self.tokenizer.get_and_next() );
        let block = parse_simple!(self, &curr_t, [ TokenType::TBegin => { self.parse_statements(&curr_t) } ], expected_token);
        let block = try!( block );

        self.in_circle = false;

        check_token!(self, TokenType::TSemicolom);
        check_token!(self, TokenType::TUntil);

        let cond = try!( self.parse_simple_expr() );

        let e = try!( RepeatNode::new(cond, block) );
        Ok( Rc::new(e) )
    }

    fn parse_while(&mut self) -> NodeResult {
        try!( self.tokenizer.my_next() );
    
        let cond = try!( self.parse_simple_expr() );

        check_token!(self, TokenType::TDo);

        self.in_circle = true;

        let curr_t = try!( self.tokenizer.get_and_next() );
        let block = parse_simple!(self, &curr_t, [ TokenType::TBegin => { self.parse_statements(&curr_t) } ], expected_token);
        let block = try!( block );

        self.in_circle = false;

        let e = try!( WhileNode::new(cond, block) );
        Ok( Rc::new(e) )
    }

    fn parse_if(&mut self) -> NodeResult {
        try!( self.tokenizer.my_next() );
        
        let cond = try!( self.parse_simple_expr() );

        check_token!(self, TokenType::TThen);

        let curr_t = try!( self.tokenizer.get_and_next() );
        let block = parse_simple!(self, &curr_t, [ TokenType::TBegin => { self.parse_statements(&curr_t) } ], expected_token);
        let block = try!( block );
 
        let curr_t = self.tokenizer.current.clone();
        let else_block = match parse!(self, &curr_t, [TokenType::TElse => {
            try!(self.tokenizer.my_next());
            self.parse_stmts(&curr_t) 
        } ]) {
            Some(res) => Some( try!(res) ),
            None => { None }
        };

        let e = try!( IfNode::new(cond, block, else_block) );

        Ok( Rc::new(e) )
    }

    fn parse_stmts(&mut self, _t: &Token) -> NodeResult {
        let curr_t = try!( self.tokenizer.get_and_next() );
        parse_simple!(self, &curr_t, [ TokenType::TBegin => { self.parse_statements(&curr_t) } ], expected_token)
    }

    pub fn parse_simple_expr(&mut self) -> NodeResult {
        parse_bin!(self, parse_expr, [ TokenType::TGe, TokenType::TGt,
                                       TokenType::TEq, TokenType::TLe,
                                       TokenType::TLt, TokenType::TNe ])
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
                                 TokenType::TChar   => { self.parse_char(&t) },
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
        Ok( Rc::new(try!(UnaryOpNode::new(t.clone(), child))) )
    }

    fn parse_double(&mut self, t: &Token) -> NodeResult { 
        let self_type = Rc::new(DoubleType::new(t.value.as_double())) as Rc<Type>;
        Ok( Rc::new(ConstNode::new(self_type.clone())) )
    }
    fn parse_int(&mut self, t: &Token) -> NodeResult { 
        let self_type = Rc::new(IntegerType::new(t.value.as_int())) as Rc<Type>;
        Ok( Rc::new(ConstNode::new(self_type.clone())) )
    }
    fn parse_char(&mut self, t: &Token) -> NodeResult { 
        let value = t.value.as_string().as_bytes()[0];
        let self_type = Rc::new(CharType::new(value)) as Rc<Type>;
        Ok( Rc::new(ConstNode::new(self_type.clone())) )
    }
    fn parse_id(&mut self, t: &Token) -> NodeResult { 
        let name = t.text.clone();
        let mut e = try!( self.parse_simple_id(name.clone()) );
        loop {
            let curr_t = self.tokenizer.current.clone(); 
            let name = t.text.clone();

            e = match parse!(self, &curr_t, [ TokenType::TOp    => { self.parse_func_call(name) },
                                              TokenType::TObr   => { self.parse_array_element( e.clone() ) },
                                              TokenType::TPoint => { self.parse_record_field( e.clone() ) }  ]) {
                Some(node) => try!(node),
                None => { return Ok( e ) }
            }
        }
    }
    fn parse_simple_id(&mut self, name: String) -> NodeResult { 
        let e = try!( self.semantic_checker.find_id(name) );
        match e.get_kind() {
            KindIdentifier::Typedef | KindIdentifier::Other  => { 
                Err( CompilerErrors::from(SemanticErrors::OtherError{msg: "Недопустимое выражение".to_string()}) ) 
            },
            _ => { 
                let rc_node = e.clone();
                Ok( Rc::new((IdNode::new( rc_node as Rc<Node> ))))
            }
        }
        
    }

    fn parse_record_field(&mut self, parent: Rc< Node >) -> NodeResult {
        try!(self.tokenizer.my_next());
        let t = try!( self.tokenizer.get_and_next() ); 
        let field_name = t.text.clone();

        let e = try!( RecordFieldNode::new(parent.clone(), field_name) );
        Ok( Rc::new(e) )
    }

    fn parse_array_element(&mut self, parent: Rc< Node >) -> NodeResult {
        try!(self.tokenizer.my_next());

        let index = try!( self.parse_simple_expr() );
        check_token!(self, TokenType::TCbr);

        let e = try!( ArrayElementNode::new(parent.clone(), index.clone()) );
        Ok( Rc::new(e) )
    }


    fn parse_func_call(&mut self, name: String) -> NodeResult {
        let t = self.tokenizer.after.clone();
        let ttype = match t.token_type {
            TokenType::TCp => {
                try!(self.tokenizer.my_next());  
                Rc::new( FunctionType::new(vec![], Rc::new( VoidType::new() )) )
            },
            _ => { try!( self.parse_arg_list() ) }
        };
        try!(self.tokenizer.my_next());

        let func = try!( self.semantic_checker.find_override(name.clone(), ttype) );
        let e = try!( FunctionCallNode::new(name, func) );
        Ok( Rc::new(e) )
    }

    fn parse_arg_list(&mut self) -> TypeResult {
        let mut args: Vec< Rc<Type> > = vec![];

        let t = try!( self.tokenizer.get_and_next() ); 
        let child = parse_simple!(self, &t, [TokenType::TOp => { self.parse_simple_expr() } ], expected_token);

        args.push( try!(child).get_type().unwrap() );
        
        loop {
            let t = self.tokenizer.current.clone();
            break_if!(t.token_type == [TokenType::TCp]);
            try!(self.tokenizer.my_next());

            let child = parse_simple!(self, &t, [TokenType::TComma => { self.parse_simple_expr() } ], expected_token);
            args.push( try!(child).get_type().unwrap() );
        }

        let e = FunctionType::new(args, Rc::new( VoidType::new() ));

        Ok( Rc::new((e)) )
    }

    fn parse_op_in_expr(&mut self, _t: &Token) -> NodeResult { 
        let e = self.parse_simple_expr();
        check_token!(self, TokenType::TCp);
        return e;
    }


    pub fn expected_token(&self, x: i32, y: i32, token_type: TokenType ) -> CompilerErrors {
    let token = match token_type {
        TokenType::TSemicolom => { ";".to_string() },
        TokenType::TColon     => { ":".to_string() },
        TokenType::TComma     => { ",".to_string() },
        TokenType::TObr       => { "[".to_string() },
        TokenType::TOp        => { "(".to_string() },
        TokenType::TCbr       => { "]".to_string() },
        TokenType::TCp        => { ")".to_string() },
        TokenType::TPoint     => { ".".to_string() },
        TokenType::TEq        => { "=".to_string() },
        TokenType::TRange     => { "..".to_string() },
        TokenType::TBegin     => { "begin".to_string() },
        TokenType::TEnd       => { "end".to_string() },
        TokenType::TThen      => { "then".to_string() },
        TokenType::TDo        => { "do".to_string() },
        TokenType::TOf        => { "of".to_string() },
        TokenType::TId        => { "идентификатор".to_string() },
        TokenType::TInt       => { "целое число".to_string() },
        TokenType::TDouble    => { "вещественное число".to_string() },
        TokenType::TChar      => { "символ".to_string() },
        _                     => { "".to_string() }
    };
    let err = ParserErrors::ExpectedToken{ x, y, token };
    CompilerErrors::ParserError{ err }
}
}