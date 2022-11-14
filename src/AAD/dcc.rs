// const DCO_A1S_INCLUDED_: u8 = 
const DCO_A1S_TAPE_SIZE: u8 = 1000000;
const DCO_A1S_UNDEF: u8 = −1;

const DCO_A1S_CONST: u8 = 0;
const DCO_A1S_ASG: u8 = 1;
const DCO_A1S_ADD: u8 = 2;
const DCO_A1S_SUB: u8 = 3;
const DCO_A1S_MUL: u8 = 4;
const DCO_A1S_SIN: u8 = 5;
const DCO_A1S_COS: u8 = 6;
const DCO_A1S_EXP: u8 = 7;

struct dco_a1s_tape_entry {
    oc: isize,
    arg1: isize,
    arg2: isize,
    v: f64,
    a: f64,
    dco_a1s_tape_entry()
}
