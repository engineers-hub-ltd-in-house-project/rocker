use crate::instruction::Instruction;
use serde::{Deserialize, Serialize};

/// Stage represents a stage in a Rockerfile (for multi-stage builds)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    /// Name of the stage
    pub name: Option<String>,
    /// Base image for the stage
    pub base_image: Option<String>,
    /// Instructions in the stage
    pub instructions: Vec<Instruction>,
}

impl Stage {
    /// Create a new stage
    pub fn new(name: Option<String>) -> Self {
        Stage {
            name,
            base_image: None,
            instructions: Vec::new(),
        }
    }

    /// Add an instruction to the stage
    pub fn add_instruction(&mut self, instruction: Instruction) {
        // If it's a FROM instruction, set the base image
        if let Instruction::From { image, .. } = &instruction {
            self.base_image = Some(image.clone());
        }

        self.instructions.push(instruction);
    }

    /// Check if the stage has the given name
    pub fn has_name(&self, name: &str) -> bool {
        self.name.as_ref().map_or(false, |n| n == name)
    }

    /// Get the name of the stage
    pub fn get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get the base image of the stage
    pub fn get_base_image(&self) -> Option<&str> {
        self.base_image.as_deref()
    }

    /// Get the FROM instruction of the stage
    pub fn get_from_instruction(&self) -> Option<&Instruction> {
        self.instructions.first().filter(|i| i.is_from())
    }

    /// Get all the instructions of the stage
    pub fn get_instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    /// Get a mutable reference to the instructions of the stage
    pub fn get_instructions_mut(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }
} 
