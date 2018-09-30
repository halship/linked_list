//! std の双方向リストの機能縮小版に remove_if を足したようなもの

use std::marker::PhantomData;
use std::ptr::NonNull;

/// 双方向リストのノード
struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    elem: T,
}

/// 双方向リスト
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    /// 双方向リストを生成して返す
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    /// 先頭に要素を追加する
    pub fn push_front(&mut self, elem: T) {
        let mut node = Box::new(Node {
            prev: None,
            next: self.head,
            elem,
        });

        let new_node = NonNull::from(&mut *node);
        Box::into_raw(node);

        if let Some(mut head) = self.head {
            unsafe {
                head.as_mut().prev = Some(new_node);
            }
        }
        if let None = self.tail {
            self.tail = Some(new_node);
        }
        self.head = Some(new_node);
    }

    /// 末尾に要素を追加する
    pub fn push_back(&mut self, elem: T) {
        let mut node = Box::new(Node {
            prev: self.tail,
            next: None,
            elem,
        });

        let new_node = NonNull::from(&mut *node);
        Box::into_raw(node);

        if let Some(mut tail) = self.tail {
            unsafe {
                tail.as_mut().next = Some(new_node);
            }
        }
        if let None = self.head {
            self.head = Some(new_node);
        }
        self.tail = Some(new_node);
    }

    /// 先頭の要素を削除する
    pub fn pop_front(&mut self) {
        if let Some(mut head) = self.head {
            unsafe {
                if let Some(mut next) = head.as_mut().next {
                    next.as_mut().prev = None;
                } else {
                    self.tail = None;
                }
                self.head = head.as_ref().next;

                Box::from_raw(head.as_ptr());
            }
        }
    }

    /// 末尾の要素を削除する
    pub fn pop_back(&mut self) {
        if let Some(mut tail) = self.tail {
            unsafe {
                if let Some(mut prev) = tail.as_mut().prev {
                    prev.as_mut().next = None;
                } else {
                    self.head = None;
                }
                self.tail = tail.as_ref().prev;

                Box::from_raw(tail.as_ptr());
            }
        }
    }

    /// リストの先頭の要素を返す
    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.map(|node| &(*node.as_ptr()).elem) }
    }

    /// リストの末尾の要素を返す
    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.map(|node| &(*node.as_ptr()).elem) }
    }

    /// 条件に合った要素を削除する
    pub fn remove_if<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut opt_node = self.head;
        let mut head_node = None;
        let mut tail_node = None;

        while let Some(mut node) = opt_node {
            unsafe {
                if f(&node.as_ref().elem) {
                    if let Some(mut prev_node) = node.as_mut().prev {
                        prev_node.as_mut().next = node.as_ref().next;
                    }
                    if let Some(mut next_node) = node.as_mut().next {
                        next_node.as_mut().prev = node.as_ref().prev;
                    }

                    Box::from_raw(node.as_ptr());
                } else {
                    if let None = head_node {
                        head_node = opt_node;
                    }

                    tail_node = opt_node;
                }

                opt_node = node.as_ref().next;
            }
        }

        self.head = head_node;
        self.tail = tail_node;
    }

    /// イテレータを返す
    pub fn iter(&self) -> Iter<T> {
        Iter {
            node: self.head,
            phantom: PhantomData,
        }
    }

    /// mutable なイテレータを返す
    pub fn iter_mut(&self) -> IterMut<T> {
        IterMut {
            node: self.head,
            phantom: PhantomData,
        }
    }

    /// リストが空かどうかを返す
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// 全要素を削除する
    pub fn clear(&mut self) {
        let mut head_node = self.head;

        while let Some(node) = head_node {
            unsafe {
                let next_node = node.as_ref().next;
                Box::from_raw(node.as_ptr());
                head_node = next_node;
            }
        }

        self.head = None;
        self.tail = None;
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut head_node = self.head;

        while let Some(node) = head_node {
            unsafe {
                let next_node = node.as_ref().next;
                Box::from_raw(node.as_ptr());
                head_node = next_node;
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

// LinkedListのイテレータ
pub struct Iter<'a, T: 'a> {
    node: Option<NonNull<Node<T>>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            match self.node {
                Some(node) => {
                    self.node = node.as_ref().next;
                    Some(&(*node.as_ptr()).elem)
                }
                None => None,
            }
        }
    }
}

// LinkedListのmutableなイテレータ
pub struct IterMut<'a, T: 'a> {
    node: Option<NonNull<Node<T>>>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        unsafe {
            match self.node {
                Some(node) => {
                    self.node = node.as_ref().next;
                    Some(&mut (*node.as_ptr()).elem)
                }
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn it_works() {
        let mut list = LinkedList::new();

        list.push_back(1);
        assert_eq!(Some(&1), list.front());
        assert_eq!(Some(&1), list.back());

        list.push_back(2);
        assert_eq!(Some(&1), list.front());
        assert_eq!(Some(&2), list.back());

        list.push_front(0);
        assert_eq!(Some(&0), list.front());
        assert_eq!(Some(&2), list.back());

        list.pop_back();
        assert_eq!(Some(&0), list.front());
        assert_eq!(Some(&1), list.back());

        list.pop_front();
        assert_eq!(Some(&1), list.front());
        assert_eq!(Some(&1), list.back());

        list.pop_front();
        assert_eq!(None, list.front());
        assert_eq!(None, list.back());
    }

    #[test]
    fn iter_test() {
        let correct = vec![0, 1, 2, 3];

        let mut list = LinkedList::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        for (i, n) in list.iter().enumerate() {
            assert_eq!(correct[i], *n);
        }
    }

    #[test]
    fn itermut_test() {
        let correct = vec![0, 2, 4, 6];

        let mut list = LinkedList::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        for (i, n) in list.iter_mut().enumerate() {
            *n *= 2;
            assert_eq!(correct[i], *n);
        }
    }

    #[test]
    fn remove_if_test() {
        let correct = vec![0, 2, 4];

        let mut list = LinkedList::new();

        for i in 0..6 {
            list.push_back(i);
        }

        list.remove_if(|i| i % 2 == 1);

        for (i, n) in list.iter().enumerate() {
            assert_eq!(correct[i], *n);
        }

        list.remove_if(|_| true);
        assert_eq!(None, list.front());
        assert_eq!(None, list.back());
    }

    #[test]
    fn clear_test() {
        let mut list = LinkedList::new();

        list.push_back(0);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.clear();

        assert!(list.is_empty());
    }
}
