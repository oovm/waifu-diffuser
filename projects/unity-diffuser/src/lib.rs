

#[no_mangle]
pub extern fn test_run_method(val:i32)->i32{
    return val + 1;
}

pub struct RustObject {

}

#[no_mangle]
pub extern fn rust_object_new()-> * const RustObject{
    let obj = RustObject{
        val :0.
    };
    let b = Box::new(obj);
    return Box::into_raw(b);
}

#[no_mangle]
pub extern fn rust_object_dispose(ptr: * mut RustObject){
    unsafe{
        let _ = Box::from_raw(ptr);
    }
}