//! Main entry point for Portal

#![deny(warnings, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use portal::application::APPLICATION;

/// Boot Portal
fn main() {
    abscissa_core::boot(&APPLICATION);
}
