use snafu::{Snafu, ResultExt, Backtrace, ErrorCompat, ensure};
use std::result;

#[derive(Debug, Snafu)]
pub enum SeparateError {
    #[snafu_display("General Error: {}", "mesg")]
    CatchAllError { mesg: String },
}

pub type SeparateResult<T, E = SeparateError> = result::Result<T, E>;
