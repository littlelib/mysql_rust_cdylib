# 1. Basic strategy
## 1. Handling rust enums
- Rust side
```rust
# Test::C is a byte representation of a UTF-8 string.
#[repr(C)]
pub enum Test {
    A,
    B(i32),
    C([u8])
}
#[no_mangle]
pub extern fn some_fn()->*mut Test {
    ...
}
```


- Julia Side
```julia
mutable struct Test
    tag::Cint,
    val::Ptr{Cvoid}
end

obj_ptr=@ccall "somelib".some_fn()::Ptr{Test}
obj=unsafe_load(obj_ptr)
if obj.tag==0
elseif obj.tag==1
    reinterpret(Ptr{Int32}, obj.val)|>unsafe_load
elseif obj.tag==2
    bytes_ptr=reinterpret(Ptr{UInt8}, obj.val)
    bytes=UInt[]
    while unsafe_load(bytes_ptr)!=0
        push!(bytes, unsafe_load(bytes_ptr))
        bytes_ptr=Ptr{Int32}(bytes_ptr+1)
    end
    String(bytes)
end
```

## 2. Communication between MySQL server & Julia
1. Julia gives url(bytes) to Rust
2. Rust connects to MySQL server
3. Julia gives parameters to rust
- via rust ffi function
    - @ccall "lib".execute(params)::Executeresults
4. Rust (sends to sql and) executes sql statements
5. Rust gives the info to julia


```julia
#=
mutable struct Value
    tag::Cint
    val_ptr::Ptr{Cvoid}
end

mutable struct ValueFFI
    val
    val_ffi::Value
end
=#

mutable struct ValueFFI
    tag::Cint
    val_ptr::Ptr{Cvoid}
end

function type_to_num(val)
    type=typeof(val)
    if type==Nothing
        return 0
    elseif type==Int64
        return 1
    elseif type==UInt64
        return 2
    elseif type==Float32
        return 3
    elseif type==Float64
        return 4
    elseif type==DateFFI
        return 5
    elseif type==TimeFFI
        return 6
    else
        error("Unsupported type: $(type)")
    end
end

#=
function val_to_objref(val)
    if isimmutable(val).
        return Ref(val)|>pointer_from_objref
    else
        return pointer_from_objref(val)
    end
end
#No, no, no. Absolutely not like this. This function will copy the val if immutable, and give the copied value's address-which will be gone after the function is finished!
=#

mutable struct Value
    val
    ref
end

Value(val)=begin
    obj=Value(val, nothing)
    obj.ref=Ref(obj.val)|>pointer_from_objref
    return obj
end

# But this does work. How about that!

# Keeps the value inside, to prevent gc of the value until the instance is out of scope.
ValueFFI(val)=begin
    x=Value(val)
    ValueFFI(type_to_num(val), x.ref)
end



```





# 2. Functions

connect
- via url
    - url: UTF-8 String into bytes

close
- connection
- statement

prepare
-

execute
- query
- statement

free?



