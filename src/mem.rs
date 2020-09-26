use crate::helpers::readFileIntoVec;

pub struct Memory {
    // main. non-IO memory
    pub ROM: Vec<u8>,
    pub ROMMask: usize // Mask to handle ROM read mirroring
}

impl Memory {
    pub fn new(romPath: String) -> Memory {
        let ROM = readFileIntoVec(&romPath);
        let ROMMask = ROM.capacity() - 1;
        
        Memory {
            ROM,
            ROMMask
        }
    }
}