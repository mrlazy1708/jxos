pub fn main() {
    let pool = {
        let ram = crate::mm::Region(crate::mm::Address(0x80400000), 0x400000);
        let allocator = crate::mm::page::allocator::stack::Allocator::from(ram);

        crate::mm::page::Pool::from(allocator)
    };

    if let Some(page) = pool.get() {
        crate::printk!("get page {:?}", page.number);

        if let Some(page) = pool.get() {
            crate::printk!("get another page {:?}", page.number);
        };
    };

    if let Some(page) = pool.get() {
        crate::printk!("free and get page {:?}", page.number);
    };
}
