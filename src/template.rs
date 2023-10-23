//! Шаблон для нового функционального блока

use crate::function_block::{FunctionBlockBase, IFunctionBlock};

#[derive(Clone, Default)]
pub struct VarInput {}

#[derive(Clone, Default)]
pub struct VarOutput {}

#[derive(Clone, Default)]
pub struct VarStatic {}

impl IFunctionBlock<VarInput, VarOutput, VarStatic>
    for FunctionBlockBase<VarInput, VarOutput, VarStatic>
{
    fn logic(&mut self) -> VarOutput {
        VarOutput {}
    }
}

#[allow(dead_code)]
pub type FunctionBlock = FunctionBlockBase<VarInput, VarOutput, VarStatic>;
