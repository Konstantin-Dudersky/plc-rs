use std::{fs::File, io::Write};

use serde::Serialize;
use serde_json::ser::to_string as serialize;

use crate::fb1_example;

#[derive(Clone, Default, Serialize)]
pub struct VarInput {}

#[derive(Clone, Default, Serialize)]
pub struct VarOutput {}

#[derive(Clone, Default, Serialize)]
pub struct VarStatic {
    pub fb1_inst: fb1_example::FunctionBlock,
    pub counter: u32,
}

#[derive(Default, Serialize)]
pub struct MainCycle<TInput, TOutput, TStatic>
where
    TInput: Serialize,
    TStatic: Serialize,
    TStatic: Serialize,
{
    input: TInput,
    output: TOutput,
    stat: TStatic,
}

impl<TInput, TOutput, TStatic> MainCycle<TInput, TOutput, TStatic>
where
    TInput: Serialize,
    TOutput: Serialize,
    TStatic: Serialize,
    Self: IOB,
{
    pub fn run(&mut self) {
        self.run_internal();

        let ser = serialize(&self).unwrap();
        let mut file = File::create("static.json").unwrap();
        file.write_all(ser.as_bytes()).unwrap();

        println!("main cycle");
    }
}

trait IOB {
    fn run_internal(&mut self);
}

impl IOB for MainCycle<VarInput, VarOutput, VarStatic> {
    fn run_internal(&mut self) {
        self.stat.counter += 1;
    }
}

pub type MainCycleType = MainCycle<VarInput, VarOutput, VarStatic>;
