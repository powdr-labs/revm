use super::i256::{i256_cmp, i256_sign_compl, two_compl, Sign};
use crate::{
    gas,
    primitives::SpecId::CONSTANTINOPLE,
    primitives::{Spec, U256},
    Host, InstructionResult, Interpreter,
};
use core::cmp::Ordering;
use core::ops::{BitAnd, BitOr, BitXor};

pub fn lt(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = if op1.lt(op2) {
        U256::from(1)
    } else {
        U256::ZERO
    };
}

pub fn gt(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = if op1.gt(op2) {
        U256::from(1)
    } else {
        U256::ZERO
    };
}

pub fn slt(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = U256::from(i256_cmp(&op1, op2) == Ordering::Less);
}

pub fn sgt(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = U256::from(i256_cmp(&op1, op2) == Ordering::Greater);
}

pub fn eq(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = if op1.eq(op2) {
        U256::from(1)
    } else {
        U256::ZERO
    };
}

pub fn iszero(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1);
    *op1 = if *op1 == U256::ZERO {
        U256::from(1)
    } else {
        U256::ZERO
    };
}
pub fn bitand(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = op1.bitand(*op2);
}
pub fn bitor(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = op1.bitor(*op2);
}
pub fn bitxor(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = op1.bitxor(*op2);
}

pub fn not(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1);
    *op1 = !*op1;
}

pub fn byte(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    let mut ret = U256::ZERO;

    let o1 = as_usize_saturated!(op1);
    if o1 < 32 {
        let o2 = &*op2;
        ret = (o2 << (8 * o1)) >> (8 * 31);
    }

    *op2 = ret;
}

pub fn shl<SPEC: Spec>(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    // EIP-145: Bitwise shifting instructions in EVM
    check!(interpreter, SPEC::enabled(CONSTANTINOPLE));
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 <<= as_usize_saturated!(op1);
}

pub fn shr<SPEC: Spec>(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    // EIP-145: Bitwise shifting instructions in EVM
    check!(interpreter, SPEC::enabled(CONSTANTINOPLE));
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 >>= as_usize_saturated!(op1);
}

pub fn sar<SPEC: Spec>(interpreter: &mut Interpreter, _host: &mut dyn Host) {
    // EIP-145: Bitwise shifting instructions in EVM
    check!(interpreter, SPEC::enabled(CONSTANTINOPLE));
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);

    let value_sign = i256_sign_compl(op2);

    *op2 = if *op2 == U256::ZERO || op1 >= U256::from(256) {
        match value_sign {
            // value is 0 or >=1, pushing 0
            Sign::Plus | Sign::Zero => U256::ZERO,
            // value is <0, pushing -1
            Sign::Minus => U256::MAX,
        }
    } else {
        const ONE: U256 = U256::from_limbs([1, 0, 0, 0]);
        let shift = usize::try_from(op1).unwrap();
        match value_sign {
            Sign::Plus | Sign::Zero => op2.wrapping_shr(shift),
            Sign::Minus => two_compl(op2.wrapping_sub(ONE).wrapping_shr(shift).wrapping_add(ONE)),
        }
    };
}
