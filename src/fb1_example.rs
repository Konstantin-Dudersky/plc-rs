use serde::Serialize;

use crate::{
    function_block::{FunctionBlockBase, IFunctionBlock},
    library::timer::ton,
    types,
};

#[derive(Clone, Default, Serialize)]
pub struct VarInput {
    pub counter: u32,
}

#[derive(Clone, Default, Serialize)]
pub struct VarOutput {
    pub out_counter: u32,
}

#[derive(Clone, Default, Serialize)]
pub struct VarStatic {
    timer: ton::FunctionBlock,
}

impl IFunctionBlock<VarInput, VarOutput, VarStatic>
    for FunctionBlockBase<VarInput, VarOutput, VarStatic>
{
    fn logic(&mut self) -> VarOutput {
        let ton_res = self.stat.timer.call(ton::VarInput {
            input: true,
            preset_time: types::TimeDuration::from_secs(10),
        });

        println!(
            "in fb1, timer: {}, elapsed: {:?}",
            ton_res.output, ton_res.elapsed_time
        );

        VarOutput {
            out_counter: self.input.counter,
        }
    }
}

pub type FunctionBlock = FunctionBlockBase<VarInput, VarOutput, VarStatic>;
