use std::default;

use derive_more::derive::{Add, Sub};


#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Add, Sub)]
#[allow(non_camel_case_types)]
enum Container {
    None(),
    u8(u8),
    i8(i8),
    u16(u16),
    i16(i16),
    u32(u32),
    i32(i32),
    u64(u64),
    i64(i64),
    u128(u128),
    i128(i128),
    f32(f32),
    f64(f64)
}

impl Default for Container {
    fn default() -> Self {
        Self::None()
    }
}

type Reg = usize;
type Add = usize;

#[derive(Default)]
pub struct VirtualMachine {
    registers: [Container; 8],
    ip: Add,
    program: Vec<u8>,
    stack: Vec<Container>,
    data: Vec<Container>,
    executing: bool,
    eq_flat: bool,
    gt_flat: bool,
}

impl VirtualMachine {
    fn new(program: Vec<u8>, heap_size: usize) -> Self {
        Self {
            registers: [Container::u8(0); 8],
            data: vec![Container::u8(0); heap_size],
            program,
            ..Self::default()
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(),                         // nop
    Mov(Reg, Container),           // mov immediate to reg
    Movr(Reg, Reg),                // mov reg contents to another reg
    Jmp(Reg),                      // program jump position
    Je(Reg),                       // jump if location is eq
    Jne(Reg),                      // jump if location is not eq
    Jg(Reg),                       // jump if greater than
    Jl(Reg),                       // jump if less than
    Cmp(Reg, Reg),                 // compares two registers
    Printr(Reg),                   // print contents of register
    Printv(Add),                   // print contents of immediate at address
    Vstore(Add, Container),        // store immediate into VMHeap at specific address from stack
    Vload(Add),                    // limmediate from VMHeap and pushes value to stack
    Vstorer(Add, Reg),             // store immediate into VMHeap from register contents
    Vloadr(Reg, Add),              // loads a immediate from VMHeap to register
    Add(Reg, Reg),                 // Add 2 registers and pushes result on stack
    Sub(Reg, Reg),                 // Subtract 2 registers and pushes result on stack
    Mul(Reg, Reg),                 // Multiple 2 registers and pushes result on stack
    Div(Reg, Reg),                 // Divide 2 registers and pushes result on stack
    And(Reg, Reg),                 // Bitwise AND on 2 registers. pushes result on stack
    Or(Reg, Reg),                  // Bitwise OR on 2 registers. pushes result on stack
    Xor(Reg, Reg),                 // Bitwise XOR on 2 registers. pushes result on stack
    Shr(Reg, Container),           // Shifts register to the right by (immediate)
    Shl(Reg, Container),           // Shifts register to the left by (immediate)
    Vpush(Container),              // Push immediate on to the stack
    Vpushr(Reg),                   // Push register contents on the stack
    Vpop(Reg),                     // pops immediate from stack to register
    Call(Reg),                     // call functon at address in register
    Ret(),                         // return from routine
    Abort(),                       // quit VM
}


fn main() {
    println!("Hello, world!");
}
