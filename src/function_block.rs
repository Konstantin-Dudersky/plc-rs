use serde::Serialize;

#[derive(Clone, Default, Serialize)]
pub struct FunctionBlockBase<TInput, TOutput, TStatic>
where
    TInput: Clone + Default + Serialize,
    TOutput: Clone + Default + Serialize,
    TStatic: Clone + Default + Serialize,
{
    pub input: TInput,
    pub output: TOutput,
    pub stat: TStatic,
}

impl<TInput, TOutput, TStatic> FunctionBlockBase<TInput, TOutput, TStatic>
where
    TInput: Clone + Default + Serialize,
    TOutput: Clone + Default + Serialize,
    TStatic: Clone + Default + Serialize,
    Self: IFunctionBlock<TInput, TOutput, TStatic>,
{
    pub fn call(&mut self, input: TInput) -> TOutput {
        self.input = input;
        self.output = self.logic();
        self.output.clone()
    }
}

pub trait IFunctionBlock<TInput, TOutput, TStatic> {
    /// Основная логика выполнения блока
    ///
    /// Нужно переопределить для своего функционального блока.
    /// Вызывать самому не нужно, вызывается функцией `call`
    fn logic(&mut self) -> TOutput;
}
