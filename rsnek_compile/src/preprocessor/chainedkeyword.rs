use nom::Slice;
use slog;
use slog_scope;

use ::token::{Tk, Id, Tag};
use ::slice::TkSlice;
use ::preprocessor::Preprocessor;


const NOT_IN_BYTES: &'static [u8] = &[110, 111, 116, 32, 105, 110];
const IS_NOT_BYTES: &'static [u8] = &[105, 115, 32, 110, 111, 116];

const TK_NOT_IN: Tk = Tk::const_(
    Id::NotIn,
    NOT_IN_BYTES,
    Tag::Note(ChainedKeywordPreprocessor::NAME)
);

const TK_IS_NOT: Tk = Tk::const_(
    Id::NotIn,
    IS_NOT_BYTES,
    Tag::Note(ChainedKeywordPreprocessor::NAME)
);

const NOT_IN: &'static [Tk] = &[TK_NOT_IN];
const IS_NOT: &'static [Tk] = &[TK_IS_NOT];



/// Preprocessor to find occurrences of chained keywords ("not in", "is not", etc) and collapse them
/// into a single token. Not the best way to do it, but it does allow exploration of the
/// preprocessor idea.
///
#[derive(Debug, Clone, Serialize)]
pub struct ChainedKeywordPreprocessor {
    #[serde(skip_serializing)]
    log: slog::Logger
}


impl ChainedKeywordPreprocessor {
    pub const NAME: &'static str = "ChainedKeywordPreprocessor";

    // TODO: {114} This is a prototype of using a Logger for this struct which is in accordance
    // with best practices for slog, even though we are grabbing the root logger which is not.
    // Consider this an experimentation of the worth of using slog.
    pub fn new() -> Self {
        ChainedKeywordPreprocessor {
            log: slog_scope::logger().new(slog_o!())
        }
    }

    pub const fn name(&self) -> &str {
        ChainedKeywordPreprocessor::NAME
    }
}


impl<'a> Preprocessor<'a> for ChainedKeywordPreprocessor {
    type In = TkSlice<'a>;
    type Out = Box<[Tk<'a>]>;
    type Error = String;

    fn transform<'b>(&self, tokens: TkSlice<'b>) -> Result<Box<[Tk<'b>]>, String> {

        let mut acc: Vec<TkSlice<'b>> = Vec::new();

        let mut start: Option<usize>        = None;
        let mut last_tk: (usize, &Tk<'b>)    = (0, &Tk::default());

        for (idx, tk) in tokens.iter().enumerate() {

            let combined = match (last_tk.1.id(), tk.id()) {
                (_, Id::Space) => continue,
                (Id::Not, Id::In) => Some(NOT_IN),
                (Id::Is, Id::Not) => Some(IS_NOT),
                _ => None
            };

            if let Some(slice) = combined {
                if let Some(s) = start {
                    acc.push(tokens.slice(s..idx - 1))
                } else {
                    acc.push(tokens.slice(..idx - 1));
                }
                acc.push(TkSlice(&slice));

                start = Some(idx);
            }

            last_tk = (idx, &tk);
        }

        let collapsed_tokens = acc.iter()
            .flat_map(TkSlice::iter)
            .map(Tk::clone)
            .collect::<Vec<Tk<'b>>>();

        Ok(collapsed_tokens.into_boxed_slice())
    }

}