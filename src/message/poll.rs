use deku::prelude::*;

use super::markers::Command;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Poll {}

impl Command for Poll {}
