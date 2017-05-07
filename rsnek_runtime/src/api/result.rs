use std::result;

use ::api::error::Error;
use ::api::RtObject;

pub type RtResult<T> = result::Result<T, Error>;
pub type ObjectResult = RtResult<RtObject>;
