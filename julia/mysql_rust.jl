

using Dates

abstract type FFIReceived end
abstract type FFITransmitted end

mutable struct Value <: FFITransmitted
    tag::Cint
    val_ref::Ref
end

mutable struct ValueFFI <: FFIReceived
    tag::Cint
    val_ptr::Ptr{Cvoid}
end

mutable struct DateFFI 
    y::UInt16
    m::UInt8
    d::UInt8
    h::UInt8
    mi::UInt8
    s::UInt8
    ms::UInt32
end

mutable struct TimeFFI 
    negative::Int32
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

MySQLTime(x::TimeFFI)=begin
    MySQLTime(Bool(x.negative), x.d, x.h, x.mi, x.s, x.ms)
end

TimeFFI(x::MySQLTime)=begin
    TimeFFI(Int32(x.nagative), x.d, x.h, x.mi, x.s, x.ms)
end

function type_to_num(val)
    type=typeof(val)
    if type==Nothing || type==Missing
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
    elseif type==Dates.DateTime
        return 6
    elseif type==MySQLTime
        return 7
    else
        error("Unsupported type: $(type)")
    end
end


DateFFI(date::Dates.DateTime)=begin
    functions=(Dates.year, Dates.month, Dates.day, Dates.hour, Dates.minute, Dates.second, Dates.millisecond)
    DateFFI((map(y->y(date), functions))...)
end

Dates.DateTime(x::DateFFI)=begin
    Dates.DateTime(x.y, x.m, x.d, x.h, x.mi, x.s, x.ms)
end

mutable struct ByteStringFFI <: FFIReceived
    length::Cint
    vals::Ptr{UInt8}
end

mutable struct ByteString <: FFITransmitted
    length::Cint
    vals::Ref
end

Base.String(x::ByteStringFFI)=begin
    bytes=unsafe_wrap(Array, x.vals, x.length)
    String(bytes)
end

ByteString(x::String)=begin
    bytes=Vector{UInt8}(x)
    ByteString(length(bytes), Ref(bytes))
end

mutable struct ArrayStructFFI
    length::Cint
    size::Cint
    vals::Ptr{Cvoid}
end

mutable struct ArrayStruct
    length::Cint
    size::Cint
    vals::Ref
end
ArrayStruct(x::Vector)=begin
    ref=map(y->Value(y), x)|>Ref
    size=sizeof(Value)
    len=length(x)
    ArrayStruct(len, size, ref)
end

function cast(x::ValueFFI)
    if x.tag==0
        missing
    elseif x.tag==1
        reinterpret(Ptr{ByteStringFFI}, x.val_ptr)|>unsafe_load|>Base.String
    elseif x.tag==2
        reinterpret(Ptr{Int64}, x.val_ptr)|>unsafe_load
    elseif x.tag==3
        reinterpret(Ptr{UInt64}, x.val_ptr)|>unsafe_load
    elseif x.tag==4
        reinterpret(Ptr{Float32}, x.val_ptr)|>unsafe_load
    elseif x.tag==5
        reinterpret(Ptr{Float64}, x.val_ptr)|>unsafe_load
    elseif x.tag==6
        reinterpret(Ptr{DateFFI}, x.val_ptr)|>unsafe_load|>Dates.DateTime
    elseif x.tag==7
        reinterpret(Ptr{TimeFFI}, x.val_ptr)|>unsafe_load|>MySQLTime
    else
        error("Unsupported tag value")
    end
end

Value(x::Missing)=Value(type_to_num(x), Ref(nothing))
Value(x::String)=begin
    bytestring=ByteString(x)
    Value(type_to_num(x), Ref(bytestring))
end
Value(x::Union{Int64, UInt64, Float32, Float64})=Value(type_to_num(x), Ref(x))
Value(x::Dates.DateTime)=begin
    dateffi=DateFFI(x)
    Value(type_to_num(x), Ref(dateffi))
end
Value(x::MySQLTime)=begin
    timeffi=TimeFFI(x)
    Value(type_to_num(x), Ref(timeffi))
end



function cast(x::ArrayStructFFI)
    array_vals_ptr=reinterpret(Ptr{Ptr{ValueFFI}}, x.vals)
    unsafe_wrap(Array, array_vals_ptr, array.length)|>
    x->map(y->y|>unsafe_load, x)|>
    x->map(y->cast(y), x)
end


cd(@__DIR__)
array_ptr=@ccall "../target/debug/libmysql_native".rust_to_julia()::Ptr{ArrayStructFFI}
array=unsafe_load(array_ptr)
println(array)
array_vals_ptr=Base.unsafe_convert(Ptr{Ptr{ValueFFI}}, array.vals)
vals=unsafe_wrap(Array, array_vals_ptr, array.length)|>
x->map(y->y|>unsafe_load, x)

array=ArrayStruct([1,2,"asdfasf", missing, Dates.now()])
