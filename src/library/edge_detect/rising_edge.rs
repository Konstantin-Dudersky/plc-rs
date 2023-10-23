use serde::{Deserialize, Serialize};

pub use crate::function_block::FunctionBlockBase;
use crate::function_block::IFunctionBlock;

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct VarInput {
    pub i: bool,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct VarOutput {
    pub q: bool,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct VarStatic {
    prev_i: bool,
}

impl IFunctionBlock<VarInput, VarOutput, VarStatic>
    for FunctionBlockBase<VarInput, VarOutput, VarStatic>
{
    fn logic(&mut self) -> VarOutput {
        let rising_edge = self.input.i && !self.stat.prev_i;
        self.stat.prev_i = self.input.i;

        VarOutput { q: rising_edge }
    }
}

pub type FunctionBlock = FunctionBlockBase<VarInput, VarOutput, VarStatic>;
