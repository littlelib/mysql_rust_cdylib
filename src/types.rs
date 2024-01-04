use std::ffi::{c_int};
use crate::Value;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ArrayStruct<T> {
    pub length: c_int,
    size: c_int,
    pub vals: *mut [Box<T>]
}

impl<T> From<Vec<T>> for ArrayStruct<T> {
    fn from(vec:Vec<T>) -> Self{
        let array_length=vec.len();
        let size = std::mem::size_of::<T>();
        let boxed_vec=vec.into_iter().map(|x| Box::new(x)).collect::<Vec<Box<T>>>();
        let pointer_to_vec=Box::into_raw(boxed_vec.into_boxed_slice());
        ArrayStruct{length: array_length as c_int, size: size as c_int, vals: pointer_to_vec}
    }
}

impl<T: Clone> From<&[T]> for ArrayStruct<T> {
    fn from(val:&[T]) -> Self {
        let array_length=val.len();
        let size=std::mem::size_of::<T>();
        let boxed_slice=(*val).into_iter().map(|x| Box::new((*x).clone())).collect::<Vec<Box<T>>>();
        let pointer_to_vec=Box::into_raw(boxed_slice.into_boxed_slice());
        ArrayStruct{length: array_length as c_int, size: size as c_int, vals: pointer_to_vec}
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ByteString {
    length: c_int,
    bytes: *mut [u8]
}

impl From<&str> for ByteString {
    fn from(string: &str)->Self {
        let length=string.len();
        let bytes=string.as_bytes() as *const [u8] as *mut [u8];
        ByteString{length: length as c_int, bytes: bytes.into()}
    }
}

impl From<String> for ByteString {
    fn from(string: String)->Self {
        let length=string.len();
        let bytes=string.as_bytes() as *const [u8] as *mut [u8];
        ByteString{length: length as c_int, bytes: bytes.into()}
    }
}

impl From<ByteString> for Vec<u8> {
    fn from(bstr: ByteString)->Self {
        let bytes=unsafe {std::slice::from_raw_parts(bstr.bytes as *mut u8, bstr.length as usize)};
        bytes.to_vec()
    }
}

impl TryFrom<&ByteString> for String {
    type Error=&'static str;
    fn try_from(byte_str: &ByteString)->Result<Self, Self::Error> {
        let u8_array=unsafe{std::slice::from_raw_parts(byte_str.bytes as *mut u8, byte_str.length as usize)};
        let string_result=std::str::from_utf8(u8_array);
        match string_result {
            Ok(val)=>Ok(val.to_string()),
            Err(_)=>Err("UTF8_CONVERSION_ERROR")
        }
    }
}


#[derive(Clone, Debug)]
#[repr(C)]
pub enum ValueFFI {
    NULL,
    Bytes(Box<ByteString>),
    Int(Box<i64>),
    UInt(Box<u64>),
    Float(Box<f32>),
    Double(Box<f64>),
    /// year, month, day, hour, minutes, seconds, micro seconds
    Date(Box<(u16, u8, u8, u8, u8, u8, u32)>),
    /// is negative, days, hours, minutes, seconds, micro seconds
    Time(Box<(i32, u32, u8, u8, u8, u32)>),
}

impl From<Value> for ValueFFI {
    fn from(value:Value)->Self {
        match value {
            Value::NULL=>ValueFFI::NULL,
            Value::Bytes(val)=>ValueFFI::Bytes(Box::new(ByteString{length: val.len() as c_int, bytes: Box::into_raw(val.into_boxed_slice())})),
            Value::Int(val)=>ValueFFI::Int(Box::new(val)),
            Value::UInt(val)=>ValueFFI::UInt(Box::new(val)),
            Value::Float(val)=>ValueFFI::Float(Box::new(val)),
            Value::Double(val)=>ValueFFI::Double(Box::new(val)),
            Value::Date(a,b,c,d,e,f,g)=>ValueFFI::Date(Box::new((a,b,c,d,e,f,g))),
            Value::Time(a,b,c,d,e,f)=>ValueFFI::Time(Box::new(({match a {false=>0, true=>1}},b,c,d,e,f))),
        }
    }
}

impl From<ValueFFI> for Value {
    fn from(value:ValueFFI)->Self {
        match value {
            ValueFFI::NULL=>Value::NULL,
            ValueFFI::Bytes(val)=>Value::Bytes((*val).clone().into()),
            ValueFFI::Int(val)=>Value::Int((*val).clone()),
            ValueFFI::UInt(val)=>Value::UInt((*val).clone()),
            ValueFFI::Float(val)=>Value::Float((*val).clone()),
            ValueFFI::Double(val)=>Value::Double((*val).clone()),
            ValueFFI::Date(val)=>{
                let (a,b,c,d,e,f,g)=(*val).clone();
                Value::Date(a,b,c,d,e,f,g)
            },
            ValueFFI::Time(val)=>{
                let (a,b,c,d,e,f)=(*val).clone();
                Value::Time(match a {0=>false, _=>true},b,c,d,e,f)
            },
        }
    }
}


pub fn into_ValueFFI(vals: Vec<Value>)->ArrayStruct<ValueFFI> {
    let mut return_vec=vals.into_iter().map(|x| x.into()).collect::<Vec<ValueFFI>>();
    return_vec.into()
}