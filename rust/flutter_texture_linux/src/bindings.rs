/* automatically generated by rust-bindgen 0.71.1 */

unsafe extern "C" {
    pub fn fltx_gpu_take_five() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 8usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 4usize];
    ["Offset of field: Foo::a"][::std::mem::offset_of!(Foo, a) - 0usize];
    ["Offset of field: Foo::b"][::std::mem::offset_of!(Foo, b) - 4usize];
};
unsafe extern "C" {
    pub fn fltx_gpu_foo_new() -> *mut Foo;
}
