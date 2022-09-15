pub mod asm;
pub mod code_gen;
pub mod parser;

use code_gen::Generator;
use code_gen::InterRep::*;
use code_gen::Node::*;
use code_gen::Type::*;

use std::{fs::File, io, io::Write};

fn main() -> io::Result<()> {
    let mut generator = Generator::new();
    generator.apply(vec![
        Define("x".to_string(), Str("`%d\\n`".to_string())),
        Define("y".to_string(), Literal(Bool(true))),
        CCall(
            "printf".to_string(),
            vec![Ident("x".to_string()), Ident("y".to_string())],
        ),
    ]);

    let mut file = File::create("out.asm")?;
    file.write_all(&generator.export())?;

    Ok(())
}
