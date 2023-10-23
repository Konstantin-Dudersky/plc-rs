use serde::Serialize;

use crate::function_block::{FunctionBlockBase, IFunctionBlock};
use crate::{library::edge_detect::rising_edge, types};

#[derive(Clone, Default, Serialize)]
pub struct VarInput {
    pub input: bool,
    pub preset_time: types::TimeDuration,
}

#[derive(Clone, Default, Serialize)]
pub struct VarOutput {
    pub output: bool,
    pub elapsed_time: types::TimeDuration,
}

#[derive(Clone, Default, Serialize)]
pub struct VarStatic {
    input_rising_edge: rising_edge::FunctionBlock,
    delay: types::TimeInstant,
}

impl IFunctionBlock<VarInput, VarOutput, VarStatic>
    for FunctionBlockBase<VarInput, VarOutput, VarStatic>
{
    fn logic(&mut self) -> VarOutput {
        if self
            .stat
            .input_rising_edge
            .call(rising_edge::VarInput {
                i: self.input.input,
            })
            .q
        {
            self.stat.delay = types::TimeInstant::now();
        }

        VarOutput {
            output: self.stat.delay.elapsed() >= self.input.preset_time,
            elapsed_time: self.stat.delay.elapsed(),
        }
    }
}

pub type FunctionBlock = FunctionBlockBase<VarInput, VarOutput, VarStatic>;
