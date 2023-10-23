//! Шаблон для нового функционального блока

use serde::{Deserialize, Serialize};

use crate::function_block::{FunctionBlockBase, IFunctionBlock};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct VarInput {}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct VarOutput {}

#[derive(Clone, Default, Deserialize, Serialize)]
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
