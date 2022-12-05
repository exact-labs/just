pub mod compile;

pub trait CommandRunner {
    fn execute(&self);
}
