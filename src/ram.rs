#[derive(Debug)]
pub enum Timer {
    Delay,
    Sound,
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Ram {
    pub mem: [u8; 4096],
    pub PC: usize,
    pub V: [u8; 16],
    pub I: u16,
    pub stack: Vec<usize>,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Ram {
    const FONT: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    pub const START_PRGM_REGISTER: usize = 0x200;
    /// creates Ram struct
    pub fn load(program: &[u8]) -> Self {
        let memory = [0; 4096];
        let program_len = program.len();

        let loaded = memory[0..80]
            .iter()
            .chain(Self::FONT.iter())
            .chain(memory[(80 + Self::FONT.len())..Self::START_PRGM_REGISTER].iter())
            .chain(program.iter())
            .chain(memory[Self::START_PRGM_REGISTER + program_len..].iter())
            .copied()
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Game loading failed");

        Ram {
            mem: loaded,
            stack: vec![],
            V: [0; 16],
            I: 0,
            PC: Self::START_PRGM_REGISTER,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn get_next_instruction(&mut self) -> [u8; 2] {
        let hi_byte = self.mem[self.PC as usize];
        let lo_byte = self.mem[self.PC + 1_usize];

        // Move PC to next instruction. We consume instructions
        // two bytes at a time.
        self.PC += 2;
        [hi_byte, lo_byte]
    }

    /// sets VF register
    pub fn update_vf_register(&mut self, should_update: bool) {
        self.V[0xF] = if should_update { 1 } else { 0 };
    }

    /// sets Index register
    pub fn set_i_register(&mut self, value: u16) {
        self.I = value;
    }

    /// sets V register
    pub fn set_register(&mut self, idx: usize, value: u8) {
        self.V[idx] = value;
    }

    /// adds address to Ram.stack
    pub fn store_addr(&mut self, addr: usize) {
        self.stack.push(addr);
    }

    /// removes most recently added memory address from Ram.stack
    pub fn remove_addr(&mut self) -> Option<usize> {
        self.stack.pop()
    }

    /// sets sound or delay timer register
    pub fn set_timer_register(&mut self, which_timer: Timer, value: u8) {
        match which_timer {
            Timer::Delay => self.delay_timer = value,
            Timer::Sound => self.sound_timer = value,
        }
    }

    /// returns current value of sound or delay timer register
    pub fn get_timer_register(&mut self, which_timer: Timer) -> u8 {
        match which_timer {
            Timer::Delay => self.delay_timer,
            Timer::Sound => self.sound_timer,
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    const PROGRAM: &[u8] = &[1, 2, 3, 4];

    #[test]
    fn loads_font() {
        let ram = Ram::load(&PROGRAM);
        let end = 80 + Ram::FONT.len();
        assert_eq!(&ram.mem[80..end], Ram::FONT);
    }

    #[test]
    fn loads_program() {
        let ram = Ram::load(&PROGRAM);
        let end = ram.PC + PROGRAM.len();
        assert_eq!(&ram.mem[ram.PC..end], PROGRAM);
    }

    #[test]
    fn sets_registers() {
        let mut ram = Ram::load(&PROGRAM);
        ram.set_register(1, 10u8);
        assert_eq!(ram.V[1], 10u8)
    }

    #[test]
    fn adds_address() {
        let mut ram = Ram::load(&PROGRAM);
        ram.store_addr(0x0F);
        assert_eq!(ram.stack, vec![0x0F]);
    }

    #[test]
    fn removes_address() {
        let mut ram = Ram::load(&PROGRAM);
        ram.store_addr(0x0F);
        let removed = ram.remove_addr().unwrap();
        assert!(removed == 0x0F);
    }

    #[test]
    fn updates_vf_register() {
        let mut ram = Ram::load(&PROGRAM);
        ram.update_vf_register(true);
        assert!(ram.V[15] == 1);
        ram.update_vf_register(false);
        assert!(ram.V[15] == 0);
    }

    #[test]
    fn sets_delay_timer_register() {
        let mut ram = Ram::load(&PROGRAM);
        ram.set_timer_register(Timer::Delay, 255);
        assert_eq!(ram.delay_timer, 255);
    }

    #[test]
    fn gets_delay_timer_register() {
        let mut ram = Ram::load(&PROGRAM);
        ram.set_timer_register(Timer::Delay, 255);
        assert_eq!(255, ram.get_timer_register(Timer::Delay));
    }

    #[test]
    fn sets_sound_timer_register() {
        let mut ram = Ram::load(&PROGRAM);
        ram.set_timer_register(Timer::Sound, 255);
        assert_eq!(ram.sound_timer, 255);
    }

    #[test]
    fn gets_sound_timer_register() {
        let mut ram = Ram::load(&PROGRAM);
        ram.set_timer_register(Timer::Sound, 255);
        assert_eq!(255, ram.get_timer_register(Timer::Sound));
    }
}
