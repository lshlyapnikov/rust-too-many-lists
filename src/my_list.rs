use std::ops::Deref;

pub struct List<A> {
    head: Option<NonEmptyList<A>>,
}

struct NonEmptyList<A> {
    head: A,
    tail: Option<Box<NonEmptyList<A>>>,
}

impl<A> List<A> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, a: A) {
        let new_head = NonEmptyList {
            head: a,
            tail: self.head.take().map(|nel| Box::new(nel)),
        };

        self.head = Some(new_head);
    }

    pub fn pop(&mut self) -> Option<A> {
        self.head.take().map(|mut nel| {
            self.head = nel.tail.take().map(|x| *x);
            nel.head
        })
    }

    pub fn size(&self) -> usize {
        let mut result = 0;
        let mut list: Option<&NonEmptyList<A>> = self.head.as_ref();
        while let Some(nel) = list {
            list = nel.tail.as_ref().map(|x| x.deref());
            result += 1;
        }
        result
    }
}

impl<A> Drop for List<A> {
    fn drop(&mut self) {
        let mut list: Option<NonEmptyList<A>> = self.head.take();
        while let Some(mut nel) = list {
            list = nel.tail.take().map(|x| *x);
        }
    }
}

#[cfg(test)]
mod test {
    use std::mem;

    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.size(), 0);
        assert_eq!(list.pop(), None);

        list.push(1);
        assert_eq!(list.size(), 1);

        list.push(2);
        list.push(3);

        assert_eq!(list.size(), 3);
        assert_eq!(list.pop(), Some(3));

        assert_eq!(list.size(), 2);
        assert_eq!(list.pop(), Some(2));

        assert_eq!(list.size(), 1);
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.size(), 0);
        assert_eq!(list.pop(), None);

        mem::drop(list);
    }
}
