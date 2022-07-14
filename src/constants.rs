pub const APP_NAME: &str = "Base81 Calculator";

pub enum Sign {
    N = -1,
    P = 1
}

pub struct Num {
    udec_value: u64,
    sign: Sign,
    base_min1: u8
}

impl Default for Num { //The default is set to -0 in Base0 so it's easy to recognise that nothing is stored and it should not be displayed or used in calculations
    fn default() -> Self {
        Self {
            udec_value: 0,
            sign: Sign::N,
            base_min1: 0
        }
    }
}