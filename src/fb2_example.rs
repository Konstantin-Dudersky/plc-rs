pub use crate::function_block::FunctionBlockBase;
use crate::function_block::IFunctionBlock;

use super::fb1_example;

#[derive(Clone, Default)]
pub struct VarInput {
    pub counter: u32,
}

#[derive(Clone, Default)]
pub struct VarOutput {
    pub out_counter: u32,
}

#[derive(Clone, Default)]
pub struct VarStatic {
    pub internal_counter: u32,
    pub fb1_inst: fb1_example::Fb1Example,
}

impl IFunctionBlock<VarInput, VarOutput, VarStatic>
    for FunctionBlockBase<VarInput, VarOutput, VarStatic>
{
    fn logic(&mut self) -> VarOutput {
        println!("in fb2");
        let mut internal_counter = self.stat.internal_counter;

        self.stat
            .fb1_inst
            .call(fb1_example::VarInput { counter: 1 });

        VarOutput {
            out_counter: self.input.counter * 2,
        }
    }
}
