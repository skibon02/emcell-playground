#![no_std]
#![no_main]

use emcell_macro::{define_header, extern_header};
use cells_defs::{Cell1, Cell2};

extern crate panic_halt;
extern crate at32f4xx_pac;

define_header!{
    Cell2 {
        b: 23,
        run_some_code,
        access_static
    }
}

extern_header!(Cell1Wrapper: Cell1);

pub fn run_some_code() {
    if let Some(cell1) = Cell1Wrapper::new() {
        (cell1.print_some_value)(cell1.a)
    }
}

pub const FLASH_UNLOCK_KEY1: u32 = 0x4567_0123;
pub fn access_static() -> u32 {
    FLASH_UNLOCK_KEY1
}