use std::fmt::Display;
use std::rc::Rc;
use ParserPack::*;

pub enum KindIdentifier { Var, Const, Function, ForwardFunction, Typedef, Other } 

pub trait Node: PrintableNode + Display {
	fn add_child(&mut self, _new_node: Rc< Node >) {} //Удалить!!!!!!!!
	fn get_value(&self) -> String { "".to_string() }
	fn get_type(&self) -> Option< Rc<Type> > { None }
	fn get_name(&self) -> String { "".to_string() }
	fn get_kind(&self) -> KindIdentifier { KindIdentifier::Other }

	fn as_printable(&self) -> &PrintableNode;
}

pub trait PrintableNode {
	fn get_children(&self) -> Vec< &PrintableNode >;
	fn get_caption(&self) -> String;
	fn as_str(&self, indent: String, is_tail: bool) -> String {
		let childrens = self.get_children();
		let name = self.get_caption();

		let mut ans = if is_tail {
			indent.clone() + "└── " + &*name + "\n"
		}
		else {
			indent.clone() + "├── " + &*name + "\n"
		};

		let addition = if is_tail {	"    " } else { "│   " };
		if childrens.len() == 0 { return ans; }
		
        for i in 0 .. childrens.len() - 1 {
            ans += &*childrens[i].as_str(indent.clone() + &addition, false);
        }
        if childrens.len() > 0 {
            ans += &*childrens[childrens.len() - 1].as_str(indent.clone() + &addition, true);
        }

        ans
	}
}


pub struct Label { msg: String }

impl Label {
	pub fn new(msg: String) -> Label { Label{msg} }
}

impl PrintableNode for Label {
	fn get_children(&self) -> Vec< &PrintableNode > { vec![] }
	fn get_caption(&self) -> String { self.msg.clone() }
}