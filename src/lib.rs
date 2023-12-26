use std::{boxed::Box, ffi::{CString, c_char, c_int}, marker::Sized, convert::From};
use mysql::{*, prelude::{*, Queryable}};

mod connections;
mod prepare;
mod execute;
mod types;
pub use connections::*;
pub use prepare::*;
pub use execute::*;
pub use types::*;



#[no_mangle]
pub extern "C" fn execute_drop(conn_enum_ptr: *mut ConnResult, stmt_enum_ptr: *mut PrepareResult, params: mysql::Params) -> *mut ExecuteResult {
    let conn_enum=unsafe {&mut *conn_enum_ptr};
    let mut conn=if let ConnResult::Success(conn)=conn_enum {
        conn
    } else {
        return Box::into_raw(Box::new(ExecuteResult::Error));
    };
    if let PrepareResult::Success(val)=unsafe{&mut *stmt_enum_ptr} {
        conn.exec_iter(val.clone(), params);
    } else {
        return Box::into_raw(Box::new(ExecuteResult::Error));
    }
    return Box::into_raw(Box::new(ExecuteResult::Success));
}


#[no_mangle]
pub extern "C" fn rust_to_julia()->*mut ArrayStruct<ValueFFI> {
    let vec=vec![ValueFFI::NULL, ValueFFI::Bytes(Box::new(ByteString::from("this is the end"))), ValueFFI::Int(Box::new(64 as i64)), ValueFFI::UInt(Box::new(64 as u64)), ValueFFI::Float(Box::new(32.0 as f32)), ValueFFI::Double(Box::new(64.0 as f64)), ValueFFI::Date(Box::new((1,1,1,1,1,1,1))), ValueFFI::Time(Box::new((0,1,1,1,1,1)))];
    Box::into_raw(Box::new(ArrayStruct::from(vec)))
}