use std::ffi::c_int;
use mysql::prelude::Queryable;
use crate::{ArrayStruct, ByteString, ValueFFI, ConnResult, PrepareResult};

#[repr(C)]
pub enum ExecuteResult {
    Error,
    Success,
}

#[repr(C)]
pub struct QueryResultFFI {
    tag: c_int,
    message: *mut ByteString,
    colnames: *mut ArrayStruct<ByteString>,
    vals: *mut ArrayStruct<ArrayStruct<ValueFFI>>
}

impl Default for QueryResultFFI {
    fn default()->Self {
        QueryResultFFI {
            tag: 0 as c_int,
            message: Box::into_raw(Box::new("".into())),
            colnames: Box::into_raw(Box::new(ArrayStruct::from(Vec::<ByteString>::new()))),
            vals: Box::into_raw(Box::new(ArrayStruct::from(Vec::<ArrayStruct<ValueFFI>>::new())))
        }
        
    }
}

#[repr(C)]
pub enum QuerySetFFI {
    Error,
    Success(*mut ArrayStruct<QueryResultFFI>)
}




#[no_mangle]
pub extern "C" fn execute_query(conn: *mut ConnResult, query: *mut ByteString) -> *mut QuerySetFFI {
    let mut err=Box::into_raw(Box::new(QuerySetFFI::Error));
    let conn=if let ConnResult::Success(ref mut conn)=unsafe {&mut *conn} {
        conn
    } else {
        return err;
    };
    let query=if let Ok(val)=String::try_from(unsafe {&*query}) {
        val
    } else {
        return err;
    };
    let execute_result=conn.query_iter(&query);
    match execute_result {
        Ok(mut result)=>{
            let mut results_ffi=Vec::<QueryResultFFI>::new();
            'result: loop {
                if let Some(set)=result.iter() {
                    let columns=set.columns();
                    let column_names=columns.as_ref().iter().map(|x| ByteString::from(x.name_str().to_string())).collect::<Vec<ByteString>>();
                    let colnames=ArrayStruct::from(column_names);
                    let mut rows=Vec::<ArrayStruct<ValueFFI>>::new();
                    for i in set {
                        match i {
                            Ok(row)=>{
                                let row_vals=row.unwrap().into_iter().map(|x| ValueFFI::from(x)).collect::<Vec<ValueFFI>>();
                                rows.push(ArrayStruct::from(row_vals));
                            },
                            Err(err)=>{
                                results_ffi.push(QueryResultFFI{tag: 0 as c_int, message: Box::into_raw(Box::new(ByteString::from(err.to_string()))), ..Default::default()});
                                continue 'result;  
                            }
                        }
                    }
                    results_ffi.push(QueryResultFFI{tag: 1 as c_int, vals: Box::into_raw(Box::new(ArrayStruct::from(rows))), ..Default::default()});

                } else {
                    break;
                }
            }
            return Box::into_raw(Box::new(QuerySetFFI::Success(Box::into_raw(Box::new(ArrayStruct::from(results_ffi))))));
        },
        Err(_)=>{return err;}
    }

}
/*
#[no_mangle]
pub extern "C" fn execute_statement(conn: *mut ConnResult, statement: *mut PrepareResult, params: *mut ArrayStruct<ValueFFI>) -> *mut QuerySetFFI {
    let mut err=Box::into_raw(Box::new(QuerySetFFI::Error));
    let conn=if let ConnResult::Success(ref mut conn)=unsafe {&*conn} {
        conn
    } else {
        return err;
    };
    let statement=if let PrepareResult::Success(ref mut val)=unsafe {&*statement} {
        val
    } else {
        return err;
    };
    match conn.execute_iter(statement, )

}
*/