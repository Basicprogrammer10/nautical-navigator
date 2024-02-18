//! ## References
//! - [ISO/IEC 8211](https://iho.int/uploads/user/Services%20and%20Standards/S-100WG/MISC/US_S100-Part10a.pdf)
//! - [iso8211 crate](https://crates.io/crates/iso8211) (unmaintained)

pub mod data_descriptive_record;
pub mod data_record;
pub mod error;
pub mod parser;

pub const FIELD_TERMINATOR: u8 = 0x1E;
pub const UNIT_TERMINATOR: u8 = 0x1F;
