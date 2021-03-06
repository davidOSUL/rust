// FIXME: missing sysroot spans (#53081)
// ignore-i586-unknown-linux-gnu
// ignore-i586-unknown-linux-musl
// ignore-i686-unknown-linux-musl
use std::thread;
use std::rc::Rc;

#[derive(Debug)]
struct Port<T>(Rc<T>);

fn main() {
    #[derive(Debug)]
    struct Foo {
      _x: Port<()>,
    }

    impl Drop for Foo {
        fn drop(&mut self) {}
    }

    fn foo(x: Port<()>) -> Foo {
        Foo {
            _x: x
        }
    }

    let x = foo(Port(Rc::new(())));

    thread::spawn(move|| {
        //~^ ERROR `std::rc::Rc<()>` cannot be sent between threads safely
        let y = x;
        println!("{:?}", y);
    });
}
