/// Generic Preproccessor which just enforces a simple adpater/transformer
/// interface on the implementor.
pub trait Preprocessor<'a> {
    type In;
    type Out;
    type Error;

    fn transform(&self, input: Self::In) -> Result<Self::Out, Self::Error>;
}

