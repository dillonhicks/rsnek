use std::result;

use ::error::Error;
use ::object::RtObject;

pub type RtResult<T> = result::Result<T, Error>;
pub type ObjectResult = RtResult<RtObject>;
