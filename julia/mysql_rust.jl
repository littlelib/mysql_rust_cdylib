

using Dates

abstract type FFIType end

mutable struct Value <: FFIType
    tag::Cint
    val_ref::Ref
end

mutable struct ValueFFI <: FFIType
    tag::Cint
    val_ptr::Ptr{Cvoid}
end

mutable struct DateFFI <: FFIType
    y::UInt16
    m::UInt8
    d::UInt8
    h::UInt8
    mi::UInt8
    s::UInt8
    ms::UInt8
end

mutable struct TimeFFI <: FFIType
    negative::Bool
    d::UInt32
    h::UInt8
    mi::UInt8
    s::UInt8
    ms::UInt32
end

mutable struct MySQLTime
    negative::Bool
    d::UInt32
    h::UInt8
    mi::UInt8
    s::UInt8
    ms::UInt32
end


MySQLTime(x::Dates.Millisecond)=begin
    total_ms=x.value
    remainder=total_ms
    negative=false
    if total_ms<0
        remainder*=-1
        negative=true
    end
    vals=Int64[]
    for i in (86400000, 3600000, 60000, 1000)
        (share, remainder)=(div(remainder, i), remainder%i)
        push!(vals, share)
    end
    MySQLTime(negative, UInt32(vals[1]), UInt8(vals[2]), UInt8(vals[3]), UInt8(vals[4]), UInt32(remainder))
end

function type_to_num(val)
    type=typeof(val)
    if type==Nothing
        return 0
    elseif type==String
        return 1
    elseif type==Int64
        return 2
    elseif type==UInt64
        return 3
    elseif type==Float32
        return 4
    elseif type==Float64
        return 5
    elseif type==DateFFI
        return 6
    elseif type==TimeFFI
        return 7
    else
        error("Unsupported type: $(type)")
    end
end

Value(val)=begin
    tag=type_to_num(val)
    ref=Ref(val)
    Value(tag, ref)
end

DateFFI(date::DateTime)=begin
    functions=(Dates.year, Dates.month, Dates.day, Dates.hour, Dates.minute, Dates.second, Dates.millisecond)
    DateFFI((map(y->y(date), functions))...)
end

Dates.DateTime(x::DateFFI)=begin
    
end

mutable struct ByteString
    length::Cint
    vals::Ptr{UInt8}
end

Base.String(x::ByteString)=begin
    bytes=unsafe_wrap(Array, x.vals, x.length)
    String(bytes)
end

ByteString(x::String)=begin
    bytes=Vector{UInt8}(x)
    value=Value(bytes)
    ByteString(length(bytes),value.ref)
end

mutable struct ArrayStruct
    length::Cint
    size::Cint
    vals::Ptr{Cvoid}
end

cd(@__DIR__)
array_ptr=@ccall "../target/debug/libmysql_native".rust_to_julia()::Ptr{ArrayStruct}
array=unsafe_load(array_ptr)
println(array)
array_vals_ptr=Base.unsafe_convert(Ptr{Ptr{ValueFFI}}, array.vals)
vals=unsafe_wrap(Array, array_vals_ptr, array.length)|>
x->map(y->y|>unsafe_load, x)

