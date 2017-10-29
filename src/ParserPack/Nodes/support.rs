use std::fmt::Display;
use std::rc::Rc;


pub trait Node: Printable + Display {
	fn add_child(&mut self, new_node: Box<Node>) {}
}

pub trait Printable {
	fn get_children(&self) -> Vec<Rc<Box<Node>>>;
	fn get_name(&self) -> String;
	fn as_str(&self, indent: String, is_tail: bool) -> String {
		let childrens = self.get_children();
		let name = self.get_name();

		let mut ans = if is_tail {
			indent.clone() + "└── " + &*name + "\n"
		}
		else {
			indent.clone() + "├── " + &*name + "\n"
		};

		let addition = if is_tail {	"    " } else { "│   " };
		if childrens.len() == 0 { return ans; }
		
        for i in (0 .. childrens.len() - 1) {
            ans += &*childrens[i].as_str(indent.clone() + &addition, false);
        }
        if childrens.len() > 0 {
            ans += &*childrens[childrens.len() - 1].as_str(indent.clone() + &addition, true);
        }

        ans
	}
}