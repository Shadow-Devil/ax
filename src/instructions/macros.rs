#[macro_export]
macro_rules! calculate_rm_r {
    [u8; $self:expr; $i:expr; $op:expr] => {
        {
			use crate::instructions::operand::Operand;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_8(src.into());


            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_8($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.mem_write_8($self.mem_addr(m), result)
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_8(r);
                    let result = $op(dest_val, src_val);
                    $self.reg_write_8(r, result);
                    Ok(())
                }
                _ => panic!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u16; $self:expr; $i:expr; $op:expr] => {
        {
			use crate::instructions::operand::Operand;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_16(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_16($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.mem_write_16($self.mem_addr(m), result)
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_16(r);
                    let result = $op(dest_val, src_val);
                    $self.reg_write_16(r, result);
                    Ok(())
                }
                _ => panic!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u32; $self:expr; $i:expr; $op:expr] => {
        {
			use crate::instructions::operand::Operand;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_32(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_32($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.mem_write_32($self.mem_addr(m), result)
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_32(r);
                    let result = $op(dest_val, src_val);
                    $self.reg_write_32(r, result);
                    Ok(())
                }
                _ => panic!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
    [u64; $self:expr; $i:expr; $op:expr] => {
        {
			use crate::instructions::operand::Operand;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = $self.reg_read_64(src.into());

            match dest {
                Operand::Memory(m) => {
                    let dest_val = $self.mem_read_64($self.mem_addr(m))?;
                    let result = $op(dest_val, src_val);
                    $self.mem_write_64($self.mem_addr(m), result)
                }
                Operand::Register(r) => {
                    let dest_val = $self.reg_read_64(r);
                    let result = $op(dest_val, src_val);
                    $self.reg_write_64(r, result);
                    Ok(())
                }
                _ => panic!("Invalid destination operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            }
        }
    };
}

#[macro_export]
macro_rules! calculate_r_rm {
    [u8; $self:expr; $i:expr; $op:expr] => {
        {
			use crate::instructions::operand::Operand;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Memory(m) => {
                    $self.mem_read_8($self.mem_addr(m))?
                }
                Operand::Register(r) => {
                    $self.reg_read_8(r)
                }
                _ => panic!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            let dest = dest.into();
            let dest_val = $self.reg_read_8(dest);
            let result = $op(src_val, dest_val);
            $self.reg_write_8(dest, result);
            Ok(())
        }
    };
    [u16; $self:expr; $i:expr; $op:expr] => {
        {
			use crate::instructions::operand::Operand;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Memory(m) => {
                    $self.mem_read_16($self.mem_addr(m))?
                }
                Operand::Register(r) => {
                    $self.reg_read_16(r)
                }
                _ => panic!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            let dest = dest.into();
            let dest_val = $self.reg_read_16(dest);
            let result = $op(src_val, dest_val);
            $self.reg_write_16(dest, result);
            Ok(())
        }
    };
    [u32; $self:expr; $i:expr; $op:expr] => {
        {
			use crate::instructions::operand::Operand;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Memory(m) => {
                    $self.mem_read_32($self.mem_addr(m))?
                }
                Operand::Register(r) => {
                    $self.reg_read_32(r)
                }
                _ => panic!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            let dest = dest.into();
            let dest_val = $self.reg_read_32(dest);
            let result = $op(src_val, dest_val);
            $self.reg_write_32(dest, result);
            Ok(())
        }
    };
    [u64; $self:expr; $i:expr; $op:expr] => {
        {
			use crate::instructions::operand::Operand;

            let (dest, src) = $self.instruction_operands_2($i)?;
            let src_val = match src {
                Operand::Memory(m) => {
                    $self.mem_read_64($self.mem_addr(m))?
                }
                Operand::Register(r) => {
                    $self.reg_read_64(r)
                }
                _ => panic!("Invalid source operand {:?} for {:?} instruction", dest, $i.mnemonic()),
            };

            let dest = dest.into();
            let dest_val = $self.reg_read_64(dest);
            let result = $op(src_val, dest_val);
            $self.reg_write_64(dest, result);
            Ok(())
        }
    };
}
