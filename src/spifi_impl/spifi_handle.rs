use mik32v2_pac::{
    PadConfig, Pm, SpifiConfig
};
use super::cmd::Command;

enum SpifiHandleError {
    TIMEOUT,
    INVALID_BUFFER_SIZE,
    ERROR,
}

struct SpifiHandle {
    _marker: core::marker::PhantomData<()>, 
}

impl SpifiHandle {
    #[inline(always)]
    unsafe fn spifi_msp_init() {
        let pm = Pm::steal();
        let pad_cfg = PadConfig::steal();
    
        pm.clk_ahb_set().modify(|_, w| {
            w.spifi().enable()
        });

        pad_cfg.pad2_cfg().modify(|_, w| {
            w.port_2_0().func2_interface();
            w.port_2_1().func2_interface();
            w.port_2_2().func2_interface();
            w.port_2_3().func2_interface();
            w.port_2_4().func2_interface();
            w.port_2_5().func2_interface();
            w
        });
        pad_cfg.pad2_pupd().modify(|_, w| {
            w.port_2_0().pull_none();
            w.port_2_1().pull_none();
            w.port_2_2().pull_up();
            w.port_2_3().pull_up();
            w.port_2_4().pull_up();
            w.port_2_5().pull_up();
            w
        });
    }

    fn new_msp() -> Self {
        unsafe {
            SpifiHandle::spifi_msp_init();
        }
        Self {
            _marker: core::marker::PhantomData
        }
    }

    fn send_command(
        cmd: Command,
        addr: u32,
        buff_len: u16,
        read_buffer: &mut [u8],
        write_buffer: &[u8],
        interim_data: u32,
        timeout: u32,
    ) -> Result<(), SpifiHandleError> {
        unsafe {
            SpifiHandle::send_command_private(
                cmd.bits(), 
                addr, 
                buff_len, 
                read_buffer, 
                write_buffer, 
                interim_data, 
                timeout
            )?;
        }
        Ok(())
    }

    unsafe fn send_command_private(
        cmd: u32,
        addr: u32,
        buff_len: u16,
        read_buffer: &mut [u8],
        write_buffer: &[u8],
        interim_data: u32,
        timeout: u32,
    ) -> Result<(), SpifiHandleError>{
        let spifi = SpifiConfig::steal();

        spifi.stat().modify(|_, w| {
            w.intrq().set_bit()
        });
        spifi.address().write(|w| {
            w.address().bits(addr)
        });
        spifi.idata().write(| w| {
            w.bits(interim_data)
        });
        spifi.cmd().write(|w| {
            w.bits(cmd).datalen().bits(buff_len)
        });

        let command = spifi.cmd().read();

        if command.dout().bit_is_set() {
            if (buff_len > 0) && (write_buffer.len() == 0) {
                return Err(SpifiHandleError::INVALID_BUFFER_SIZE)
            }
            for &i in write_buffer {
                spifi.data().write(|w| {
                    w.data8().bits(i)
                });
            }
        } else {
            if (buff_len > 0) && (read_buffer.len() == 0) {
                return Err(SpifiHandleError::INVALID_BUFFER_SIZE);
            }

            for  i in 0..buff_len{
                read_buffer[i as usize] = spifi.data().read().data8().bits() as u8;
            }
        }

        let status = SpifiHandle::wait_cmd_processing(timeout);

        if status.is_ok() && (command.poll().bit_is_set()) {
            return Err(SpifiHandleError::ERROR)
        }

        Ok(())
    }

    #[inline(always)]
    unsafe fn wait_cmd_processing(timeout: u32) -> Result<(), SpifiHandleError> {
        while timeout > 0 {
            if (*SpifiConfig::ptr()).stat().read().intrq().bit_is_set() {
                return Ok(());
            }
        }
        Err(SpifiHandleError::TIMEOUT)
    }
}