extern crate min_region;

use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::process::Command;
use min_region::syntax::ast::Expr;
use min_region::syntax::ast::Constant;
use min_region::codegen::llvm::generator::LLVMIRGenerator;

fn main() {
    let _ = fs::create_dir("tmp");
    let _ = fs::remove_file("tmp/test_case1.ll");
    let mut file = File::create("tmp/test_case1.ll").unwrap();
    let ir = LLVMIRGenerator::generate(Expr::Const(Constant::Nil));
    let _ = file.write_all(ir.as_bytes());

    let status = Command::new("lli")
        .arg("tmp/test_case1.ll")
        .status();

    match status {
        Ok(s) => println!("ExitStatus: {}", s.code().unwrap_or(-999)),
        Err(e) => println!("{}", e)
    }
}