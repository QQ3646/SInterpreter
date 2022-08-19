use std::{env, io};
use std::fs::File;
use std::io::Write;

mod Expr;

struct Field {
    name: String,
    type_: String,
}

fn define_ast(base_name: &str, types: Vec<&'static str>) -> io::Result<()> {
    let mut file = File::create(format!("{base_name}.rs")).expect("Failed to create file.");

    writeln!(&mut file, "//That file created by \"metaprogramming\" package.\n")?;

    writeln!(&mut file, "mod ast {{")?;

    writeln!(&mut file, "    enum Object {{")?;
    writeln!(&mut file, "        Number(f64),")?;
    writeln!(&mut file, "        Str(String),")?;
    writeln!(&mut file, "    }}")?;
    writeln!(&mut file)?;


    writeln!(&mut file, "    enum {base_name} {{")?;
    for type_raw in types {
        let mut type_data = type_raw.split(":");
        let type_name = type_data.nth(0).unwrap().trim();
        let type_fileds = type_data.nth(0).unwrap().trim();
        define_type(&mut file, base_name, type_name, type_fileds)?;
    }
    writeln!(&mut file, "    }}")?;

    writeln!(&mut file, "}}")?;

    define_visitor(&mut file, base_name)?;
    Ok(())
}

fn define_type(file: &mut File, base_name: &str, enum_name: &str, fields: &'static str) -> io::Result<()> {

    writeln!(file, "        {enum_name} {{")?;
    for field in fields.split(", ") {
        let mut field = field.split(" ");
        let mut field = Field {
            type_: field.nth(0).unwrap().to_string(),
            name: field.nth(0).unwrap().to_string(),
        };
        if field.type_ == base_name {
            field.type_ = format!("Option<Box<{}>>", field.type_);
        }
        writeln!(file, "            {}: {},", field.name, field.type_)?;
    }
    writeln!(file, "        }},")?;
    Ok(())
}

fn define_visitor(file: &mut File, base_name: &str) -> io::Result<()> {
    writeln!(file, "mod visitor {{")?;
    writeln!(file, "    use super::ast::*;\n")?;
    writeln!(file, "    pub trait Visitor<T> {{")?;

    writeln!(file, "        fn visit_{}(&mut self, {}: &{}) -> T;", base_name.to_lowercase(), base_name.to_lowercase(), base_name)?;

    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    Ok(())
}

fn main() {
    // AST generator
    define_ast("Expr",
               vec!["Binary   : Expr left, Token operator, Expr right",
                    "Grouping : Expr expression",
                    "Literal  : Object value",
                    "Unary    : Token operator, Expr right"]);
}
