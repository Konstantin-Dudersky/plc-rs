#[derive(Clone, Default)]
pub struct FunctionBlockBase<TInput, TOutput, TStatic>
where
    TInput: Clone + Default,
    TOutput: Clone + Default,
    TStatic: Clone + Default,
{
    pub input: TInput,
    pub output: TOutput,
    pub stat: TStatic,
}

impl<TInput, TOutput, TStatic> FunctionBlockBase<TInput, TOutput, TStatic>
where
    TInput: Clone + Default,
    TOutput: Clone + Default,
    TStatic: Clone + Default,
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
