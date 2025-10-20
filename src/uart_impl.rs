use mik32v2_pac::{
    PadConfig, Pm, Usart0, usart_0
};
use ufmt::uWrite;
use ufmt::uwriteln;
use crate::{TIMEOUT, TIMEOUT_VALUE};

pub struct UartHandle;

enum UartHandleError {
    TIMOUT,
}



impl UartHandle {
    pub unsafe fn uart_init() {
        let pm = Pm::steal();
        let pad_config = PadConfig::steal();
        let uart0 = Usart0::steal();

        pm.clk_apb_p_set().modify(|_, w| {
            w.uart_0().enable()
        });

        pad_config.pad0_cfg().modify(|_, w| {
            w.port_0_5().func2_interface();
            w.port_0_6().func2_interface();
            w
        });

        pad_config.pad0_pupd().modify(|_, w| {
            w.port_0_5().pull_up()
        });

        uart0.control1().write(|w| {w.bits(0)});
        uart0.control2().write(|w| {w.bits(0)});
        uart0.control3().write(|w| {w.bits(0)});
        uart0.flags().write(|w| w.bits(0xFFFFFFFF));
        uart0.divider().write(|w| w.brr().bits(138));

        uart0.control1().modify(|_, w| {
            w.re().enable().te().enable().ue().enable()
        });

        while (!(uart0.flags().read().reack().is_ready() && uart0.flags().read().teack().is_ready())) {}
    }

    pub unsafe fn write_byte(byte: u16, uart: &mut Usart0) {
        uart.txdata().write(|w| w.tdr().bits(byte));
        while (!(uart.flags().read().tc().is_1())) {}
    }

    pub unsafe fn read_byte(uart: &Usart0) -> Result<u16, UartHandleError> {
        let timeout = 0;
        while !(uart.flags().read().rxne().is_1()) && timeout != crate::TIMEOUT_VALUE {
            crate::TIMEOUT += 1;
            crate::VALID_COMMANDS_TIMEOUT += 1;
        }

        if crate::TIMEOUT == TIMEOUT_VALUE {
            return Err(UartHandleError::TIMOUT);
        }

        Ok(uart.rxdata().read().bits() as u16)
    }


    pub fn write_buffer(buf: &[u8]) {
        unsafe {
            let mut uart = Usart0::steal();
            for &byte in buf {
                UartHandle::write_byte(byte as u16, &mut uart);
            }
        }
    }

    pub fn read_buffer(buf: &mut [u8]) -> Result<(), UartHandleError> {
        unsafe {
            let uart = Usart0::steal();
            for i in 0..buf.len() {
                buf[i] = UartHandle::read_byte(&uart)? as u8;
            };
            Ok(())
        }
    }
}

impl uWrite for UartHandle {
    type Error = ();
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        UartHandle::write_buffer(&[c as u8]);
        Ok(())
    }
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        UartHandle::write_buffer(s.as_bytes());
        Ok(())
    }
}

macro_rules! uprintln {
    () => {
        uwriteln!(crate::uart_impl::UartHandle, "").ok()
    };

    ($fmt:literal) => {
        uwriteln!(crate::uart_impl::UartHandle, $fmt).ok()
    };

    ($fmt:literal, $($arg:tt)*) => {
        uwriteln!(crate::uart_impl::UartHandle, $fmt, $($arg)*).ok()
    };
}