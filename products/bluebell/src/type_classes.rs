use std::clone::Clone;
use std::fmt;

pub trait BaseType {
    fn get_instance(&self) -> TypeAnnotation;
    fn to_string(&self) -> String;
    fn clone_boxed(&self) -> Box<dyn BaseType>;
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

impl BaseType for TypeAnnotation {
    fn get_instance(&self) -> TypeAnnotation {
        match self {
            TypeAnnotation::FunType(fun_type) => fun_type.get_instance(),
            TypeAnnotation::TypeVar(type_var) => type_var.get_instance(),
            TypeAnnotation::TemplateType(template_type) => template_type.get_instance(),
            TypeAnnotation::BuiltinType(builtin_type) => builtin_type.get_instance(),
            TypeAnnotation::UnionType(union_type) => union_type.get_instance(),
            TypeAnnotation::EnumType(enum_type) => enum_type.get_instance(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            TypeAnnotation::FunType(fun_type) => fun_type.to_string(),
            TypeAnnotation::TypeVar(type_var) => type_var.to_string(),
            TypeAnnotation::TemplateType(template_type) => template_type.to_string(),
            TypeAnnotation::BuiltinType(builtin_type) => builtin_type.to_string(),
            TypeAnnotation::UnionType(union_type) => union_type.to_string(),
            TypeAnnotation::EnumType(enum_type) => enum_type.to_string(),
        }
    }

    fn clone_boxed(&self) -> Box<dyn BaseType> {
        match self {
            TypeAnnotation::FunType(fun_type) => fun_type.clone_boxed(),
            TypeAnnotation::TypeVar(type_var) => type_var.clone_boxed(),
            TypeAnnotation::TemplateType(template_type) => template_type.clone_boxed(),
            TypeAnnotation::BuiltinType(builtin_type) => builtin_type.clone_boxed(),
            TypeAnnotation::UnionType(union_type) => union_type.clone_boxed(),
            TypeAnnotation::EnumType(enum_type) => enum_type.clone_boxed(),
        }
    }
}

macro_rules! impl_base_type {
    ($type_name:ident) => {
        impl BaseType for $type_name {
            fn get_instance(&self) -> TypeAnnotation {
                TypeAnnotation::$type_name(self.clone())
            }

            fn to_string(&self) -> String {
                self.symbol.clone()
            }

            fn clone_boxed(&self) -> Box<dyn BaseType> {
                Box::new(self.clone())
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct FunType {
    pub template_types: Vec<TypeAnnotation>,
    pub arg_types: Vec<TypeAnnotation>,
    pub to_type: Box<TypeAnnotation>,
    pub symbol: String,
}

impl_base_type!(FunType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct TypeVar {
    pub instance: Option<Box<TypeAnnotation>>,
    pub symbol: String,
}

impl_base_type!(TypeVar);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct TemplateType {
    pub name: String,
    pub symbol: String,
}

impl_base_type!(TemplateType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct BuiltinType {
    pub name: String,
    pub symbol: String,
}

impl_base_type!(BuiltinType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct UnionType {
    pub types: Vec<TypeAnnotation>,
    pub symbol: String,
}

impl_base_type!(UnionType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct EnumType {
    pub name: String,
    pub values: Vec<String>,
    pub symbol: String,
}

impl_base_type!(EnumType);
