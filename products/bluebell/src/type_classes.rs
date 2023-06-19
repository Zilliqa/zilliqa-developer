use std::clone::Clone;
use std::collections::HashMap;
use std::fmt;

pub trait Named {
    fn basename(&self) -> String;
    fn symbol_name(&self) -> String;
}
pub trait Namespace {
    fn get_namespace(&self) -> Option<String>;
}

pub trait Templatable {
    fn get_template_parameters(&self) -> Option<Vec<Box<dyn BaseType>>> {
        None
    }
    fn is_templated(&self) -> bool {
        false
    }
}

pub trait Callable {
    fn as_function(&self) -> Option<String>;
}
pub trait Comparable {
    fn equals(&self, other: &dyn BaseType) -> bool;
}
/* TODO: import error
pub trait Resolvable {
    fn resolve_symbols(&self) -> Result<(), Error>;
}
*/
pub trait Metadata {
    fn set_metadata(&mut self, metakey: String, metavalue: String);
    fn get_metadata(&self, metakey: String) -> Option<String>;
}
pub trait Typed {
    fn is_integral(&self) -> bool;
    fn is_floating_point(&self) -> bool;
    fn is_composite(&self) -> bool;
    fn is_pointer(&self) -> bool;
    fn is_const(&self) -> bool;
    fn get_element_type(&self) -> Option<Box<dyn BaseType>>;
    fn get_arg_types(&self) -> Vec<Box<dyn BaseType>>;
}

pub trait SizedType {
    fn size_of(&self) -> Option<usize>;
}
/*
TODO: Define Qualifier
pub trait Qualified {
    fn add_qualifier(&mut self, qualifier: Qualifier);
    fn has_qualifier(&self, qualifier: Qualifier) -> bool;
}
*/

pub trait BaseType {
    fn get_instance(&self) -> TypeAnnotation;
    fn to_string(&self) -> String;

