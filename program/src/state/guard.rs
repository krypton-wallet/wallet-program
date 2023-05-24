use crate::prelude::*;

pub trait Guard {
    fn setup(&mut self, accounts: &[AccountInfo]) -> ProgramResult;
    fn run(&mut self, accounts: &[AccountInfo]) -> ProgramResult;
}
