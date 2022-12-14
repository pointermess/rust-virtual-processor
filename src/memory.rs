use std::collections::HashMap;
use crate::types::{MemoryOperationSize, Register};


/**
 * ManageMemory Trait
 * 
 * Contains definitions to read and write memory in
 * either 1-, 2- or 4-byte chunks.
 * 
 * ToDo: Add 4 byte operations
 */
pub trait ManageMemory {
    // Read operations
    // will call either "read8" or  "read16" on size parameter
    fn read(&self, position : usize, size : MemoryOperationSize) -> i32;
    // reads 1 byte from the memory
    fn read8(&self, position : usize) -> i8;
    // reads 2 bytes from the memory
    fn read16(&self, position : usize) -> i16;

    // Write operations
    // will call either "write8" or  "write16" on size parameter
    fn write(&mut self, position : usize, value : i32, size : MemoryOperationSize);
    // reads 1 byte from the memory
    fn write8(&mut self, position : usize, value : i8);
    // reads 2 bytes from the memory
    fn write16(&mut self, position : usize, value : i16);
}

pub trait ManageRegisters {
    // returns the address location of the register in parameter
    fn get_register_address(&self, register: Register) -> usize;
    // returns the memory size of the register in parameter
    fn get_register_size(register: Register) -> MemoryOperationSize;
}

/**
 * ManageHeap Trait
 * 
 * Contains definition to manage the heap memory.
 * Allocates or de-allocates specific ranges of memory for
 * programs or data within those programs.
 */
pub trait ManageHeap {
    // Finds available heap location with enough space
    fn find_available_heap(size: u32) -> usize;

    // Allocates heap memory with a specified amount of
    // bytes at the specified position.
    fn allocate_heap(size: u32) -> usize;

    // Frees the heap at a specified position
    fn free_heap(position : usize);
}

pub struct Memory {
    // internal array / vector containing the complete memory
    raw_memory:  Vec<i8>,

    // hash map containing memory locations for each register
    register_lookup_table: HashMap<i32, i32>,
}

impl Memory {
    fn init_register_lookup_table(memory : &mut Memory) {
        // fill registers lookup table
        // ToDo: Find nicer solution?
        memory.register_lookup_table.insert(Register::AL as i32, 3);
        memory.register_lookup_table.insert(Register::BL as i32, 7);
        memory.register_lookup_table.insert(Register::CL as i32, 11);
        memory.register_lookup_table.insert(Register::DL as i32, 15);
        memory.register_lookup_table.insert(Register::AH as i32, 2);
        memory.register_lookup_table.insert(Register::BH as i32, 6);
        memory.register_lookup_table.insert(Register::CH as i32, 10);
        memory.register_lookup_table.insert(Register::DH as i32, 14);
        
        memory.register_lookup_table.insert(Register::AX as i32, 0);
        memory.register_lookup_table.insert(Register::BX as i32, 4);
        memory.register_lookup_table.insert(Register::CX as i32, 8);
        memory.register_lookup_table.insert(Register::DX as i32, 12);
        
        memory.register_lookup_table.insert(Register::EAX as i32, 0);
        memory.register_lookup_table.insert(Register::EBX as i32, 4);
        memory.register_lookup_table.insert(Register::ECX as i32, 8);
        memory.register_lookup_table.insert(Register::EDX as i32, 12);
        
        memory.register_lookup_table.insert(Register::ESP as i32, 16);
        memory.register_lookup_table.insert(Register::EBP as i32, 20);

        memory.register_lookup_table.insert(Register::Unknwown as i32, -1);
    }


    // initializes the internal state of the memory implementation
    fn init() -> Memory {
        let mut result = Memory {
            raw_memory: Vec::new(),
            register_lookup_table: HashMap::new()
        };

        Memory::init_register_lookup_table(&mut result);

        // init memory
        for _index in 0..4096  { 
            result.raw_memory.push(0);
        }

        return result;
    }

    // returns an initialized memory struct
    pub fn new() -> Memory {
        Memory::init()
    }
}

// ToDo: Check out of bounds when reading and writing
// ToDo: Implement 4 byte operations
impl ManageMemory for Memory {
    fn read(&self, position : usize, size : MemoryOperationSize) -> i32 {
        match size {
            MemoryOperationSize::Byte => return self.read8(position) as i32,
            MemoryOperationSize::Word => return self.read16(position) as i32,
        };
    }

    fn read8(&self, position : usize) -> i8 {
        return self.raw_memory[position];
    }

    fn read16(&self, position : usize) -> i16 {
        return (i16::from(self.raw_memory[position]) << 8) + i16::from(self.raw_memory[position + 1]);
    }

    fn write(&mut self, position : usize, value : i32, size : MemoryOperationSize) {
        match size {
            MemoryOperationSize::Byte => self.write8(position, value as i8),
            MemoryOperationSize::Word => self.write16(position, value as i16),
        };
    }

    fn write8(&mut self, position : usize, value : i8) {
        self.raw_memory[position] = value;
    }

    fn write16(&mut self, position : usize, value : i16) {
        self.raw_memory[position] = (value >> 8) as i8;
        self.raw_memory[position+1] = value as i8;
    }
} // impl ManageMemory for Memory


impl ManageRegisters for Memory {
    fn get_register_address(&self, register: Register) -> usize {
        return self.register_lookup_table[&(register as i32)] as usize; 
    }

    // ToDo: implement 4 byte registers
    fn get_register_size(register: Register) -> MemoryOperationSize {
        let val : i8 = register as i8;

        if val >= 0 && val <= 7 {
            return MemoryOperationSize::Byte;
        } else if val >= 8 && val <= 11 {
            return MemoryOperationSize::Word;
        }

        // if none of the above, return byte
        return MemoryOperationSize::Byte;
    }
}