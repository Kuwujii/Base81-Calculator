pub const APP_NAME: &str = "Base81 Calculator";

pub enum Base {
    Null = 0,
    Binary = 2,
    Ternary = 3,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
    Unoctogesimal = 81
}

pub enum Sign {
    N = -1,
    P = 1
}

pub struct Num {
    udec_value: u64,
    sign: Sign,
    base: Base
}

impl Default for Num { //The default is set to -0 in Base0 so it's easy to recognise that nothing is stored and it should not be displayed or used in calculations
    fn default() -> Self {
        Self {
            udec_value: 0,
            sign: Sign::N,
            base: Base::Null
        }
    }
}