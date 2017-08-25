mod syntax;
mod codegen;

use syntax::ast::Expr;
use syntax::ast::Constant;
use codegen::llvm::generator::LLVMIRGenerator;

fn main() {
    println!("{}", LLVMIRGenerator::generate(Expr::Const(Constant::Nil)));
}
