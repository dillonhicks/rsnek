use nom::Slice;

use ::token::{Id, Tk, BLOCK_START, BLOCK_END, NEWLINE};
use ::slice::{TkSlice};
use ::preprocessor::Preprocessor;


/// Limit taken from CPython. They use a fixed size array to cap the indent stack.
const INDENT_STACK_SIZE: usize = 100;


// TODO: {T91} move to rsnek_runtime::macros
macro_rules! strings_error_indent_mismatch {
    ($len:expr, $indent:expr) => {
        format!("Indent len={} is not a multiple of first indent len={}", $len, $indent);
    }
}

// TODO: {T91} move to rsnek_runtime::macros
macro_rules! strings_error_unexpected_indent {
    ($len:expr, $indent:expr) => {
        format!("Unexpected indent len={}, expected indent len={}", $len, $indent);
    }
}


// TODO: {T91} move to rsnek_runtime::macros
macro_rules! strings_error_indent_overflow {
    ($max:expr) => {
        format!("Number of INDENT is more than the max allowed {}", $max);
    }
}


/// Preprocessor to splice BlockStart and BlockEnd tokens based on indent deltas
/// into the slice of tokens so we do not have to keep the state in the main parser
/// logic.
#[derive(Debug, Clone, Copy, Serialize, Default)]
pub struct BlockScopePreprocessor;


impl BlockScopePreprocessor {
    pub fn new() -> Self {
        BlockScopePreprocessor {}
    }

    /// Determine the length of the first indent as the longest consecutive span of
    /// space tokens after a newline before a non space is reached where the the length
    /// of the span must be greater than 0.
    fn determine_indent<'a>(&self, tokens: &TkSlice<'a>) -> usize {

        let mut start: Option<usize> = None;
        let mut last: (usize, Tk<'a>) = (0, Tk::default());

        for (idx, tk) in tokens.iter().enumerate() {
            match (last.1.id(), tk.id(), start) {
                // A space preceded by a newline starts the search for the indent
                (Id::Newline, Id::Space, None) => start = Some(idx),
                // A newline preceded by a space but in the middle of a span means
                // a line of all whitespace. Reset search.
                (Id::Space, Id::Newline, Some(_)) => {
                    start = None;
                },
                // Search started and we have encountered the first non ws char
                // so send back the len of the search span as the known indent size.
                (Id::Space, id, Some(s)) if id != Id::Space && id != Id::Newline => {
                    return tokens.slice(s..idx).len();
                },
                _ => {}
            };

            last = (idx, tk.clone());
        }

        return 0
    }

    /// Given the length of the current span of whitespace, the size of the first discovered
    /// indent, and the current index in the indent stack.
    #[inline]
    fn balance_scopes<'b>(&self, span_len: usize, indent: usize,
                          stack_idx_start: usize, indent_stack: &mut [usize],
                          acc: &mut Vec<TkSlice<'b>>) -> Result<usize, String> {

        if span_len % indent != 0 {
            return Err(strings_error_indent_mismatch!(span_len, indent));
        }

        let mut stack_idx = stack_idx_start;

        match indent_stack[stack_idx] {
            // The next indent span must be the current + known indent size, or it
            // is an error case.
            curr if span_len == curr + indent => {
                if INDENT_STACK_SIZE <= stack_idx + 1 {
                    return Err(strings_error_indent_overflow!(INDENT_STACK_SIZE))
                }

                stack_idx += 1;
                indent_stack[stack_idx] = curr + indent;
                info!("IndentStack";
                "action" => "emit",
                "token" => "BlockStart",
                "stack_idx" => stack_idx);
                acc.push(TkSlice(&BLOCK_START));
            },
            // The de-indent case where we are going from a nested scope N back to the
            // N-1 scope.
            curr if span_len < curr => {
                'emit_block_end: while stack_idx != 0 {
                    stack_idx -= 1;

                    info!("IndentStack";
                    "action" => "emit",
                    "token" => "BlockEnd",
                    "stack_idx" => stack_idx);
                    acc.push(TkSlice(&BLOCK_END));

                    if indent_stack[stack_idx] == span_len {
                        break 'emit_block_end;
                    }
                }
            },
            // Indent is the same as the current indent, so no changes
            curr if span_len == curr => {
                info!("IndentStack"; "action" => "noop", "stack_idx" => stack_idx);
                acc.push(TkSlice(&NEWLINE));
            }
            // Over indenting
            curr => {
                return Err(strings_error_unexpected_indent!(span_len, curr + indent));
            }
        }

        Ok(stack_idx)
    }
}


