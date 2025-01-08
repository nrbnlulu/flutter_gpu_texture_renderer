mod bindings;

fn add_five(x: i32) -> i32 {
    unsafe { x + bindings::fltx_gpu_take_five() }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let res = add_five(5);
        assert_eq!(res, 10);
    }
}
