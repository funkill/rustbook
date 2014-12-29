// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Common API for all rust-book subcommands.

use error::CliResult;
use term::Term;

use help;
use build;
use serve;
use test;

pub trait Subcommand {
    /// Mutate the subcommand by parsing its arguments.
    ///
    /// Returns `Err` on a parsing error.
    fn parse_args(&mut self, args: &[String]) -> CliResult<()>;
    /// Print the CLI usage information.
    fn usage(&self);
    /// Actually execute the subcommand.
    fn execute(&mut self, term: &mut Term);
}

/// Create a Subcommand object based on its name.
pub fn parse_name(name: &str) -> Option<Box<Subcommand>> {
    for parser in [
        help::parse_cmd as fn(&str) -> Option<Box<Subcommand>>,
        build::parse_cmd as fn(&str) -> Option<Box<Subcommand>>,
        serve::parse_cmd as fn(&str) -> Option<Box<Subcommand>>,
        test::parse_cmd as fn(&str) -> Option<Box<Subcommand>>].iter() {
        let parsed = (*parser)(name);
        if parsed.is_some() { return parsed }
    }
    None
}
