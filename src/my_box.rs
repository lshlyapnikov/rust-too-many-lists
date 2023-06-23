use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct MyBox<A> {
    value: A,
}

impl<A> MyBox<A> {
    pub fn new(value: A) -> Self {
        Self { value }
    }
}

impl<A> Deref for MyBox<A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<A> DerefMut for MyBox<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;

    use super::MyBox;
    #[test]
    fn basics() {
        let x = MyBox::new(10u32);
        let mut y = MyBox::new(vec![0u32; 5]);
        y[1] = 100;
        y[2] = 200;
        println!("{:?}", x);
        println!("{:?}", *x);
        println!("{:?}", x.deref());

        assert_eq!(10, *x);
        assert_eq!(10, *x.deref());

        println!("{:?}", y);
        println!("{:?}", *y);
        println!("{:?}", y.deref());
    }
}
