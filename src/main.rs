#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate NameByField;

mod TokenizerPack;
mod ParserPack;
mod support;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use support::*;
use TokenizerPack::tokenizer::Tokenizer;
use ParserPack::parser::Parser;

fn procerr_errors(err: CompilerErrors) -> String {
    match err {
        CompilerErrors::TokenizerError{err} => match err {
            TokenizerErrors::ErrorInInteger{ x, y } => { 
                format!("Ошибка в ({}, {}): Неверный формат целого числа", x, y)
            },
            TokenizerErrors::ErrorInHex{ x, y } => { 
                format!("Ошибка в ({}, {}): Неверный формат шестнадцатеричного числа", x, y)
            },
            TokenizerErrors::ErrorInBin{ x, y } => { 
                format!("Ошибка в ({}, {}): Неверный формат двоичного числа", x, y)
            },
            TokenizerErrors::ErrorInOctal{ x, y } => { 
                format!("Ошибка в ({}, {}): Неверный формат восьмиричного числа", x, y)
            },
            TokenizerErrors::ErrorInDouble{ x, y } => { 
                format!("Ошибка в ({}, {}): Неверный формат вешественного числа", x, y)
            },
            TokenizerErrors::ErrorInExp{ x, y } => { 
                format!("Ошибка в ({}, {}): Неверный формат экспоненты", x, y)
            },
            TokenizerErrors::UnknownCharCode{ x, y } => { 
                format!("Ошибка в ({}, {}): Неизвестный код символа", x, y)
            },
            TokenizerErrors::SimpleError{ x, y } => { 
                format!("Ошибка в ({}, {})", x, y)
            },
        },
        CompilerErrors::ParserError{err} => match err {
            ParserErrors::EmptyFile{ x, y } => { 
                format!("Ошибка в ({}, {}): Пустой файл", x, y)
            },
            ParserErrors::MissingOperand{ x, y } => { 
                format!("Ошибка в ({}, {}): Пропущен операнд", x, y)
            },
            ParserErrors::ExpectedToken{ x, y, token } => {
                format!("Ошибка в ({}, {}): Ожидалось {}", x, y, token)
            },
        },
    }
}

fn main() {
	let mut tokenizer_mode = false;
	let mut parser_mode = false;
	let mut infile_mode = false;
	let mut file = "".to_string();

	if env::args().len() == 1 {
	   println!("Приходько Олег. 2017 год.");
	}

	for arg in env::args() {
		if arg[0..1].to_string() != "-" {
			file = arg.to_string();
		}
		if arg == "-h" {
			println!("Приходько Олег. 2017 год.");
			println!("-h -> help");
			println!("-l file -> run tokenizer in file");
			return;
		}
		if arg == "-f" {
			infile_mode = true;
		}

		if arg == "-l" && !parser_mode {
			tokenizer_mode = true;
		}

		if arg == "-p" && !tokenizer_mode {
			parser_mode = true;
		}
	}

	

	if tokenizer_mode {
		let tokenizer = Tokenizer::new(file.clone());
		let mstr = file[0..file.len() - 4].to_string() + ".res";

		if infile_mode {
			let mut file = File::create(mstr).unwrap();
    		file.write_fmt(format_args!("\t{:6} {:6} {:15} {:25} {:25}\n", "Line", "Col", "Type", "Value", "Text")).unwrap();
    		for res in tokenizer {
   				match res {
   					Ok(token) => {
   						file.write_fmt(format_args!("{}\n", token)).unwrap();
   					}
   					Err(err) => {
                        let error = CompilerErrors::TokenizerError{err};
   						file.write_fmt(format_args!("{}\n", procerr_errors(error))).unwrap();
   						break;
   					}
   				}
			}
		} 
		else {
    		println!("\t{:6} {:6} {:15} {:25} {:25}", "Line", "Col", "Type", "Value", "Text");
   			for res in tokenizer {
   				match res {
   					Ok(token) => {
   						println!("{}", token);
   					}
   					Err(err) => {
                        let error = CompilerErrors::TokenizerError{err};
   						println!("{}", procerr_errors(error));
   						break;
   					}
   				}
			}
		}
    }
    else if parser_mode {
    	let mut tokenizer = Tokenizer::new(file.clone());
        tokenizer.next();
    	let mut parser = Parser::new(tokenizer);

		let mstr = file[0..file.len() - 4].to_string() + ".res";
    	let mut file = File::create(mstr).unwrap();

    	let tree = match parser.parse() {
    		Ok(val) => val,
    		Err(err) => {
    			if infile_mode {
    				file.write_fmt(format_args!("{}", procerr_errors(err))).unwrap();
    			}
    			else {
    				println!("{}", procerr_errors(err));
    			}
    			return;
    		}
    	};
    	
    	if infile_mode {
    		file.write_fmt(format_args!("{}", tree)).unwrap();
		} 
		else {
    		println!("{}", tree);
		}
    }
}
