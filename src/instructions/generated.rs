// THIS FILE IS AUTOGENERATED, DO NOT EDIT
// You can regenerate it using `make generate` after creating a new instruction file with `python3 generate.py <mneumonic>`

use super::{axecutor::Axecutor, errors::AxError};
use iced_x86::{
    Instruction,
    Mnemonic::{self, *},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

impl Axecutor {
    pub fn switch_instruction_mnemonic(&mut self, i: Instruction) -> Result<(), AxError> {
        match i.mnemonic() {
            Add => self.mnemonic_add(i),
            Cmp => self.mnemonic_cmp(i),
            Ja => self.mnemonic_ja(i),
            Jae => self.mnemonic_jae(i),
            Jb => self.mnemonic_jb(i),
            Jbe => self.mnemonic_jbe(i),
            Je => self.mnemonic_je(i),
            Jecxz => self.mnemonic_jecxz(i),
            Jmp => self.mnemonic_jmp(i),
            Jrcxz => self.mnemonic_jrcxz(i),
            Lea => self.mnemonic_lea(i),
            Mov => self.mnemonic_mov(i),
            Nop => self.mnemonic_nop(i),
            Pop => self.mnemonic_pop(i),
            Push => self.mnemonic_push(i),
            Shl => self.mnemonic_shl(i),
            Sub => self.mnemonic_sub(i),
            Test => self.mnemonic_test(i),
            Xor => self.mnemonic_xor(i),
            _ => Err(AxError::from(format!(
                "unimplemented mnemonic {:?}",
                i.mnemonic()
            ))),
        }
    }
}

#[wasm_bindgen(js_name = Mnemonic)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportedMnemonic {
    Add = 7,
    Cmp = 93,
    Ja = 297,
    Jae = 298,
    Jb = 299,
    Jbe = 300,
    Je = 302,
    Jecxz = 303,
    Jmp = 308,
    Jrcxz = 316,
    Lea = 374,
    Mov = 414,
    Nop = 465,
    Pop = 590,
    Push = 640,
    Shl = 712,
    Sub = 740,
    Test = 751,
    Xor = 1518,
}

impl SupportedMnemonic {
    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

impl From<Mnemonic> for SupportedMnemonic {
    fn from(mnemonic: Mnemonic) -> Self {
        match mnemonic {
            Add => SupportedMnemonic::Add,
            Cmp => SupportedMnemonic::Cmp,
            Ja => SupportedMnemonic::Ja,
            Jae => SupportedMnemonic::Jae,
            Jb => SupportedMnemonic::Jb,
            Jbe => SupportedMnemonic::Jbe,
            Je => SupportedMnemonic::Je,
            Jecxz => SupportedMnemonic::Jecxz,
            Jmp => SupportedMnemonic::Jmp,
            Jrcxz => SupportedMnemonic::Jrcxz,
            Lea => SupportedMnemonic::Lea,
            Mov => SupportedMnemonic::Mov,
            Nop => SupportedMnemonic::Nop,
            Pop => SupportedMnemonic::Pop,
            Push => SupportedMnemonic::Push,
            Shl => SupportedMnemonic::Shl,
            Sub => SupportedMnemonic::Sub,
            Test => SupportedMnemonic::Test,
            Xor => SupportedMnemonic::Xor,
            _ => panic!("unimplemented mnemonic {:?}", mnemonic),
        }
    }
}
