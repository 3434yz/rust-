# A Bad Stack

## 数据结构定义

```rust
enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

struct List {
    head: Link,
}
```

- rust没有```null```，使用枚举中的空枚举实现“空指针”。
- `Box<T>` 是智能指针，独占、离开作用域后自动gc。

## List的方法及其实现

- new
  - 创建一个空链表
  - 实现:

  ```rust
    impl List {
        fn new() -> Self {
            List { head: Link::Empty }
        }
    }
  ```

  - Self是一个语法糖，代表```impl Type{}```中的```Type```

- push
  - 向Stack中压入元素
  - 实现:

    ```rust
    impl List {
        fn push(&mut self, elem: i32) {
            let box_node = Link::More(Box::new(Node {
                elem,
                next: mem::replace(&mut self.head, Link::Empty),
            }));
            self.head = box_node
        }
    }
    ```

  - 方法中的```self```形参类似golang中struct的方法```golang func (t typeName)funcName{}```中的t，修饰符```&mut```代表"可变借用"。

  - 变量与可变性以及借用规则。
    - 没有被```mut```修饰的值是无法进行改变的。
    - 为什么使用```&mut```修饰？
      - ```push```方法存在对```List```的修改操作，需要```mut```修饰。
      - 为了客户端调用```push```方法后不丢失```List.head```的对链表中头节点的所有权，我们需要使用```&```修饰（详见rust借用规则）。

  - ```mem::replace()```简介
    - 方法签名：```rust pub const fn replace<T>(dest: &mut T, src: T) -> T```。
    - 作用：将```dest```的值弹出函数（返回值）,再将值```src```写入```dest```。
    - 为什么要使用```replace```方法？
      - 插入过程分为：（1）新节点指向头节点指向的部分。（2）头节点再指向这个新的节点。在过程（1）中我们依据常识会写出：

      ```rust
            let box_node = Link::More(Box::new(Node {
                elem,
                next: self.head,
            }));
      ```

      但是执行cargo check检查的时候编译器会提醒我们:"[E0507] cannot move out of `self.head` which is behind a mutable reference..."为什么会这样？这是rust安全性的体现之一；避免同一个内存块被两个```&mut```指向。为什么是安全性的体现？看下段代码，并假设这段代码可以编过：

      ```rust
        impl List {
            fn push(&mut self, elem: i32) {
                let box_node = Link::More(Box::new(Node {
                    elem,
                    next: self.head,
                }));

                // 假设Node有一个改变next的值的方法op
                self.head.op();
                /*
                    此时box_node.next的值也跟着改变了！
                */
                self.head = box_node
            }
        }

      ```

  - pop
    - 弹出head指向的节点
    - 实现:

    ```rust
        impl List {
            fn pop(&mut self) ->Option<i32> {
                match mem::replace(&mut self.head,Link::Empty) {
                    Link::More(node) => {
                        self.head = node.next;
                        Some(node.elem)
                    }
                    Link::Empty => None,
                }
            }
        }
    ```

    - 默认情况下match会发生借用。
    - ```match```使用```replace```的理由和```push```中类似。我们需要取得```head```指向的```Node```，同时避免取出来的工作指针```node```和```head```指向同一块内存。

    ```rust
        impl List {
            fn pop(&mut self) ->Option<i32> {
                let link = mem::replace(&mut self.head, Link::Empty);
                match link {
                    Link::More(node) => {
                        self.head = node.next;
                        Some(node.elem)
                    }
                    Link::Empty => None,
                }
            }
        }
    ```

## 为了List实现Drop-Trait

- 直觉上来说可以这样实现:

```rust
impl Drop for List {
    fn drop(&mut self) {
        // NOTE: you can't actually explicitly call `drop` in real Rust code;
        // we're pretending to be the compiler!
        self.head.drop(); // tail recursive - good!
    }
}

impl Drop for Link {
    fn drop(&mut self) {
        match *self {
            Link::Empty => {} // Done!
            Link::More(ref mut boxed_node) => {
                boxed_node.drop(); // tail recursive - good!
            }
        }
    }
}

impl Drop for Box<Node> {
    fn drop(&mut self) {
        self.ptr.drop(); // uh oh, not tail recursive!
        dealloc(self.ptr);
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.next.drop();
    }
}


```

然而我们执行```cargo check```时，编译器产生了错误：

```
84 | impl Drop for Box<Node> {
   | ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: conflicting implementation in crate `alloc`:
           - impl<T, A> Drop for Box<T, A>
             where A: Allocator, T: ?Sized;

error[E0120]: the `Drop` trait may only be implemented for local structs, enums, and unions
  --> src/first.rs:84:15
   |
84 | impl Drop for Box<Node> {
   |               ^^^^^^^^^ must be a struct, enum, or union in the current crate

```

rust的"孤儿原则"只允许我们在本地crate中impl-trait。于是我们无法为```Box<Node>```实现```Drop-Trait```

```rust
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}
// 等价的实现
impl Drop for List {
    // add code here
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        loop {
            match cur_link {
                Link::More(mut box_node) => {
                    cur_link = mem::replace(&mut box_node.next, Link::Empty);
                }
                Link::Empty => {
                    break;
                }
            }
        }
    }
}
```
