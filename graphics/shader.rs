#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Compute,
}

pub struct Shader {
    pub shader_type: ShaderType,
    pub source: String,
    pub compiled: bool,
}

impl Shader {
    pub fn new(
        shader_type: ShaderType,
        source: String,
    ) -> Self {
        Self {
            shader_type,
            source,
            compiled: false,
        }
    }

    pub fn compile(&mut self) -> Result<(), String> {
        if self.source.trim().is_empty() {
            return Err("Shader vazio.".to_string());
        }

        self.compiled = true;
        Ok(())
    }

    pub fn is_compiled(&self) -> bool {
        self.compiled
    }
}
