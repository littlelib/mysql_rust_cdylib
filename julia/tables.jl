using Tables, DataFrames


mutable struct SomeRow <: Tables.AbstractRow
    vals::Vector{Any}
    names::Ref
end

mutable struct SomeTable
    names::Vector{Symbol}
    rows::Vector{SomeRow}
end


Tables.isrowtable(::Type{SomeTable})=true
Base.push!(x::SomeTable, y)=Base.push!(getfield(x, :rows), y)
Base.append!(x::SomeTable,y)=Base.append!(getfield(x, :rows), y)

Base.eltype(m::SomeTable)=SomeRow
Base.length(m::SomeTable)=Base.length(getfield(m, :rows))
Base.iterate(m::SomeTable, st=1)= st > length(m) ? nothing : (getfield(m, :rows)[st], st+1)


Tables.getcolumn(m::SomeRow, i::Int)=getfield(m, :vals)[i]
Tables.getcolumn(m::SomeRow, nm::Symbol)=begin
    names=getfield(getfield(m, :names)[], :names)
    i=findfirst(x->x==nm, names)
    getfield(m, :vals)[i]
end
Tables.columnnames(m::SomeRow)=getfield(m, :names)[]
