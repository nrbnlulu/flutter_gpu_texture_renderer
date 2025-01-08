use bindings::Foo;

mod bindings;

fn add_five(x: i32) -> i32 {
    unsafe { x + bindings::fltx_gpu_take_five() }
}
fn get_foo() -> Foo {
    unsafe { *bindings::fltx_gpu_foo_new() }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let res = add_five(5);
        let foo = get_foo();

        assert_eq!(foo.a, 22);
        assert_eq!(res, 10);
    }
}
