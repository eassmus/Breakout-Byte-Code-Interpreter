use crate::common::OpCode;

#[repr(C)]
pub union ChunkData {
    opcode: OpCode,
    data: u8,
}

pub struct Chunk {
    data: Vec<ChunkData>,
    pointer: usize,
}

impl Chunk {
    pub fn new(data: Vec<ChunkData>) -> Chunk {
        Chunk { data, pointer: 0 }
    }
    pub fn add_data(&mut self, data: &mut Vec<ChunkData>) {
        self.data.append(data);
    }
    #[inline]
    pub fn set_pointer(&mut self, pointer: usize) {
        self.pointer = pointer;
    }
    #[inline]
    pub fn get_instruction(&mut self) -> (OpCode, &[u8]) {
        let oc: OpCode = unsafe { self.data[self.pointer].opcode };
        match oc {
            OpCode::Return => {
                const DATA_SIZE: usize = 0;
                let byte_slice = unsafe {
                    let ptr = self.data[self.pointer].data as *const u8;
                    std::slice::from_raw_parts(ptr as *const u8, 1 + DATA_SIZE)
                };
                self.pointer += 1 + DATA_SIZE;
                (oc, byte_slice)
            }
            _ => todo!(),
        }
    }
}