impl<'a> Preprocessor<'a> for BlockScopePreprocessor {
    type In = TkSlice<'a>;
    type Out = Box<[Tk<'a>]>;
    type Error = String;

    fn transform<'b>(&self, tokens: TkSlice<'b>) -> Result<Box<[Tk<'b>]>, String> {
        let indent = self.determine_indent(&tokens);

        if indent == 0 {
            return Ok(tokens.tokens().to_owned().into_boxed_slice());
        }

        let mut acc: Vec<TkSlice<'b>> = Vec::new();

        let mut stack_idx = 0;
        let mut indent_stack: [usize; INDENT_STACK_SIZE]  = [0; INDENT_STACK_SIZE];

        let mut start: Option<usize>        = None;
        let mut end: Option<usize>          = None;
        let mut is_continuation             = false;
        let mut seen_non_ws                 = false;

        let mut last_tk: (usize, Tk<'b>)    = (0, Tk::default());

        for (idx, tk) in tokens.iter().enumerate() {
            match (last_tk.1.id(), tk.id(), is_continuation, seen_non_ws, start) {
                // Just seen a newline on the last pass and it is not a \ escaped
                // continuation so start collapsing the whitespace
                (Id::Newline, Id::Space, false, false, None) => {
                    if let Some(e) = end{
                        acc.push(tokens.slice(e..idx - 1))
                    } else {
                        acc.push(tokens.slice(..idx - 1));
                    }

                    start = Some(idx);
                    end = None;
                },
                // Continue to consume spaces
                (Id::Space,   Id::Space, false, false, _) => {},
                // Found the first non ws char
                (Id::Space, id, false, false, Some(s)) if id != Id::Space => {
                    // TODO: {T92} Formalize preprocessing to allow for injection of expression start
                    // and end
                    let span = tokens.slice(s..idx);

                    seen_non_ws = true;
                    start = None;
                    end = Some(idx);

                    match self.balance_scopes(span.len(), indent, stack_idx,
                                              &mut indent_stack, &mut acc) {
                        Ok(new_stack_idx) => stack_idx = new_stack_idx,
                        Err(string) => return Err(string)
                    };

                    acc.push(span);
                },
                // A top level scope cannot close its scopes until it finds the next one
                (Id::Newline, id, false, false, _) if id != Id::Space && id != Id::Newline => {
                    // TODO: {T92} Formalize preprocessing to allow for injection of expression start
                    // and end
                    if let Some(e) = end{
                        acc.push(tokens.slice(e..idx - 1))
                    } else {
                        acc.push(tokens.slice(..idx - 1));
                    }

                    seen_non_ws = true;
                    start = None;
                    end = Some(idx);

                    match self.balance_scopes(0, indent, stack_idx,
                                              &mut indent_stack, &mut acc) {
                        Ok(new_stack_idx) => stack_idx = new_stack_idx,
                        Err(string) => return Err(string)
                    };

                },
                // Continuation start case.
                (_, Id::Backslash, _, _, _) => {
                    // TODO: {T92} Formalize preprocessing to allow for injection of expression start
                    // and end
                    // TODO: {T93} Handle backslash continuations... rewrite the
                    //   newline masked as a Id::Space??
                    is_continuation = true;
                },
                // TODO: {T92} Formalize preprocessing to allow for injection of expression start
                // and end
                // Newline after a Backslash Continuation, toggle the continuation to
                // false but keep seen_non_ws == true.
                (_, Id::Newline, true, _, _) => {
                    is_continuation = false;
                },
                // Normal newline
                (_, Id::Newline, false, _, _) => {
                    seen_non_ws = false;
                },
                _ => {}
            };

            last_tk = (idx, tk.clone());
        }

        // Put the rest of the tokens in the acc if the loops did not end on a
        // scope boundary.
        match (start, end) {
            (Some(i), _) |
            (_, Some(i)) => acc.push(tokens.slice(i..)),
            _ => unreachable!()
        };

        // There was not another block indented scope between the last
        // start and the end of the file. So emit the closing block ends until
        // all open block starts are balanced with block ends.
        'emit_trailing_end_scopes: while stack_idx != 0 {
            stack_idx -= 1;
            acc.push(TkSlice(&BLOCK_END));
        }

        let scoped_tokens = acc.iter()
            .flat_map(TkSlice::iter)
            .map(Tk::clone)
            .collect::<Vec<Tk<'b>>>();

        Ok(scoped_tokens.into_boxed_slice())
    }

}