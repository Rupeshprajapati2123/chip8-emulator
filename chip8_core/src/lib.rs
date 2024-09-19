pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const NUM_REGS: usize = 16;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

pub struct Emu{
    pc: u16,        // prgoram counter
    ram: [u16; RAM_SIZE],    // ram 
    screen: [bool; SCREEN_HEIGHT*SCREEN_WIDTH],     // screen
    v_reg: [u8; NUM_REGS],          
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Emu {
    pub fn new() ->Self{
        Self{
            pc:START_ADDR,
            ram:[0; RAM_SIZE],
            screen: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            v_reg: [0;NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] =val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp-=1;
        self.stack[self.sp  as usize]
    }
}