use std::collections::HashMap;
use std::rc::Rc;
use support::SemanticErrors;
use ParserPack::*;

type Overrides = HashMap<String, Rc< Node >>;

struct Scope {
	vars: HashMap< String, Rc< Node > >, 
	functions: HashMap< String, Overrides >,

	pub count_forward: i32,
}

impl Scope {
	pub fn new() -> Scope { Scope{vars: HashMap::new(), functions: HashMap::new(), count_forward: 0 } }
	pub fn add_id(&mut self, id: Rc< Node > ) -> Result< String, SemanticErrors > {
		match id.get_kind() {
		    KindIdentifier::Function | 
		    KindIdentifier::ForwardFunction => { try!( self.add_function(id.clone()) ); },
			_ => { try!( self.add_var(id.clone()) ); },
		};

		Ok( "Ok".to_string() )
	}

	pub fn add_var(&mut self, id: Rc< Node >) -> Result< String, SemanticErrors > {
		match self.vars.get(&id.get_name()) {
			Some(_res) => return Err( SemanticErrors::DuplicateIdentifier{name: id.get_name()} ),
			None => {}
		};	

		match self.functions.get(&id.get_name()) {
			Some(_res) => return Err( SemanticErrors::DuplicateIdentifier{name: id.get_name()} ),
			None => {}
		};	

		self.vars.insert(id.get_name(), id.clone());

		Ok( "Ok".to_string() )
	}

	pub fn add_function(&mut self, id: Rc< Node >) -> Result< String, SemanticErrors > {
		let sign = id.get_type().unwrap().as_str();
		let name = id.get_name();

		match self.vars.get(&name) {
			Some(_res) => return Err( SemanticErrors::DuplicateIdentifier{name: name.clone()} ),
			None => {}
		};	

		if self.functions.contains_key(&name) {
			let overrides = self.functions.get_mut(&name).unwrap();
			let over = overrides.insert( sign.clone(), id.clone() );

			if !over.is_none() {
				match over.unwrap().get_kind() {
					KindIdentifier::ForwardFunction => {
						match id.get_kind() {
							KindIdentifier::Function => { self.count_forward -= 1; },
							_ => return Err( SemanticErrors::OtherError{msg: "Ошибка при перегрузке функции".to_string()} )
						};
					},
					_ => return Err( SemanticErrors::OtherError{msg: "Ошибка при перегрузке функции".to_string()} )
				}
			}
		}
		else {
			let mut overrides: Overrides = HashMap::new();
			overrides.insert( sign.clone(), id.clone() );
			self.functions.insert(name, overrides);
		}

		match id.get_kind() {
		    KindIdentifier::ForwardFunction => { self.count_forward += 1; }, 
			_ => {  },
		};

		Ok("Ok".to_string())
	}

	pub fn get_var(&self, name: &str) -> Option< Rc< Node > > {
		match self.vars.get(name) {
			Some(item) => Some((*item).clone()),
			None => None
		}
	}

	pub fn get_overrides(&self, name: &str) -> Option< Overrides > {
		match self.functions.get(name) {
			Some(item) => return Some((*item).clone()),
			None => {}
		}

		None
	}
}

fn create_system_scope() -> Box< Scope > {
	let mut scope = Box::new( Scope::new() );

	let var_list = vec!["false".to_string(), "true".to_string()];
	let value_map: HashMap< String, i64 > = HashMap::new();

	let tboolean = Rc::new( EnumType::new( var_list.clone(), value_map.clone(), "boolean".to_string(), 0 )) as Rc<Type>;
	let tfalse   = Rc::new( EnumType::new( var_list.clone(), value_map.clone(), "boolean".to_string(), 0 )) as Rc<Type>;
	let ttrue    = Rc::new( EnumType::new( var_list.clone(), value_map.clone(), "boolean".to_string(), 1 )) as Rc<Type>;

	let nboolean = Rc::new( TypedefNode::new("boolean".to_string(), tboolean)) as Rc<Node>;
	let nfalse   = Rc::new( DeclConstVarNode::new("false".to_string(), tfalse)) as Rc<Node>;
	let ntrue    = Rc::new( DeclConstVarNode::new("true".to_string(), ttrue)) as Rc<Node>;


	scope.add_id( nboolean ).unwrap();
	scope.add_id( ntrue ).unwrap();
	scope.add_id( nfalse ).unwrap();

	scope
}

pub struct SemanticChecker {
	system_scope: Box< Scope >,
	scopes:       Vec< Box<Scope> >,
}

