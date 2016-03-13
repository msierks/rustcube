
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

    pub fn update_cr0(&mut self, r: u32) {
        if r == 0 {
            self.value[0] = 2; // EQ
        } else if (r & 0x80000000) != 0 {
            self.value[0] = 8; // LT
        } else {
            self.value[0] = 4; // GT
        }

        // FixMe: copy XER[SO] to forth bit
    }

}
