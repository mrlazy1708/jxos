/* -------------------------------------------------------------------------- */
/*                                   STRUCT                                   */
/* -------------------------------------------------------------------------- */

#[derive(Clone, Copy)]
pub struct Address(pub usize);
impl From<page::Number> for Address {
    fn from(page::Number(ppn): page::Number) -> Address {
        Address(ppn * page::SIZE)
    }
}

impl core::fmt::Debug for Address {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Address(addr) = self;
        fmt.write_fmt(format_args!("{:#018x}", addr))
    }
}

pub struct Region(pub Address, pub usize);

/* -------------------------------------------------------------------------- */
/*                                     MOD                                    */
/* -------------------------------------------------------------------------- */

pub mod page;
