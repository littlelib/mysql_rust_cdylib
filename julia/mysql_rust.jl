using Dates

mutable struct Value
    val
    ref
end

mutable struct ValueFFI
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
    ms::UInt8
end

mutable struct TimeFFI
    negative::Bool
    d::UInt32
    h::UInt8
    mi::UInt8
    s::UInt8
    ms::UInt32
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

Value(val)=begin
    obj=Value(val, nothing)
    obj.ref=Ref(obj.val)|>pointer_from_objref
    return obj
end

ValueFFI(val)=begin
    x=Value(val)
    ValueFFI(type_to_num(val), x.ref)
end

DateFFI(date::DateTime)=begin
    functions=(Dates.year, Dates.month, Dates.day, Dates.hour, Dates.minute, Dates.second, Dates.millisecond)
    DateFFI((map(y->y(date), functions))...)
end