use std::fmt;
use TokenizerPack::support::*;

pub trait Node {
	fn as_str(&self, indent: String, is_right: bool, depth: i32) -> String;
}

pub struct Tree {
	pub root: Box<Node>
}

impl Tree {
	pub fn new(root: Box<Node>) -> Tree {
		Tree {
			root
		}
	}
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let ans = self.root.as_str("".to_string(), true, 0);
    	write!(f, "{}", ans)
    }
}


pub struct BinNode {
	pub left: Box<Node>,
	pub right: Box<Node>,

	pub op: Token,
}

impl BinNode {
	pub fn new(left: Box<Node>, right: Box<Node>, op: Token) -> BinNode {
		BinNode {
			left,
			right, 

			op: op,
		}
	}
}

impl Node for BinNode {
	fn as_str(&self, indent: String, is_right: bool, depth: i32) -> String {
		let mut ans = "".to_string();

		if depth == 0 {
			ans += &(self.right.as_str("".to_string(), true, depth + 1) + "|\n");
			ans += &self.op.value;
			ans += &("\n|\n".to_string() + &self.left.as_str("".to_string(), false, depth + 1));
		}
		else {
			if is_right {
				ans = self.right.as_str(indent.clone() + "     ", true, depth + 1);
				ans += &(indent.clone() + "     |\n");
			}
			else {
				ans = self.right.as_str(indent.clone() + "|    ", true, depth + 1);
				ans += &(indent.clone() + "|    |\n");
			}

			ans += &(indent.clone() + "#----");
			ans += &self.op.value;
			ans += "\n";

			if is_right {
				ans += &(indent.clone() + "|    |\n");
				ans += &self.left.as_str(indent.clone() + "|    ", false, depth + 1);
			}
			else {
				ans += &(indent.clone() + "     |\n");
				ans += &self.left.as_str(indent.clone() + "     ", false, depth + 1);
			}
		}

		ans
	}
}

pub struct ConstNode<T> {
	pub value: T
}

impl<T> ConstNode<T> {
	pub fn new(value: T) -> ConstNode<T> {
		ConstNode {
			value: value
		}
	}
}

impl<T: fmt::Display> Node for ConstNode<T> {
	fn as_str(&self, indent: String, is_right: bool, depth: i32) -> String {
		let mut ans = "".to_string();

		if depth != 0 {
			ans += &(indent.clone() + "#----");
		}

		ans += &self.value.to_string();
		ans += "\n";
		ans
	}
}

pub struct IdNode {
	pub value: String
}

impl IdNode {
	pub fn new(value: String) -> IdNode {
		IdNode {
			value: value
		}
	}
}

impl Node for IdNode {
	fn as_str(&self, indent: String, is_right: bool, depth: i32) -> String {
		let mut ans = "".to_string();

		if depth != 0 {
			ans += &(indent.clone() + "#----");
		}

		ans += &self.value;
		ans += "\n";
		ans
	}
}