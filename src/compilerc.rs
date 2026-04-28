use std::{fs::File, io::Write};

use crate::token::{Ast, ExternalFunctionNode, FuncArgType, Ingot};

pub struct CompilerC<'a> {
    filename: String,
    ast: &'a Ast
}


impl CompilerC<'_> {

    pub fn new(filename_in: String, ast: &'_ Ast) -> CompilerC<'_> {
        CompilerC {filename: filename_in, ast: ast}
    }
 
    pub fn compile(&mut self) {
        let mut c_file = File::create(self.filename.clone() + ".c").expect("Failed to create file");
        let mut h_file = File::create(self.filename.clone() + ".h").expect("Failed to create file");
        write!(h_file, "void AnilloISRRegister();").expect("Failed to write to header file");

        let c_output = self.gen_cfile();

        write!(c_file, "{}", c_output).expect("Failed to write to C file");
    }

    fn gen_extern_funcs(&self) -> String {
        let mut out_string = String::new();
        for val in self.ast.asVec() {
            if let Ingot::ExternalFunction(ef_node) = val {
                out_string += &EXTERN_FUNC_TEMPLATE
                    .replace("{func_name}", &ef_node.name)
                    .replace("{func_args}", &self.gen_func_args(ef_node));
            }
        }
        out_string
    }

    fn gen_func_args(&self, ef_node: &ExternalFunctionNode ) -> String {
        let mut out_string = String::new();
        let mut first = true;
        for arg in &ef_node.args {
            if !first {
                out_string += ", ";
            } else {
                first = false;
            }
            out_string += &format!("{} {}", 
                match arg.type_t {
                    FuncArgType::U8 => "u8",
                    FuncArgType::U16 => "u16",
                    FuncArgType::U32 => "u32",
                    FuncArgType::U64 => "u64",
                    FuncArgType::I8 => "i8",
                    FuncArgType::I16 => "i16",
                    FuncArgType::I32 => "i32",
                    FuncArgType::I64 => "i64",
                }, arg._name);
        }
        out_string
    }

    fn gen_isr_funcs(&self) -> String {
        let out_string = String::new();
        for val in self.ast.asVec() {
            if let Ingot::Isr(isr_node) = val {
                
            }
        }
        out_string
    }

    fn gen_gate_descriptors(&self) -> String {
        let out_string = String::new();
        for val in self.ast.asVec() {
            if let Ingot::Isr(isr_node) = val {
                
            }
        }
        out_string
    }

    fn gen_cfile(&self) -> String {
        include_str!("c_out.tmpl")
            .replace("{filename}", &self.filename)
            .replace("{external_funcs}", &self.gen_extern_funcs())
            .replace("{isr_funcs}", &&self.gen_isr_funcs())
            .replace("{gate_descriptors}", &&self.gen_gate_descriptors())
    }

}

const EXTERN_FUNC_TEMPLATE: &str = "extern void {func_name}({func_args});\n";

const ISR_FUNC_TEMPLATE: &str = "\
void AnilloISR{isr_id}() {
    {body}
}";

const GATE_DESCRIPTOR_TEMPLATE: &str = "\
idt[{isr_id}] = (struct AnilloGateDescriptor) {
    .addr_l = ((u32) AnilloISR{isr_id} & 0xF),
    .segment = 0x8,
    .attributes = 0b1{privilege_bits}01110,
    .addr_h = ((u32) AnilloISR{isr_id} >> 16)
};";