    fn clone_boxed(&self) -> Box<dyn BaseType>;
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum TypeAnnotation {
    Void,
    FunType(FunType),
    TypeVar(TypeVar),
    TemplateType(TemplateType),
    BuiltinType(BuiltinType),
    NamespaceType(NamespaceType),
    UnionType(UnionType),
    EnumType(EnumType),
    StructType(StructType),
    MapType(MapType),
}

pub trait IsIndexable {
    fn type_of_key(&self, key: String) -> TypeAnnotation;
}

impl BaseType for TypeAnnotation {
    fn get_instance(&self) -> TypeAnnotation {
        match self {
            TypeAnnotation::Void => TypeAnnotation::Void,
            TypeAnnotation::FunType(fun_type) => fun_type.get_instance(),
            TypeAnnotation::TypeVar(type_var) => type_var.get_instance(),
            TypeAnnotation::TemplateType(template_type) => template_type.get_instance(),
            TypeAnnotation::BuiltinType(builtin_type) => builtin_type.get_instance(),
            TypeAnnotation::NamespaceType(namespace_type) => namespace_type.get_instance(),
            TypeAnnotation::UnionType(union_type) => union_type.get_instance(),
            TypeAnnotation::EnumType(enum_type) => enum_type.get_instance(),
            TypeAnnotation::StructType(struct_type) => struct_type.get_instance(),
            TypeAnnotation::MapType(map_type) => map_type.get_instance(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            TypeAnnotation::Void => "Void".to_string(),
            TypeAnnotation::FunType(fun_type) => fun_type.to_string(),
            TypeAnnotation::TypeVar(type_var) => type_var.to_string(),
            TypeAnnotation::TemplateType(template_type) => template_type.to_string(),
            TypeAnnotation::BuiltinType(builtin_type) => builtin_type.to_string(),
            TypeAnnotation::NamespaceType(namespace_type) => namespace_type.to_string(),
            TypeAnnotation::UnionType(union_type) => union_type.to_string(),
            TypeAnnotation::EnumType(enum_type) => enum_type.to_string(),
            TypeAnnotation::StructType(struct_type) => struct_type.to_string(),
            TypeAnnotation::MapType(map_type) => map_type.to_string(),
        }
    }

    fn clone_boxed(&self) -> Box<dyn BaseType> {
        match self {
            TypeAnnotation::Void => Box::new(TypeAnnotation::Void),
            TypeAnnotation::FunType(fun_type) => fun_type.clone_boxed(),
            TypeAnnotation::TypeVar(type_var) => type_var.clone_boxed(),
            TypeAnnotation::TemplateType(template_type) => template_type.clone_boxed(),
            TypeAnnotation::BuiltinType(builtin_type) => builtin_type.clone_boxed(),
            TypeAnnotation::NamespaceType(namespace_type) => namespace_type.clone_boxed(),
            TypeAnnotation::UnionType(union_type) => union_type.clone_boxed(),
            TypeAnnotation::EnumType(enum_type) => enum_type.clone_boxed(),
            TypeAnnotation::StructType(struct_type) => struct_type.clone_boxed(),
            TypeAnnotation::MapType(map_type) => map_type.clone_boxed(),
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

        impl Named for $type_name {
            fn basename(&self) -> String {
                self.symbol.clone()
            }
            fn symbol_name(&self) -> String {
                self.symbol.clone()
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
    // pub meta_data: HashMap<String, String>,
}

impl_base_type!(FunType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct TypeVar {
    pub instance: Option<Box<TypeAnnotation>>,
    pub symbol: String,
    // pub meta_data: HashMap<String, String>,
}

impl_base_type!(TypeVar);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct TemplateType {
    pub name: String,
    pub symbol: String,
    // pub meta_data: HashMap<String, String>,
}

impl_base_type!(TemplateType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct BuiltinType {
    pub name: String,
    pub symbol: String,
    // pub meta_data: HashMap<String, String>,
}

impl_base_type!(BuiltinType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct NamespaceType {
    pub name: String,
    pub symbol: String,
    // pub meta_data: HashMap<String, String>,
}

impl_base_type!(NamespaceType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct UnionType {
    pub name: String,
    pub types: Vec<TypeAnnotation>,
    pub symbol: String,
    // pub meta_data: HashMap<String, String>,
}

impl_base_type!(UnionType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, TypeAnnotation)>, // Fields in the struct
    pub symbol: String,
    // pub meta_data: HashMap<String, String>,
}
impl_base_type!(StructType);

pub struct MapType {
    pub name: String,
    // TODO: Implement
    pub key_type: Option<Box<dyn BaseType>>,
    pub value_type: Option<Box<dyn BaseType>>,
    pub symbol: String,
    // pub meta_data: HashMap<String, String>,
}

impl IsIndexable for MapType {
    fn type_of_key(&self, key: String) -> TypeAnnotation {
        TypeAnnotation::Void
    }
}

impl fmt::Debug for MapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapType")
            .field("name", &self.name)
            .field(
                "key_type",
                &self
                    .key_type
                    .as_ref()
                    .map_or("None".to_string(), |k| k.to_string()),
            )
            .field(
                "value_type",
                &self
                    .value_type
                    .as_ref()
                    .map_or("None".to_string(), |v| v.to_string()),
            )
            .field("symbol", &self.symbol)
            .finish()
    }
}

impl PartialEq for MapType {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && match (&self.key_type, &other.key_type) {
                (Some(a), Some(b)) => a.to_string() == b.to_string(), // TODO: Implement eq for BaseTyep
                (None, None) => true,
                _ => false,
            }
            && match (&self.value_type, &other.value_type) {
                (Some(a), Some(b)) => a.to_string() == b.to_string(), // TODO: Implement eq for BaseTyep
                (None, None) => true,
                _ => false,
            }
            && self.symbol == other.symbol
    }
}
impl Eq for MapType {}
impl PartialOrd for MapType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MapType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
impl Clone for MapType {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            key_type: match &self.key_type {
                Some(x) => Some(x.clone_boxed()),
                None => None,
            },
            value_type: match &self.value_type {
                Some(x) => Some(x.clone_boxed()),
                None => None,
            },
            symbol: self.symbol.clone(),
        }
    }
}

impl_base_type!(MapType);

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq)]
pub struct EnumType {
    pub name: String,
    pub values: Vec<String>,
    pub symbol: String,
    // pub meta_data: HashMap<String, String>,
}

impl_base_type!(EnumType);

impl Templatable for FunType {
    fn get_template_parameters(&self) -> Option<Vec<Box<dyn BaseType>>> {
        if self.template_types.is_empty() {
            None
        } else {
            Some(
                self.template_types
                    .iter()
                    .map(|x| Box::new(x.clone()) as Box<dyn BaseType>)
                    .collect(),
            )
        }
    }
    fn is_templated(&self) -> bool {
        !self.template_types.is_empty()
    }
}

impl Templatable for TemplateType {
    fn get_template_parameters(&self) -> Option<Vec<Box<dyn BaseType>>> {
        None
    }
    fn is_templated(&self) -> bool {
        false
    }
}

impl SizedType for FunType {
    fn size_of(&self) -> Option<usize> {
        // return None for now
        None
    }
}
impl SizedType for TypeVar {
    fn size_of(&self) -> Option<usize> {
        None
    }
}
impl SizedType for TemplateType {
    fn size_of(&self) -> Option<usize> {
        None
    }
}
impl SizedType for BuiltinType {
    fn size_of(&self) -> Option<usize> {
        None
    }
}
impl SizedType for UnionType {
    fn size_of(&self) -> Option<usize> {
        None
    }
}
impl SizedType for StructType {
    fn size_of(&self) -> Option<usize> {
        None
    }
}
impl SizedType for MapType {
    fn size_of(&self) -> Option<usize> {
        None
    }
}
impl SizedType for EnumType {
    fn size_of(&self) -> Option<usize> {
        None
    }
}
