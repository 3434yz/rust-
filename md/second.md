# An Ok Stack

## 使用```Option```替换```Link```

```rust
    // before
    enum Link {
        Empty,
        More(Box<Node>),
    }

    // after
    type Link = Option<Box<Node>>
```

使用```Option```实现```Link```后，我们可以使用```Option```的```take```方法替换```mem::replace```方法。

```rust
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

```

## 使用map+闭包函数替换模式匹配

```rust
impl List {
    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node|{
            self.head = node.next
            node.elem
        })
    }
}

````

## 范型

- 为什么使用范型
  - 减少重复的代码
  - 提高代码可读性

## 迭代器

- 我们为什么需要迭代器？
  - 编码更加优雅
  - 代码可读性更高

## IntoIter

- 通过实现Iterator这个trait来构建迭代器
  - ```IntoIter```通过对List现有的方法进行简单封装实现迭代器
  - ```IntoIter```的实现:

  ```rust
  // 定义 
  pub struct IntoIter<T>(List<T>)

  // 为List<T>实现转换成IntoIter的方法
  impl<T> List<T> {
    fn into_iter(self) -> IntoIter<T> {
        IntoIter{self}
    }
  }

  // 实现trait
  impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
  }
  ```

  - 测试代码:

  ```rust
  fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
  ```

  - 看起来迭代器似乎完成了，但是这种迭代器的实现有严重的缺陷
    - 由于rust的借用规则，在```list```转换为```IterMut```时它就被"夺舍"了！
    - 为了解决这个问题，我们需要转换函数中的函数签名是对容器本身的“借用”

## Iter

- Iter的目的是避免迭代器让容器本身"失效"（被迭代器独占)
- Iter的实现：

```rust
pub struct<T> Iter <T> {
    next: Option<&Node<T>>
}

impl<T> List <T> {
    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref()
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
```

- 测试代码

```rust
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
```

- ```iter```现在没有“夺舍”容器本身了，思考如何做一个“可写”的迭代器

## IterMut

- 既然是可写的迭代器，那么函数签名就只能是```&mut self```
- ```IterMut```的实现：

```rust
pub struct IterMut<'a,T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    fn iter_mut(&mut self) -> IterMut<'_,T>{
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T>Iterator for IterMut<'a, T> {
    type Item =&'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref_mut();
            & mut node.elem
        })
    }
}
```

- 测试代码：

```rust
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
```
