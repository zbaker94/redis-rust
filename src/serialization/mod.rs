pub mod deserialize;
pub mod serialize;

pub const VALID_START_CHARS: [char; 5] = ['+', '-', ':', '$', '*'];
pub const AGG_TYPE_CHARS: [char; 7] = ['$', '*', '!', '=', '%', '~', '>'];