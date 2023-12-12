use std::{boxed::Box, ffi::{CString, c_char}, marker::Sized, convert::From};
use mysql::{*, prelude::{*, Queryable}};


#[repr(C)]
pub enum ConnResult {
    Error,
    Success(mysql::PooledConn)
}

#[repr(C)]
pub enum PrepareResult {
    Error,
    Success(mysql::Statement)
}

#[repr(C)]
pub enum ExecuteResult {
    Error,
    Success,
    Result
}

#[repr(C)]
pub enum Value_FFI {
    NULL,
    Bytes(Box<[u8]>),
    Int(Box<i64>),
    UInt(Box<u64>),
    Float(Box<f32>),
    Double(Box<f64>),
    /// year, month, day, hour, minutes, seconds, micro seconds
    Date(Box<(u16, u8, u8, u8, u8, u8, u32)>),
    /// is negative, days, hours, minutes, seconds, micro seconds
    Time(Box<(bool, u32, u8, u8, u8, u32)>),
}

impl From<Value> for Value_FFI {
    fn from(value:Value)->Self {
        match value {
            Value::NULL=>Value_FFI::NULL,
            Value::Bytes(val)=>Value_FFI::Bytes(val.into_boxed_slice()),
            Value::Int(val)=>Value_FFI::Int(Box::new(val)),
            Value::UInt(val)=>Value_FFI::UInt(Box::new(val)),
            Value::Float(val)=>Value_FFI::Float(Box::new(val)),
            Value::Double(val)=>Value_FFI::Double(Box::new(val)),
            Value::Date(a,b,c,d,e,f,g)=>Value_FFI::Date(Box::new((a,b,c,d,e,f,g))),
            Value::Time(a,b,c,d,e,f)=>Value_FFI::Time(Box::new((a,b,c,d,e,f))),
        }
    }
}

pub fn into_Value_FFI(vals: Vec<Value>)->Vec<Value_FFI> {
    let mut return_vec=vals.into_iter().map(|x| x.into()).collect::<Vec<Value_FFI>>();
    return_vec.push(Value_FFI::NULL);
    return_vec
}


#[no_mangle]
pub extern "C" fn connect_from_url(url_as_bytes: *const c_char) -> *mut ConnResult {
    if url_as_bytes.is_null() {
        return Box::into_raw(Box::new(ConnResult::Error));
        
    }
    let url_as_cstring=unsafe {
        let cstring=CString::from_raw(url_as_bytes as *mut i8);
        cstring
    };
    let url_as_string={
        let string_val=url_as_cstring.into_string();
        match string_val {
            Ok(url)=>url,
            Err(err)=>{
                return Box::into_raw(Box::new(ConnResult::Error));
                "".to_string()
            }
        }
    };
    let conn_result={
        let conn_opts=if let Ok(opts)=mysql::Opts::from_url(&url_as_string) {
            opts
        } else {
            return Box::into_raw(Box::new(ConnResult::Error));
        };
        let pool=if let Ok(pool)=mysql::Pool::new(conn_opts) {
            pool
        } else {
            return Box::into_raw(Box::new(ConnResult::Error));
        };
        pool.get_conn()
    };
    if let Ok(conn)=conn_result {
        return Box::into_raw(Box::new(ConnResult::Success(conn)));
    } else {
        return Box::into_raw(Box::new(ConnResult::Error));
    }

}

#[no_mangle]
pub extern "C" fn prepare(conn_enum_ptr: *mut ConnResult, sql_as_bytes: *const c_char) -> *mut PrepareResult{
    let conn_enum=unsafe {&mut *conn_enum_ptr};
    let mut conn=if let ConnResult::Success(conn)=conn_enum {
        conn
    } else {
        return Box::into_raw(Box::new(PrepareResult::Error));
    };
    let sql_as_cstring=unsafe {
        let cstring=CString::from_raw(sql_as_bytes as *mut i8);
        cstring
    };
    let sql_as_string={
        let string_val=sql_as_cstring.into_string();
        match string_val {
            Ok(sql)=>sql,
            Err(err)=>{
                return Box::into_raw(Box::new(PrepareResult::Error));
               
            }
        }
    };

    
    let stmt=if let Ok(stmt)=conn.prep(&sql_as_string) {
        stmt
    } else {
        return Box::into_raw(Box::new(PrepareResult::Error));
    };
    return Box::into_raw(Box::new(PrepareResult::Success(stmt)));

}


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


