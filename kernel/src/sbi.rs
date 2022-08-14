#![allow(unused)]

#[macro_export]
macro_rules! call {
    // v0.1
    ( $eid: expr; $($arg: expr),* ) => {
        {
            let (ret, _) = call!($eid, 0; $($arg),*);

            ret
        }
    };

    // v0.2
    ( $eid: expr, $fid: expr; $($arg0: expr $(,$arg1: expr $(,$arg2: expr $(,$arg3: expr $(,$arg4: expr $(,$arg5: expr)?)?)?)?)?)? ) => {
        {
            let (err, ret): (usize, usize);
            unsafe {
                core::arch::asm!("ecall",
                    in("a7") $eid, lateout("a0") err,
                    in("a6") $fid, lateout("a1") ret,

                  $(in("a0") $arg0, $(in("a1") $arg1, $(in("a2") $arg2, $(in("a3") $arg3, $(in("a4") $arg4, $(in("a5") $arg5)?)?)?)?)?)?
                );
            }

            (err, ret)
        }
    };
}

pub mod legacy {
    const SET_TIMER: usize = 0x00;
    const CONSOLE_PUTCHAR: usize = 0x01;
    const CONSOLE_GETCHAR: usize = 0x02;
    const CLEAR_IPI: usize = 0x03;
    const SEND_IPI: usize = 0x04;
    const REMOTE_FENCE_I: usize = 0x05;
    const REMOTE_SFENCE_VMA: usize = 0x06;
    const REMOTE_SFENCE_VMA_ASID: usize = 0x07;
    const SHUTDOWN: usize = 0x08;

    pub fn set_timer(timer: usize) {
        call!(SET_TIMER; timer);
    }

    pub fn console_putchar(char: usize) {
        call!(CONSOLE_PUTCHAR; char);
    }

    pub fn console_getchar() -> usize {
        call!(CONSOLE_GETCHAR;)
    }

    pub fn shutdown() -> ! {
        call!(SHUTDOWN;);

        unreachable!("should have been shutdown");
    }
}

/* -------------------------------------------------------------------------- */
/*                                     MOD                                    */
/* -------------------------------------------------------------------------- */

pub mod console;
