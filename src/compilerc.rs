use crate::{compilerc::ia32::IA32, error::CompilationError, token::Ast};

mod ia32;

#[derive(Debug)]
pub enum Target {
    IA32,
    X86_64,
    ARM
}

pub trait CompileC {
    fn compile(&self) -> Result<(), CompilationError>;
}

pub struct CompilerC<'a> {
    backend: Box<dyn CompileC + 'a>
}

impl <'a> CompilerC<'a> {
    pub fn new(filename_in: &String, ast: &'a Ast, target: Target) -> CompilerC<'a>{
        CompilerC {backend: 
        match target {
            Target::IA32 => Box::from(IA32::new(&filename_in, ast)),
            _ => Box::from(NoBackend {backend_type: target}),
        }}
        
    }

    pub fn compile(&self) -> Result<(), CompilationError> {
        self.backend.compile()
    }
}

struct NoBackend {
    backend_type: Target
}

impl CompileC for NoBackend {
    fn compile(&self) -> Result<(), CompilationError>{
        Err(CompilationError::new_without_src_info(format!("C compiler not implemented for {:?} backend", self.backend_type)))
    }
}

