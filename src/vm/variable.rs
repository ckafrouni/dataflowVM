#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Variable(String);

// make a counter to generate unique variable names for us
static mut COUNTER: u32 = 0;

impl Variable {
    pub fn new() -> Variable {
        unsafe {
            COUNTER += 1;
            Variable(format!("v{}", COUNTER))
        }
    }
}
