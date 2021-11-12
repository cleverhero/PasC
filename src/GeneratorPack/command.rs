use TokenizerPack::*;

pub struct Command {
    text: String,
}

impl Command {
    pub fn create_int_var(name: String) -> Command {
        let text = "v_".to_string() + &name + " : dd 0";
        Command { text }
    }

    pub fn create_float_var(name: String) -> Command {
        let text = "v_".to_string() + &name + " : dq 0";
        Command { text }
    }

    pub fn create_char_var(name: String) -> Command {
        let text = "v_".to_string() + &name + " : db 0";
        Command { text }
    }

    pub fn create_format_string(name: String, format: String) -> Command {
        let text = name + " : db \"" + &format + "\", 0xA, 0x0";
        Command { text }
    }

    pub fn create_push_const(value: i32) -> Command {
        let text = "push ".to_string() + &value.to_string();
        Command { text }
    }

    pub fn create_push_global_var(name: String) -> Command {
        let text = "push ".to_string() + &name;
        Command { text }
    }

    pub fn create_push_global_var_by_addr(name: String) -> Command {
        let text = "push dword [v_".to_string() + &name + "]";
        Command { text }
    }

    pub fn create_call_func(name: String) -> Command {
        let text = "call ".to_string() + &name;
        Command { text }
    }

    pub fn create_decl_function(name: String) -> Command {
        let text = "global _".to_string() + &name + "\n_" + &name + ":";
        Command { text }
    }

    pub fn create_int_op(op: TokenType) -> Command {
        let mut text = "pop ebx\n".to_string();
        text += "pop eax\n";

        text += match op {
            TokenType::TPlus => "add eax, ebx\n",
            TokenType::TMinus => "sub eax, ebx\n",
            TokenType::TMul => "imul eax, ebx\n",
            TokenType::TAnd => "and eax, ebx\n",
            TokenType::TOr => "or eax, ebx\n",
            _ => "",
        };
        text += "push eax";

        Command { text }
    }

    pub fn create_int_unar_op(op: TokenType) -> Command {
        let mut text = "pop eax\n".to_string();

        text += match op {
            TokenType::TPlus => "",
            TokenType::TMinus => "neg eax\n",
            _ => "",
        };
        text += "push eax";

        Command { text }
    }

    pub fn create_float_op(op: TokenType) -> Command {
        let mut text = match op {
            TokenType::TPlus => "faddp\n",
            TokenType::TMinus => "fchs\n",
            TokenType::TMul => "fmulp\n",
            TokenType::TShare => "fdivp\n",
            _ => "",
        }.to_string();
        text += "sub esp, 4\n";
        text += "fstp DWORD [esp]";

        Command { text }
    }

    pub fn create_float_unar_op(op: TokenType) -> Command {
        let mut text = match op {
            TokenType::TPlus => "",
            TokenType::TMinus => "fchs\n",
            _ => "",
        }.to_string();
        text += "sub esp, 4\n";
        text += "fstp DWORD [esp]";

        Command { text }
    }

    pub fn create_assing(name: String) -> Command {
        let mut text = "pop eax\n".to_string();
        text += &("mov [v_".to_string() + &name + "], eax\n");
        Command { text }
    }

    pub fn create_ret(shift: i32) -> Command {
        let text = "ret ".to_string() + &shift.to_string();
        Command { text }
    }

    pub fn create_clear_stack(shift: i32) -> Command {
        let text = "add esp, ".to_string() + &shift.to_string();
        Command { text }
    }

    pub fn create_push_to_fld() -> Command {
        let mut text = "fld DWORD [esp]\n".to_string();
        text += "pop eax\n";
        Command { text }
    }

    pub fn create_float_to_double() -> Command {
        let mut text = "fld DWORD [esp]\n".to_string();
        text += "pop eax\n";
        text += "sub esp, 8\n";
        text += "fstp QWORD [esp]\n";
        Command { text }
    }

    pub fn as_str(&self) -> String {
        self.text.clone()
    }
}
