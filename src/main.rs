#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
mod spifi_impl;
mod uart_impl;


const TIMEOUT_VALUE: u32 = 1000000;


static mut TIMEOUT: u32 = 0;
static mut VALID_COMMANDS_TIMEOUT: u32 = 0;