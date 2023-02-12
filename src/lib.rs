use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub trait SelfReferencing {
    fn clone_self_ref(&self) -> Rc<RefCell<Self>>;
    fn clone_self_ref_weak(&self) -> Weak<RefCell<Self>>;
}

#[macro_export]
macro_rules! implement_self_referencing {
    (
        $struct:ident, 
        $self_ref_field:ident
    ) => {
        impl SelfReferencing for $struct {
            fn clone_self_ref(&self) -> Rc<RefCell<Self>> {
                self.$self_ref_field
                .clone()
                .upgrade()
                .expect("Getting RC self reference from weak self reference failed - very weird") 
            }

            fn clone_self_ref_weak(&self) -> Weak<RefCell<Self>> {
                self.$self_ref_field
                .clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct SelfRefTest {
        me: Weak<RefCell<Self>>
    }

    impl SelfRefTest {
        fn new() -> Rc<RefCell<Self>> {
            Rc::new_cyclic(|me| {
                RefCell::new(
                    Self {
                        me: me.clone(),
                    }
                )
            })
        }
    }

    implement_self_referencing!(SelfRefTest, me);

    #[test]
    fn it_works() {
        let foo = SelfRefTest::new();
        let bar = foo.borrow().clone_self_ref();

        assert!(Rc::ptr_eq(&foo, &bar));
    }
}
