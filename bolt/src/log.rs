#[cfg(not(test))]
pub use log::{trace, debug, info, warn, error};

#[cfg(test)]
pub use std::{println as trace, println as debug, println as info, println as warn, eprintln as error};
