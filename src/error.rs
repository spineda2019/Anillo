pub struct CompilationError {
    line: u32,
    column: u32,
    diagnostic: String,
    src_available: bool,
}

impl CompilationError {
    pub fn new(line: u32, column: u32, diagnostic: String) -> CompilationError {
        CompilationError {
            line,
            column,
            diagnostic,
            src_available: true,
        }
    }

    pub fn new_without_src_info(diagnostic: String) -> CompilationError {
        CompilationError {
            line: 0,
            column: 0,
            diagnostic,
            src_available: false,
        }
    }
}

impl std::fmt::Debug for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CompilationError: {{line = {}, column = {}, diagnostic = {}}}{}",
            self.line,
            self.column,
            self.diagnostic,
            if self.src_available {
                ""
            } else {
                "\n(Source info unavaliable)"
            }
        )
    }
}

impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CompilationError: {{line = {}, column = {}, diagnostic = {}}}",
            self.line, self.column, self.diagnostic
        )
    }
}

impl std::error::Error for CompilationError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        todo!()
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }

    fn description(&self) -> &str {
        todo!()
    }
}
