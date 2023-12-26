use crate::{ConnResult, ByteString};
use mysql::prelude::Queryable;
#[repr(C)]
pub enum PrepareResult {
    Error,
    Success(mysql::Statement)
}


#[no_mangle]
pub extern "C" fn prepare(conn_result: *mut ConnResult, query: *mut ByteString)->*mut PrepareResult {
    let err=Box::into_raw(Box::new(PrepareResult::Error));
    let conn=if let &mut ConnResult::Success(ref mut conn)=unsafe{&mut *conn_result} {
        conn
    } else {
        return err;
    };
    let query_result=String::try_from(unsafe{&*query});
    let query=if let Ok(val)=query_result {
        val
    } else {
        return err;
    };

    let stmt_result=conn.prep(&query);
    let statement=if let Ok(val)=stmt_result {
        val
    } else {
        return err;
    };
    Box::into_raw(Box::new(PrepareResult::Success(statement)))
}


#[no_mangle]
pub extern "C" fn drop_prepare(statement: *mut PrepareResult) {
    let _= unsafe {Box::from_raw(statement)};
}