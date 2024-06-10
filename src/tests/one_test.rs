use self::interfaces::Node;
mod btree_test;
fn test_box() {
    let mut listed: collections::LinkList<u64> = collections::LinkList::new();
}
pub mod interfaces {
    pub trait Node<T>
    where
        T: Clone,
    {
        //panic if out of range
        fn set_val(&mut self, _val: T, _step: usize);
        //panic if out of range
        fn get_val(&self, _step: usize) -> T;
        //panic if out of range
        fn move_to(&mut self, _step: usize);
    }
}
mod collections {
    pub struct Node<T>
    where
        T: Clone,
    {
        pub value: T,
        pub next: Option<*mut Node<T>>,
    }
    impl<T: Clone> Node<T> {
        pub fn new(_val: T) -> Self {
            return Self {
                value: _val,
                next: None,
            };
        }
        // pub fn iter(&mut self) -> iter::Node<'_, T> {}
    }

    pub struct LinkList<T>
    where
        T: Clone,
    {
        head: Option<*mut Node<T>>,
        tail: Option<*mut Node<T>>,
        _len: usize,
    }
    impl<T: Clone> Clone for LinkList<T> {
        fn clone(&self) -> Self {
            Self {
                head: self.head.clone(),
                tail: self.tail.clone(),
                _len: self._len.clone(),
            }
        }
    }
    impl<T: Clone> LinkList<T> {
        pub fn new() -> Self {
            return Self {
                head: None,
                tail: None,
                _len: 0,
            };
        }
        pub fn iter(&self) -> iter::Node<T> {
            return iter::Node::new(self.head.clone().unwrap(), self._len);
        }
        pub fn push_back(&mut self, _val: T) {
            let mut bval: Box<Node<T>> = Box::new(Node::new(_val));
            let mut bptr = Box::leak(bval);
            let mut _node: Option<*mut Node<T>> = None;
            unsafe {
                _node = Some(bptr as *mut Node<T>);
            }
            if self._len == 0 {
                self.head = _node.clone();
                self.tail = _node.clone();
                self._len += 1;
                return;
            }
            unsafe {
                (*self.tail.unwrap()).next = _node;
            }
            self.tail = _node.clone();
            self._len += 1;
        }
        pub fn push_front(&mut self, _val: T) {
            let mut bval: Box<Node<T>> = Box::new(Node::new(_val));
            let mut bptr = Box::leak(bval);
            let mut _node: Option<*mut Node<T>> = None;
            unsafe {
                _node = Some(bptr as *mut Node<T>);
            }
            if self._len == 0 {
                self.head = _node.clone();
                self.tail = _node.clone();
                self._len += 1;
                return;
            }
            bptr.next = self.head.clone();
            self.head = _node.clone();
            self._len += 1;
        }
    }
    impl<T: Clone> super::interfaces::Node<T> for LinkList<T> {
        fn set_val(&mut self, _val: T, _step: usize) {
            if self._len >= _step {
                panic!("out of range {_step} {}", self._len);
            }
            let mut i: usize = 0;
            let mut ptr = self.head.clone().unwrap();
            while i < _step {
                unsafe {
                    ptr = (*ptr).next.clone().unwrap();
                }
            }
            unsafe {
                (*ptr).value = _val;
            }
        }

        fn get_val(&self, _step: usize) -> T {
            if self._len >= _step {
                panic!("out of range {_step} {}", self._len);
            }
            let mut i: usize = 0;
            let mut ptr = self.head.clone().unwrap();
            while i < _step {
                unsafe {
                    ptr = (*ptr).next.clone().unwrap();
                }
            }
            unsafe {
                return (*ptr).value.clone();
            }
        }

        fn move_to(&mut self, _step: usize) {
            if self._len >= _step {
                panic!("out of range {_step} {}", self._len);
            }
            let mut i: usize = 0;
            let mut ptr = self.head.clone().unwrap();
            while i < _step {
                unsafe {
                    ptr = (*ptr).next.clone().unwrap();
                }
            }
            self._len -= _step;
            self.head = Some(ptr);
        }
    }
    pub mod iter {
        pub struct Node<T>
        where
            T: Clone,
        {
            node: Option<*mut super::Node<T>>,
        }
        impl<T: Clone> Node<T> {
            pub fn new(_val: *mut super::Node<T>, _size: usize) -> Self {
                return Self { node: Some(_val) };
            }
            pub fn next(&self) -> Self {
                if self.node.is_none() {
                    panic!("null node");
                }
                let mut nextnode = self.node.clone().unwrap();
                unsafe {
                    return Self {
                        node: (*nextnode).next.clone(),
                    };
                }
            }
            //add node to tail
            pub fn push_back(&mut self, _next: &Self) {
                if self.node.is_none() {
                    panic!("null node")
                }
                let mut rawptr = self.node.clone().unwrap();
                unsafe {
                    (*rawptr).next = _next.node.clone();
                }
            }
        }
    }
}