impl SemanticChecker {
	pub fn new() -> SemanticChecker { SemanticChecker{ system_scope: create_system_scope(), scopes: vec![ Box::new(Scope::new()) ] } }
	pub fn add_scope(&mut self) { self.scopes.push( Box::new(Scope::new()) ); }
	pub fn remove_scope(&mut self) -> Result< String, SemanticErrors > { 
		let scope = self.scopes.pop().unwrap();

		if scope.count_forward > 0 {
			for (name, overrides) in scope.functions {
				for (sign, over) in &overrides {
					match over.get_kind() {
		    			KindIdentifier::ForwardFunction => { 
		    				return Err( SemanticErrors::ErrorInForwardDecl{name: name.to_string(), sign: sign.to_string()} ) 
		    			}, 
						_ => {  },
					};
				}
			}
		}

		Ok("Ok".to_string())
	}

	pub fn get_id(&self, name: String) -> Option< Rc< Node > > { 
		match self.system_scope.get_var(&name) {
			Some(res) => return Some(res),
			None => {}
		}

		match self.system_scope.get_overrides(&name) {
			Some(res) => {
				for (_sign, func) in res { return Some( func.clone() ) }
			},
			None => {}
		}

		let last = self.scopes.len() - 1;
		for i in (0..last + 1).rev() {
			match self.scopes[i].get_var(&name) {
				Some(res) => return Some(res),
				None => {}
			};

			match self.scopes[i].get_overrides(&name) {
				Some(res) => {
					for (_sign, func) in res { return Some( func.clone() ) }
				},
				None => {}
			};
		}
		None
	}

	pub fn get_var(&self, name: String) -> Option< Rc< Node > > { 
		match self.system_scope.get_var(&name) {
			Some(res) => return Some(res),
			None => {}
		}

		let last = self.scopes.len() - 1;
		for i in (0..last + 1).rev() {
			match self.scopes[i].get_var(&name) {
				Some(res) => return Some(res),
				None => {}
			}
		}
		None
	}

	pub fn get_overrides(&self, name: String) -> Option< Overrides > { 
		match self.system_scope.get_overrides(&name) {
			Some(res) => return Some(res),
			None => {}
		}

		let last = self.scopes.len() - 1;
		for i in (0..last + 1).rev() {
			match self.scopes[i].get_overrides(&name) {
				Some(res) => return Some(res),
				None => {}
			}
		}
		None
	}

	pub fn add_id(&mut self, id: Rc< Node >, shift: i32) -> Result< String, SemanticErrors > {
		let last = self.scopes.len() - 1;
		self.scopes[last - shift as usize].add_id( id )
	}

	pub fn create_var(&mut self, id: Rc< Node >, shift: i32) -> Result< Rc< Node >, SemanticErrors > {
		match self.add_id(id.clone(), shift) {
			Ok(_res) => Ok(id),
			Err(err) => Err(err)
		}
		
	}

	pub fn create_typedef(&mut self, id: Rc< Node >, shift: i32) -> Result< Rc< Node >, SemanticErrors > {
		self.create_var(id, shift)
	}

	pub fn create_function(&mut self, id: Rc< DeclFunctionNode >, shift: i32) -> Result< Rc< Node >, SemanticErrors > {
		self.create_var(id as Rc<Node>, shift)	
	}

	pub fn find_var(&mut self, name: String) -> Result< Rc< Node >, SemanticErrors > {
		match self.get_var(name.clone()) {
			Some(res) => { Ok( res ) },
			None => return Err( SemanticErrors::UnknownIdentifier{name} )
		}
	}

	pub fn find_id(&mut self, name: String) -> Result< Rc< Node >, SemanticErrors > {
		match self.get_id(name.clone()) {
			Some(res) => { Ok( res ) },
			None => return Err( SemanticErrors::UnknownIdentifier{name} )
		}
	}

	pub fn find_override(&mut self, name: String, func: Rc< Type > ) -> Result< Rc< Type >, SemanticErrors > {
		let sign = func.as_str();

		let overrides = match self.get_overrides(name.clone()) {
			Some(res) => { res },
			None => return Err( SemanticErrors::NotAFunction{name} )
		};

		for (_name, over) in overrides {
			let ttype = over.get_type().unwrap();
			match func.cast_to(ttype) {
				Ok(res) => { return Ok( res.clone() ) },
				Err(err) => { return Err( err ) }
			};
		}

		Err( SemanticErrors::UnknownOverride{name, sign} )
	}
}