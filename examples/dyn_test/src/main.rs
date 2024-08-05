/*
// not safe becasue Size requirement
trait Foo: Size+Send+Sync+'static {
    fn do_something(&self) -> f64;
}
*/

/*
// not safe becasue generic argument
trait Foo:Send+Sync+'static {
    fn do_something<T>(&self, v:T) -> f64;
}
*/

/*
// not safe becasue generic argument
trait Foo {
    fn do_something() -> Self;
}
*/
trait Foo {
    fn do_something(self) -> f64;
}
struct Bar {
    foo: Box<dyn Foo>,
}

fn main() {}
