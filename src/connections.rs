use std::ffi::c_int;

#[repr(C)]
pub struct ConnectionOpts{
    pool: mysql::Pool,
}

#[repr(C)]
pub enum PoolResult {
    Error,
    Success(mysql::Pool)
}

#[repr(C)]
pub enum ConnResult {
    Error,
    Success(mysql::PooledConn)
}

#[no_mangle]
pub extern "C" fn get_pool_from_url(url_as_bytes: *const (c_int, *const u8))->*mut PoolResult {
    let err=Box::into_raw(Box::new(PoolResult::Error));
    let url_str={
        let bytes_data=unsafe{&*url_as_bytes};
        let bytes_array=unsafe {std::slice::from_raw_parts(bytes_data.1, bytes_data.0 as usize)};
        if let Ok(val)=std::str::from_utf8(bytes_array) {
            val
        } else {
            return err;
        }
    };
  
    let pool={
        let conn_opts=if let Ok(opts)=mysql::Opts::from_url(url_str) {
            opts
        } else {
            return err;
        };
        let pool=if let Ok(pool)=mysql::Pool::new(conn_opts) {
            pool
        } else {
            return err;
        };
        pool

    };
    Box::into_raw(Box::new(PoolResult::Success(pool)))
}

#[no_mangle]
pub extern "C" fn get_conn_from_pool(pool: *mut PoolResult)->*mut ConnResult {
    let err=Box::into_raw(Box::new(ConnResult::Error));
    let pool_result=unsafe {&*pool};
    match pool_result {
        &PoolResult::Error=>{return err;},
        &PoolResult::Success(ref pool)=>{
            let pool=pool.clone();
            if let Ok(conn)=pool.get_conn() {
                return Box::into_raw(Box::new(ConnResult::Success(conn)));
            } else {
                return err;
            }
        },
    }
}

#[no_mangle]
pub extern "C" fn connect_to_url_from_newpool(url_as_bytes: *const (c_int, *const u8))->*mut ConnResult {
    let pool_result=get_pool_from_url(url_as_bytes);
    let conn_result=get_conn_from_pool(pool_result);
    conn_result
}