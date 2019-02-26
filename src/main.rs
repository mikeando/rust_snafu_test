
extern crate snafu;
mod error;
use snafu::{Snafu, ResultExt, Backtrace, ErrorCompat, ensure};

use std::result;

#[derive(Debug, Snafu)]
pub enum LocalError {
    #[snafu_display("General Error: {}", "mesg")]
    CatchAllError { mesg: String },
}

pub type LocalResult<T, E = LocalError> = result::Result<T, E>;

pub fn with_io_error() -> Result<(), std::io::Error> {
    Ok(())
}

pub fn create_project(name: &str) -> LocalResult<()>
{
    with_io_error().context(CatchAllError{mesg:"Eeek an error!"})?;
    Ok(())
}

pub fn create_project_separate(name: &str) -> error::SeparateResult<()>
{
    with_io_error().context(error::CatchAllError{mesg:"Eeek an error!"})?;
    Ok(())
}


fn main() {
    println!("Hello, world!");
}
