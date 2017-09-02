use syntax::ast::Expr;

pub struct LLVMIRGenerator;

impl LLVMIRGenerator {
    fn generateInMain(expr: Expr) -> String {
        format!("\
define i32 @main(i32, i8**) #0 {{
    ret i32 1
}}\
        ")
    }

    pub fn generate(expr: Expr) -> String {
        LLVMIRGenerator::generateInMain(expr)
    }
}
