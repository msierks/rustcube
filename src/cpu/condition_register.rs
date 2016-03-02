
const NUM_CR : usize = 8;

#[derive(Default, Debug)]
pub struct ConditionRegister {
    value: [u8; NUM_CR]
}

impl ConditionRegister {

    pub fn get_field(&self) {

    }

    pub fn set_field(&mut self, index: usize, value: u8) {
        self.value[index] = value;
    }

    pub fn get_bit(&self, index: usize) -> u8 {
        (self.value[index / 4] >> (3 - (index % 4))) & 1 
    }

    pub fn set_bit(&mut self) {

    }

}
