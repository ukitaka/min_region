use syntax::ast::Expr;

pub struct LLVMIRGenerator;

impl LLVMIRGenerator {
    fn generateInMain(expr: Expr) -> String {
        String::from("main")
    }

    pub fn generate(expr: Expr) -> String {
        LLVMIRGenerator::generateInMain(expr)
    }
}
