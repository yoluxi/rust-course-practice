// 生命周期

// 手动标注生命周期解决什么问题？
// 编译器无法自动推断出引用的生命周期时，需要手动标注，使得编译通过

// 编译器为什么没法推导？

// 怎么使用？
// 'a
fn main() {
    // println!("Hello, world!");
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    foo.share();
    print!("{:?}", loan);
}

#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}
