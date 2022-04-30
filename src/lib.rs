extern crate rb_sys;

use std::ffi::{CString, CStr};

// use rb_sys::{rb_define_module, rb_define_module_function};
use rb_sys::{
    rb_inspect, rb_string_value_cstr, VALUE, rb_define_class, rb_cObject, rb_define_singleton_method, rb_define_method,
};

#[repr(C)]
struct WrappedValue {
    value: u32,
    padding: u32,
}

#[no_mangle]
unsafe extern "C" fn test_hola() -> VALUE {
    use rb_sys::ruby_special_consts::RUBY_Qnil;
    println!("Hello from Rust");
    let qnil = WrappedValue {
        value: RUBY_Qnil as u32,
        padding: 0,
    };
    std::mem::transmute::<WrappedValue, VALUE>(qnil)
}

#[no_mangle]
unsafe extern "C" fn test_show_self(rbself: VALUE) -> VALUE {
    let mut inspect = Box::new(rb_inspect(rbself));
    let inspect = inspect.as_mut() as *mut VALUE;
    let ptr = rb_string_value_cstr(inspect);
    let cstr = CStr::from_ptr(ptr);
    println!("self is: {:?}", cstr);

    rbself
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_oye_rust() {
    println!("Rust loaded");
    let name = CString::new("Rust").unwrap();
    let function_name = CString::new("oye").unwrap();
    let object = unsafe { rb_cObject };
    let klass = unsafe { rb_define_class(name.as_ptr(), object) };

    unsafe { rb_define_singleton_method(klass, function_name.as_ptr(), Some(test_hola), 0) }

    let function_name = CString::new("mostrarme").unwrap();
    let callback = unsafe {
        std::mem::transmute::<unsafe extern "C" fn(VALUE) -> VALUE, unsafe extern "C" fn() -> VALUE>(
            test_show_self,
        )
    };

    unsafe { rb_define_method(klass, function_name.as_ptr(), Some(callback), 0) }
}
