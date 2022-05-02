extern crate coffret;
extern crate rb_sys;

use coffret::class;
use coffret::exception;
use std::error::Error;
use std::ffi::CStr;

// use rb_sys::{rb_define_module, rb_define_module_function};
use rb_sys::{rb_define_singleton_method, rb_inspect, rb_string_value_cstr, VALUE};

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

fn init_oye_rust_internal() -> Result<(), Box<dyn Error>> {
    println!("Rust loaded");
    let object = class::object_class();
    let klass = class::define_class("Rust", object);

    //unsafe { rb_define_singleton_method(klass, function_name.as_ptr(), Some(test_hola), 0) }

    let callback = class::make_callback(&test_show_self);

    class::define_method(klass, "mostrarme", callback, 0);
    Ok(())
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_oye_rust() {
    match init_oye_rust_internal() {
        Err(e) => exception::rustly_raise(e.as_ref()),
        Ok(_) => {}
    }
}
