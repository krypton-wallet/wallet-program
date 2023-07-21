use crate::prelude::*;

pub trait GuardTrait {
    fn setup(&mut self, accounts: &[AccountInfo]) -> ProgramResult;
    fn run(&mut self, accounts: &[AccountInfo]) -> ProgramResult;
}
