#![allow(dead_code)]
use nom_locate::LocatedSpan;

pub type Ident<'a> = LocatedSpan<&'a [u8]>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ImportName<'a>(Ident<'a>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstantName<'a>(Ident<'a>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionName<'a>(Ident<'a>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParameterName<'a>(Ident<'a>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValueName<'a>(Ident<'a>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrimitiveTypes {
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
    String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructType<'a> {
    pub attr_name: Ident<'a>,
    pub attr_type: Type<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructTypes<'a> {
    pub name: Ident<'a>,
    pub types: Vec<StructType<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type<'a> {
    Primitive(PrimitiveTypes),
    Struct(StructTypes<'a>),
    Array(Box<Self>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConstantValue<'a>(Ident<'a>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Constant<'a> {
    name: ConstantName<'a>,
    constant_type: Type<'a>,
    constant_value: ConstantValue<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parameter<'a> {
    pub name: ParameterName<'a>,
    pub parameter_type: Type<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionStatement<'a> {
    pub name: FunctionName<'a>,
    pub parameter: Vec<Parameter<'a>>,
    pub result_type: Type<'a>,
    pub body: BodyStatement<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ValueState<'a> {
    pub name: ValueName<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BodyStatement<'a> {
    LetBinding(ValueState<'a>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MainStatement<'a> {
    Import(Vec<ImportName<'a>>),
    Constant(Constant<'a>),
    Function(FunctionName<'a>),
}

pub type Main<'a> = Vec<MainStatement<'a>>;