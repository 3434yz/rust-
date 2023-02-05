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
