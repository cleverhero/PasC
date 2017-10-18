use std::fmt;

pub trait Node {
	fn as_str(&self, indent: String, isRight: bool) -> String;
}



pub struct BinNode {
	pub left: Box<Node>,
	pub right: Box<Node>,

	pub op: String,
}

impl BinNode {
	pub fn new(left: Box<Node>, right: Box<Node>) -> BinNode {
		BinNode {
			left,
			right, 

			op: "*".to_string()
		}
	}
}

impl Node for BinNode {
	fn as_str(&self, indent: String, isRight: bool) -> String {
		let mut ans = "".to_string();
		if (isRight) {
			ans = self.right.as_str(indent.clone() + "     ", true);
		}
		else {
			ans = self.right.as_str(indent.clone() + "|    ", true);
		}
		if indent != "" {
			ans += &indent;
			if (isRight) { ans += "/"; } else { ans += "\\"; }
			ans += "-----";
		}
		
		ans += &self.op;
		ans += "\n";
		if (isRight) {
			ans += &self.left.as_str(indent.clone() + "|    ", false);
		}
		else {
			ans += &self.left.as_str(indent.clone() + "     ", false);
		}

		ans
	}
}

pub struct IntNode {
	pub value: i64
}

impl Node for IntNode {
	fn as_str(&self, indent: String, isRight: bool) -> String {
		let mut ans = "".to_string();
		if indent != "" {
			ans += &indent;
			if (isRight) { ans += "/"; } else { ans += "\\"; }
			ans += "-----";
		}
		ans += &self.value.to_string();
		ans += "\n";
		ans
	}
}

pub struct Tree {
	pub root: Box<Node>
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut ans = self.root.as_str("".to_string(), true);
    	write!(f, "{}", ans)
    }
}