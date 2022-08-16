use super::*;

pub const SIZE: usize = 4096;

/* -------------------------------------------------------------------------- */
/*                                    PAGE                                    */
/* -------------------------------------------------------------------------- */

pub struct Page<'allocator, Allocator: Allocate> {
    pub number: Number,
    allocator: &'allocator core::cell::RefCell<Allocator>,
}

impl<'allocator, Allocator: Allocate> Drop for Page<'allocator, Allocator> {
    fn drop(&mut self) {
        self.allocator.borrow_mut().dealloc(self.number);
    }
}

#[derive(Clone, Copy)]
pub struct Number(pub usize);
impl From<Address> for Number {
    fn from(Address(addr): Address) -> Number {
        Number(addr / SIZE)
    }
}

impl core::fmt::Debug for Number {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Number(ppn) = self;
        fmt.write_fmt(format_args!("{}({:?})", ppn, Address::from(*self)))
    }
}

/* -------------------------------------------------------------------------- */
/*                                  ALLOCATOR                                 */
/* -------------------------------------------------------------------------- */

pub trait Allocate: Sized + From<Region> {
    fn alloc(&mut self) -> Option<Number>;
    fn dealloc(&mut self, page_number: Number);
}

pub struct Pool<Allocator: Allocate>(core::cell::RefCell<Allocator>);
impl<Allocator: Allocate> From<Allocator> for Pool<Allocator> {
    fn from(allocator: Allocator) -> Pool<Allocator> {
        Pool(core::cell::RefCell::new(allocator))
    }
}

impl<Allocator: Allocate> Pool<Allocator> {
    #[allow(unused)]
    pub fn get(&self) -> Option<Page<Allocator>> {
        let Pool(allocator) = self;
        let result = allocator.borrow_mut().alloc();

        result.map(|number| Page { number, allocator })
    }
}

pub mod allocator {
    use super::*;

    pub mod stack {
        use super::*;

        pub struct Stack<'stack, Element: 'stack> {
            size: usize,
            elements: &'stack mut [Element],
        }

        impl<'stack, Element: 'stack> From<Region> for Stack<'stack, Element> {
            fn from(Region(Address(base), len): Region) -> Stack<'stack, Element> {
                Stack {
                    size: 0,
                    elements: {
                        let ptr = core::ptr::slice_from_raw_parts_mut(base as *mut Element, len);
                        unsafe { ptr.as_mut() }.expect("invalid start address")
                    },
                }
            }
        }

        impl<'stack, Element: 'stack> Stack<'stack, Element> {
            fn push(&mut self, item: Element) {
                drop(core::mem::replace(&mut self.elements[self.size], item));
                self.size += 1;
            }
            fn pop(&mut self) -> Option<Element> {
                self.size.gt(&0).then(|| {
                    self.size -= 1;

                    core::mem::replace(&mut self.elements[self.size], unsafe {
                        core::mem::zeroed()
                    })
                })
            }
        }

        pub struct Allocator {
            stack: Stack<'static, Number>,

            base: Address,
            n_alloc: usize,
            n_total: usize,
        }

        impl From<Region> for Allocator {
            fn from(Region(Address(base), size): Region) -> Allocator {
                let n_page = size / SIZE;
                let n_used = (n_page * core::mem::size_of::<Number>() + SIZE - 1) / SIZE;

                Allocator {
                    stack: Stack::from(Region(Address(base), n_page - n_used)),

                    base: Address(base + n_used * SIZE),
                    n_alloc: 0,
                    n_total: n_page - n_used,
                }
            }
        }

        impl Allocate for Allocator {
            fn alloc(&mut self) -> Option<Number> {
                self.stack.pop().or_else(|| {
                    self.n_alloc.lt(&self.n_total).then(|| {
                        let index = self.n_alloc;
                        self.n_alloc += 1;

                        let Number(ppn) = Number::from(self.base);
                        Number(ppn + index)
                    })
                })
            }
            fn dealloc(&mut self, page_number: Number) {
                self.stack.push(page_number);
            }
        }
    }
}
