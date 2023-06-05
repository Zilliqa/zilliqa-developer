use std::clone::Clone;
use std::fmt;

pub trait BaseType {
    fn get_instance(&self) -> TypeAnnotation;
    fn to_string(&self) -> String;
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum TypeAnnotation {
    FunType(FunType),
    TypeVar(TypeVar),
    TemplateType(TemplateType),
    BuiltinType(BuiltinType),
    UnionType(UnionType),
    EnumType(EnumType),
}
/*
impl BaseType for TypeAnnotation {
  fn get_instance(&self) -> TypeAnnotation {
      self
  }

  fn to_string(&self) -> String {
      // TODO: Implement switch and get type of enum type
  }
}
*/

macro_rules! impl_base_type {
    ($type_name:ident) => {
        impl BaseType for $type_name {
            fn get_instance(&self) -> TypeAnnotation {
                TypeAnnotation::$type_name(self.clone())
            }

            fn to_string(&self) -> String {
                self.symbol.clone()
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct FunType {
    template_types: Vec<TypeAnnotation>,
    arg_types: Vec<TypeAnnotation>,
    to_type: Box<TypeAnnotation>,
    symbol: String,
}

impl_base_type!(FunType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct TypeVar {
    instance: Option<Box<TypeAnnotation>>,
    symbol: String,
}

impl_base_type!(TypeVar);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct TemplateType {
    name: String,
    symbol: String,
}

impl_base_type!(TemplateType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct BuiltinType {
    name: String,
    symbol: String,
}

impl_base_type!(BuiltinType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct UnionType {
    types: Vec<TypeAnnotation>,
    symbol: String,
}

impl_base_type!(UnionType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct EnumType {
    name: String,
    values: Vec<String>,
    symbol: String,
}

impl_base_type!(EnumType);
