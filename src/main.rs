#[macro_use]
extern crate NameByField;
#[macro_use]
extern crate lazy_static;

mod TokenizerPack;
mod ParserPack;
mod SemanticPack;
mod GeneratorPack;
mod support;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use support::*;
use TokenizerPack::tokenizer::Tokenizer;
use ParserPack::*;
use GeneratorPack::*;

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
            ParserErrors::MissingOperand{ x, y } => { 
                format!("Ошибка в ({}, {}): Пропущен операнд", x, y)
            },
            ParserErrors::ExpectedToken{ x, y, token } => {
                format!("Ошибка в ({}, {}): Ожидалось {}", x, y, token)
            },
        },
        CompilerErrors::SemanticError{err} => match err {
            SemanticErrors::CastError{ this, other } => { 
                format!("Ошибка: Нельзя преобразовать {} в {}", this, other)
            },
            SemanticErrors::DuplicateIdentifier{ name } => { 
                format!("Ошибка: Идентификатор {} уже определен", name)
            },
            SemanticErrors::UnknownOverride{ name, sign } => { 
                format!("Ошибка: Не найдена перегрузка {} для {}", sign, name)
            },
            SemanticErrors::NotAFunction{ name } => { 
                format!("Ошибка: Нельзя вызвать {} как функцию", name)
            },
            SemanticErrors::ErrorInForwardDecl{ name, sign } => { 
                format!("Ошибка: Не найдено определение для {}{} ", name, sign)
            },
            SemanticErrors::UnknownIdentifier{ name } => { 
                format!("Ошибка: Идентификатор {} неизвестен", name)
            },
            SemanticErrors::ErrorInUnarOperation { name, op } => { 
                let msg = match op {
                    UnarOperation::Plus => { "унарный плюс".to_string() }, 
                    UnarOperation::Minus => { "унарный минус".to_string() }, 
                    UnarOperation::Not => { "унарное not".to_string() }
                };
                format!("Ошибка: Невозможно применить {} к {}", msg, name)
            },
            SemanticErrors::ErrorInBinOperation { left, right, op } => { 
                let msg = match op {
                    BinOperation::Plus  => { "сложить".to_string() }, 
                    BinOperation::Minus => { "отнять".to_string() }, 
                    BinOperation::Mul   => { "умножить".to_string() },
                    BinOperation::Share => { "поделить".to_string() }, 
                    BinOperation::And   => { "применить And".to_string() }, 
                    BinOperation::Or    => { "применить Or".to_string() },
                    _ =>  { "сравнить".to_string() },
                };
                format!("Ошибка: Невозможно {} {} c {}", msg, left, right)
            },
            SemanticErrors::OtherError{ msg } => { 
                format!("Ошибка: {}", msg)
            },
        },
    }
}

fn main() {
    let mut tokenizer_mode = false;
    let mut parser_mode = false;
    let mut generator_mode = false;
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

        if arg == "-g" && !tokenizer_mode {
            generator_mode = true;
        }
    }

    if tokenizer_mode {
        let tokenizer = Tokenizer::new(file.clone());
        let mstr = file[0..file.len() - 4].to_string() + ".res";

        if infile_mode {
            let mut file = File::create(mstr).unwrap();
            file.write_fmt(format_args!(
                "\t{:6} {:6} {:15} {:25} {:25}\n",
                "Line", "Col", "Type", "Value", "Text"
            )).unwrap();
            for res in tokenizer {
                match res {
                    Ok(token) => {
                        file.write_fmt(format_args!("{}\n", token)).unwrap();
                    }
                    Err(err) => {
                        let error = CompilerErrors::TokenizerError { err };
                        file.write_fmt(format_args!("{}\n", procerr_errors(error)))
                            .unwrap();
                        break;
                    }
                }
            }
        } else {
            println!(
                "\t{:6} {:6} {:15} {:25} {:25}",
                "Line", "Col", "Type", "Value", "Text"
            );
            for res in tokenizer {
                match res {
                    Ok(token) => {
                        println!("{}", token);
                    }
                    Err(err) => {
                        let error = CompilerErrors::TokenizerError { err };
                        println!("{}", procerr_errors(error));
                        break;
                    }
                }
            }
        }
    } else if parser_mode {
        let mut tokenizer = Tokenizer::new(file.clone());
        tokenizer.next();
        let mut parser = Parser::new(tokenizer);

        let mstr = file[0..file.len() - 4].to_string() + ".res";
        let mut file = File::create(mstr).unwrap();

        let tree = match parser.parse() {
            Ok(val) => val,
            Err(err) => {
                if infile_mode {
                    file.write_fmt(format_args!("{}", procerr_errors(err)))
                        .unwrap();
                } else {
                    println!("{}", procerr_errors(err));
                }
                return;
            }
        };

        if infile_mode {
            file.write_fmt(format_args!("{}", tree)).unwrap();
        } else {
            println!("{}", tree);
        }
    } else if generator_mode {
        let mut tokenizer = Tokenizer::new(file.clone());
        tokenizer.next();
        let mut parser = Parser::new(tokenizer);

        let mstr = file[0..file.len() - 4].to_string() + ".res";
        let mut file = File::create(mstr).unwrap();

        let tree = match parser.parse() {
            Ok(val) => val,
            Err(err) => {
                if infile_mode {
                    file.write_fmt(format_args!("{}", procerr_errors(err)))
                        .unwrap();
                } else {
                    println!("{}", procerr_errors(err));
                }
                return;
            }
        };

        let mut generator = Generator::new();
        tree.generate(&mut generator);

        if infile_mode {
            file.write_fmt(format_args!("{}", generator)).unwrap();
        } else {
            println!("{}", generator);
        }
    }
}
