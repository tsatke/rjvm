#![allow(dead_code)]

use crate::classfile::ConstantPoolInfo::{
    ClassInfo, DoubleInfo, DynamicInfo, FieldrefInfo, FloatInfo, IntegerInfo,
    InterfaceMethodrefInfo, InvokeDynamicInfo, LongInfo, MethodHandleInfo, MethodTypeInfo,
    MethodrefInfo, ModuleInfo, NameAndTypeInfo, PackageInfo, StringInfo, Utf8Info,
};
use crate::classfile::VerificationTypeInfo::{
    DoubleVariable, FloatVariable, IntegerVariable, LongVariable, NullVariable, ObjectVariable,
    TopVariable, UninitializedThisVariable, UninitializedVariable,
};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::io::Read;
use std::ops::Index;

pub mod flags;

#[derive(Eq, PartialEq, Debug)]
pub struct ConstantPool {
    items: Vec<ConstantPoolInfo>,
}

impl ConstantPool {
    pub fn from(items: Vec<ConstantPoolInfo>) -> Self {
        Self { items }
    }
}

impl Index<usize> for ConstantPool {
    type Output = ConstantPoolInfo;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl Index<u16> for ConstantPool {
    type Output = ConstantPoolInfo;

    fn index(&self, index: u16) -> &Self::Output {
        &self[index as usize]
    }
}

impl Index<i32> for ConstantPool {
    type Output = ConstantPoolInfo;

    fn index(&self, index: i32) -> &Self::Output {
        &self[index as usize]
    }
}

/// Specified by [`$4.1`]
///
/// [`$4.1`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1
#[derive(Eq, PartialEq, Debug)]
pub struct ClassFile {
    /// Always 0xCAFEBABE
    magic: u32,
    minor_version: u16,
    major_version: u16,
    cp_info: ConstantPool,
    access_flags: flags::ClassAccessFlags,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeInfo>,
}

/// Specified by [`$4.4-A`]
///
/// [`$4.4-A`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4-140
#[derive(TryFromPrimitive, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum ConstantPoolInfoTag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    Fieldref = 9,
    Methodref = 10,
    InterfaceMethodref = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

/// Specified by [`$5.4.3.5-A`]
///
/// [`$5.4.3.5-A`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-5.html#jvms-5.4.3.5-220
#[derive(TryFromPrimitive, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum ReferenceKind {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

/// Specified by [`$4.4`]
///
/// [`$4.4`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4
#[derive(Eq, PartialEq, Debug)]
pub enum ConstantPoolInfo {
    ClassInfo {
        name_index: u16,
    },
    FieldrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    MethodrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceMethodrefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    StringInfo {
        string_index: u16,
    },
    IntegerInfo {
        bytes: u32,
    },
    FloatInfo {
        bytes: u32,
    },
    LongInfo {
        high_bytes: u32,
        low_bytes: u32,
    },
    DoubleInfo {
        high_bytes: u32,
        low_bytes: u32,
    },
    NameAndTypeInfo {
        name_index: u16,
        descriptor_index: u16,
    },
    Utf8Info {
        length: u16,
        bytes: Vec<u8>,
    },
    MethodHandleInfo {
        reference_kind: ReferenceKind,
        reference_index: u16,
    },
    MethodTypeInfo {
        descriptor_index: u16,
    },
    DynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    ModuleInfo {
        name_index: u16,
    },
    PackageInfo {
        name_index: u16,
    },
}

/// Specified by [`$4.5`]
///
/// [`$4.5`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.5
#[derive(Eq, PartialEq, Debug)]
pub struct FieldInfo {
    access_flags: flags::FieldAccessFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

/// Specified by [`$4.6`]
///
/// [`$4.6`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.6
#[derive(Eq, PartialEq, Debug)]
pub struct MethodInfo {
    access_flags: flags::MethodAccessFlags,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

/// Specified by [`$4.7.4`]
///
/// [`$4.7.4`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.4
#[derive(Eq, PartialEq, Debug)]
pub enum StackMapFrame {
    Same {
        frame_type: u8,
    },
    SameLocals1StackItem {
        frame_type: u8,
        stack: VerificationTypeInfo,
    },
    SameLocals1StackItemExtended {
        frame_type: u8,
        offset_delta: u16,
        stack: VerificationTypeInfo,
    },
    Chop {
        frame_type: u8,
        offset_delta: u16,
    },
    SameExtended {
        frame_type: u8,
        offset_delta: u16,
    },
    Append {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
    },
    Full {
        frame_type: u8,
        offset_delta: u16,
        locals: Vec<VerificationTypeInfo>,
        stack: Vec<VerificationTypeInfo>,
    },
}

#[derive(Eq, PartialEq, Debug)]
pub enum VerificationTypeInfo {
    TopVariable { tag: u8 },
    IntegerVariable { tag: u8 },
    FloatVariable { tag: u8 },
    NullVariable { tag: u8 },
    UninitializedThisVariable { tag: u8 },
    ObjectVariable { tag: u8, cpool_index: u16 },
    UninitializedVariable { tag: u8, offset: u16 },
    LongVariable { tag: u8 },
    DoubleVariable { tag: u8 },
}

#[derive(Eq, PartialEq, Debug)]
pub struct InnerClass {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: flags::InnerClassAccessFlags,
}

#[derive(Eq, PartialEq, Debug)]
pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

#[derive(Eq, PartialEq, Debug)]
pub struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

#[derive(Eq, PartialEq, Debug)]
pub struct LocalVariableTypeTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Annotation {
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ElementValuePair {
    element_name_index: u16,
    value: ElementValue,
}

#[derive(Eq, PartialEq, Debug)]
pub enum ElementValue {
    ConstValueIndex(u16),
    EnumConstValue {
        type_name_index: u16,
        const_name_index: u16,
    },
    ClassInfoIndex(u16),
    AnnotationValue(Annotation),
    ArrayValue {
        values: Vec<ElementValue>,
    },
}

#[derive(Eq, PartialEq, Debug)]
pub struct TypeAnnotation {
    target_path: TypePath,
    target_info: TargetInfo,
    type_index: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum TargetInfo {
    TypeParameter {
        type_parameter_index: u8,
    },
    Supertype {
        supertype_index: u16,
    },
    TypeParameterBound {
        type_parameter_index: u8,
        bound_index: u8,
    },
    Empty,
    FormalParameter {
        formal_parameter_index: u8,
    },
    Throws {
        throws_type_index: u16,
    },
    Localvar {
        table: Vec<LocalVarTargetTableEntry>,
    },
    Catch {
        exception_table_index: u16,
    },
    Offset {
        offset: u16,
    },
    TypeArgument {
        offset: u16,
        type_argument_index: u8,
    },
}

#[derive(Eq, PartialEq, Debug)]
pub struct LocalVarTargetTableEntry {
    start_pc: u16,
    length: u16,
    index: u16,
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
#[derive(Eq, PartialEq, Debug)]
pub enum TypePathKind {
    DeepArray,
    DeepNested,
    WildcardTypeArgument,
    TypeArgument,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Path {
    type_path_kind: TypePathKind,
    type_path_argument_index: u8,
}

#[derive(Eq, PartialEq, Debug)]
pub struct TypePath {
    path: Vec<Path>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    bootstrap_arguments: Vec<u16>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct MethodParameter {
    name_index: u16,
    access_flags: flags::MethodParameterAccessFlags,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ModuleRequires {
    requires_index: u16,
    requires_flags: flags::RequiresFlags,
    requires_version_index: u16,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ModuleExports {
    exports_index: u16,
    exports_flags: flags::ExportsFlags,
    exports_to_index: Vec<u16>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ModuleOpens {
    opens_index: u16,
    opens_flags: flags::OpensFlags,
    opens_to_index: Vec<u16>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ModuleProvides {
    provides_index: u16,
    provides_with_index: Vec<u16>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct RecordComponentInfo {
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

/// Specified by [`$4.7`]
///
/// [`$4.7`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7
#[derive(Eq, PartialEq, Debug)]
pub enum AttributeInfo {
    ConstantValue {
        attribute_name_index: u16,
        attribute_length: u32,
        constantvalue_index: u16,
    },
    Code {
        attribute_name_index: u16,
        attribute_length: u32,
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<AttributeInfo>,
    },
    StackMapTable {
        attribute_name_index: u16,
        attribute_length: u32,
        entries: Vec<StackMapFrame>,
    },
    Exceptions {
        attribute_name_index: u16,
        attribute_length: u32,
        exception_index_table: Vec<u16>,
    },
    InnerClasses {
        attribute_name_index: u16,
        attribute_length: u32,
        classes: Vec<InnerClass>,
    },
    EnclosingMethod {
        attribute_name_index: u16,
        attribute_length: u32,
        class_index: u16,
        method_index: u16,
    },
    Synthetic {
        attribute_name_index: u16,
        attribute_length: u32,
    },
    Signature {
        attribute_name_index: u16,
        attribute_length: u32,
        signature_index: u16,
    },
    SourceFile {
        attribute_name_index: u16,
        attribute_length: u32,
        sourcefile_index: u16,
    },
    SourceDebugExtension {
        attribute_name_index: u16,
        attribute_length: u32,
        debug_extension: Vec<u8>,
    },
    LineNumberTable {
        attribute_name_index: u16,
        attribute_length: u32,
        line_number_table: Vec<LineNumberTableEntry>,
    },
    LocalVariableTable {
        attribute_name_index: u16,
        attribute_length: u32,
        local_variable_table: Vec<LocalVariableTableEntry>,
    },
    LocalVariableTypeTable {
        attribute_name_index: u16,
        attribute_length: u32,
        local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
    },
    Deprecated {
        attribute_name_index: u16,
        attribute_length: u32,
    },
    RuntimeVisibleAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        annotations: Vec<Annotation>,
    },
    RuntimeInvisibleAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        annotations: Vec<Annotation>,
    },
    RuntimeVisibleParameterAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        parameter_annotations: Vec<Vec<Annotation>>,
    },
    RuntimeInvisibleParameterAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        parameter_annotations: Vec<Vec<Annotation>>,
    },
    RuntimeVisibleTypeAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        annotations: Vec<TypeAnnotation>,
    },
    RuntimeInvisibleTypeAnnotations {
        attribute_name_index: u16,
        attribute_length: u32,
        annotations: Vec<TypeAnnotation>,
    },
    AnnotationDefault {
        attribute_name_index: u16,
        attribute_length: u32,
        default_value: ElementValue,
    },
    BootstrapMethods {
        attribute_name_index: u16,
        attribute_length: u32,
        bootstrap_methods: Vec<BootstrapMethod>,
    },
    MethodParameters {
        attribute_name_index: u16,
        attribute_length: u32,
        parameters: Vec<MethodParameter>,
    },
    Module {
        attribute_name_index: u16,
        attribute_length: u32,
        module_name_index: u16,
        module_flags: flags::ModuleFlags,
        module_version_index: u16,
        requires: Vec<ModuleRequires>,
        exports: Vec<ModuleExports>,
        opens: Vec<ModuleOpens>,
        uses_index: Vec<u16>,
        provides: Vec<ModuleProvides>,
    },
    ModulePackages {
        attribute_name_index: u16,
        attribute_length: u32,
        package_index: Vec<u16>,
    },
    ModuleMainClass {
        attribute_name_index: u16,
        attribute_length: u32,
        main_class_index: u16,
    },
    NestHost {
        attribute_name_index: u16,
        attribute_length: u32,
        host_class_index: u16,
    },
    NestMembers {
        attribute_name_index: u16,
        attribute_length: u32,
        classes: Vec<u16>,
    },
    Record {
        attribute_name_index: u16,
        attribute_length: u32,
        components: Vec<RecordComponentInfo>,
    },
    PermittedSubclasses {
        attribute_name_index: u16,
        attribute_length: u32,
        classes: Vec<u16>,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub enum ClassFileParseError {
    InvalidTypePathKind,
    InvalidTypeAnnotationTargetType,
    InvalidElementValueTag,
    InvalidAttributeNameIndex,
    InvalidMagicValue,
    InvalidConstantPoolInfoTag,
    InvalidVerificationTypeTag,
    InvalidReferenceKind,
    UnexpectedEOF,
}

macro_rules! read_bytes {
    ($source:expr, $count:expr) => {{
        let mut buf = [0_u8; $count];
        $source
            .read_exact(&mut buf)
            .or(Err(ClassFileParseError::UnexpectedEOF))?;
        buf
    }};
}

macro_rules! read_u8 {
    ($source:expr) => {{
        u8::from_be_bytes(read_bytes!($source, 1))
    }};
}

macro_rules! read_u16 {
    ($source:expr) => {{
        u16::from_be_bytes(read_bytes!($source, 2))
    }};
}

macro_rules! read_u32 {
    ($source:expr) => {{
        u32::from_be_bytes(read_bytes!($source, 4))
    }};
}

impl ClassFile {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let magic = read_u32!(source);
        if magic != 0xCAFEBABE {
            return Err(ClassFileParseError::InvalidMagicValue);
        }

        let minor_version = read_u16!(source);
        let major_version = read_u16!(source);

        let constant_pool_count = read_u16!(source);
        let mut cp_info: Vec<ConstantPoolInfo> =
            Vec::with_capacity((constant_pool_count - 1) as usize);
        for _ in 0..constant_pool_count - 1 {
            cp_info.push(ConstantPoolInfo::parse(source)?);
        }
        let cp = ConstantPool::from(cp_info);

        let access_flags = flags::ClassAccessFlags::from_bits_truncate(read_u16!(source));
        let this_class = read_u16!(source);
        let super_class = read_u16!(source);

        let interfaces_count = read_u16!(source);
        let mut interfaces: Vec<u16> = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(read_u16!(source));
        }

        let fields_count = read_u16!(source);
        let mut fields: Vec<FieldInfo> = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(FieldInfo::parse(&cp, source)?);
        }

        let methods_count = read_u16!(source);
        let mut methods: Vec<MethodInfo> = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(MethodInfo::parse(&cp, source)?);
        }

        let attributes_count = read_u16!(source);
        let mut attributes: Vec<AttributeInfo> = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::parse(&cp, source)?);
        }

        Ok(Self {
            magic,
            minor_version,
            major_version,
            cp_info: cp,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}

impl ConstantPoolInfo {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let tag_byte = read_u8!(source);
        let tag = ConstantPoolInfoTag::try_from(tag_byte)
            .or(Err(ClassFileParseError::InvalidConstantPoolInfoTag))?;

        match tag {
            ConstantPoolInfoTag::Utf8 => ConstantPoolInfo::parse_utf8(source),
            ConstantPoolInfoTag::Integer => ConstantPoolInfo::parse_integer(source),
            ConstantPoolInfoTag::Float => ConstantPoolInfo::parse_float(source),
            ConstantPoolInfoTag::Long => ConstantPoolInfo::parse_long(source),
            ConstantPoolInfoTag::Double => ConstantPoolInfo::parse_double(source),
            ConstantPoolInfoTag::Class => ConstantPoolInfo::parse_class(source),
            ConstantPoolInfoTag::String => ConstantPoolInfo::parse_string(source),
            ConstantPoolInfoTag::Fieldref => ConstantPoolInfo::parse_fieldref(source),
            ConstantPoolInfoTag::Methodref => ConstantPoolInfo::parse_methodref(source),
            ConstantPoolInfoTag::InterfaceMethodref => {
                ConstantPoolInfo::parse_interface_methodref(source)
            }
            ConstantPoolInfoTag::NameAndType => ConstantPoolInfo::parse_name_and_type(source),
            ConstantPoolInfoTag::MethodHandle => ConstantPoolInfo::parse_method_handle(source),
            ConstantPoolInfoTag::MethodType => ConstantPoolInfo::parse_method_type(source),
            ConstantPoolInfoTag::Dynamic => ConstantPoolInfo::parse_dynamic(source),
            ConstantPoolInfoTag::InvokeDynamic => ConstantPoolInfo::parse_invoke_dynamic(source),
            ConstantPoolInfoTag::Module => ConstantPoolInfo::parse_module(source),
            ConstantPoolInfoTag::Package => ConstantPoolInfo::parse_package(source),
        }
    }

    fn parse_utf8(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let length = read_u16!(source);
        let mut bytes = vec![0_u8; length as usize];
        source
            .read_exact(&mut bytes)
            .or(Err(ClassFileParseError::UnexpectedEOF))?;

        Ok(Utf8Info { length, bytes })
    }

    fn parse_integer(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        Ok(IntegerInfo {
            bytes: read_u32!(source),
        })
    }

    fn parse_float(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        Ok(FloatInfo {
            bytes: read_u32!(source),
        })
    }

    fn parse_long(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let high_bytes = read_u32!(source);
        let low_bytes = read_u32!(source);
        Ok(LongInfo {
            high_bytes,
            low_bytes,
        })
    }

    fn parse_double(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let high_bytes = read_u32!(source);
        let low_bytes = read_u32!(source);
        Ok(DoubleInfo {
            high_bytes,
            low_bytes,
        })
    }

    fn parse_class(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        Ok(ClassInfo {
            name_index: read_u16!(source),
        })
    }

    fn parse_string(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        Ok(StringInfo {
            string_index: read_u16!(source),
        })
    }

    fn parse_fieldref(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let class_index = read_u16!(source);
        let name_and_type_index = read_u16!(source);
        Ok(FieldrefInfo {
            class_index,
            name_and_type_index,
        })
    }

    fn parse_methodref(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let class_index = read_u16!(source);
        let name_and_type_index = read_u16!(source);
        Ok(MethodrefInfo {
            class_index,
            name_and_type_index,
        })
    }

    fn parse_interface_methodref(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let class_index = read_u16!(source);
        let name_and_type_index = read_u16!(source);
        Ok(InterfaceMethodrefInfo {
            class_index,
            name_and_type_index,
        })
    }

    fn parse_name_and_type(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let name_index = read_u16!(source);
        let descriptor_index = read_u16!(source);
        Ok(NameAndTypeInfo {
            name_index,
            descriptor_index,
        })
    }

    fn parse_method_handle(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let reference_kind = ReferenceKind::try_from(read_u8!(source))
            .or(Err(ClassFileParseError::InvalidReferenceKind))?;
        let reference_index = read_u16!(source);

        Ok(MethodHandleInfo {
            reference_kind,
            reference_index,
        })
    }

    fn parse_method_type(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        Ok(MethodTypeInfo {
            descriptor_index: read_u16!(source),
        })
    }

    fn parse_dynamic(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let bootstrap_method_attr_index = read_u16!(source);
        let name_and_type_index = read_u16!(source);
        Ok(DynamicInfo {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }

    fn parse_invoke_dynamic(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let bootstrap_method_attr_index = read_u16!(source);
        let name_and_type_index = read_u16!(source);
        Ok(InvokeDynamicInfo {
            bootstrap_method_attr_index,
            name_and_type_index,
        })
    }

    fn parse_module(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        Ok(ModuleInfo {
            name_index: read_u16!(source),
        })
    }

    fn parse_package(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        Ok(PackageInfo {
            name_index: read_u16!(source),
        })
    }
}

impl FieldInfo {
    pub fn parse(cp: &ConstantPool, source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let access_flags = flags::FieldAccessFlags::from_bits_truncate(read_u16!(source));
        let name_index = read_u16!(source);
        let descriptor_index = read_u16!(source);
        let attributes_count = read_u16!(source);
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::parse(cp, source)?);
        }
        Ok(FieldInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

impl MethodInfo {
    pub fn parse(cp: &ConstantPool, source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let access_flags_bytes = read_u16!(source);
        let access_flags = flags::MethodAccessFlags::from_bits_truncate(access_flags_bytes);
        let name_index = read_u16!(source);
        let descriptor_index = read_u16!(source);
        let attributes_count = read_u16!(source);
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::parse(cp, source)?);
        }
        Ok(MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

impl AttributeInfo {
    pub fn parse(cp: &ConstantPool, source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let attribute_name_index = read_u16!(source);
        let attribute_length = read_u32!(source);

        match &cp[(attribute_name_index - 1) as usize] {
            ConstantPoolInfo::Utf8Info { length: _, bytes } => Ok(match bytes.as_slice() {
                b"ConstantValue" => {
                    assert_eq!(2, attribute_length);
                    Self::ConstantValue {
                        attribute_name_index,
                        attribute_length,
                        constantvalue_index: read_u16!(source),
                    }
                }
                b"Code" => {
                    let max_stack = read_u16!(source);
                    let max_locals = read_u16!(source);
                    let code_length = read_u32!(source);
                    let mut code = vec![0_u8; code_length as usize];
                    source
                        .read_exact(&mut code)
                        .or(Err(ClassFileParseError::UnexpectedEOF))?;
                    let exception_table_length = read_u16!(source);
                    let mut exception_table: Vec<ExceptionTableEntry> =
                        Vec::with_capacity(exception_table_length as usize);
                    for _ in 0..exception_table_length {
                        exception_table.push(ExceptionTableEntry::parse(source)?);
                    }
                    let attributes_count = read_u16!(source);
                    let mut attributes = Vec::with_capacity(attributes_count as usize);
                    for _ in 0..attributes_count {
                        attributes.push(AttributeInfo::parse(cp, source)?);
                    }

                    Self::Code {
                        attribute_name_index,
                        attribute_length,
                        max_stack,
                        max_locals,
                        code_length,
                        code,
                        exception_table,
                        attributes,
                    }
                }
                b"StackMapTable" => {
                    let number_of_entries = read_u16!(source);
                    let mut entries = Vec::with_capacity(number_of_entries as usize);
                    for _ in 0..number_of_entries {
                        entries.push(StackMapFrame::parse(source)?);
                    }
                    Self::StackMapTable {
                        attribute_name_index,
                        attribute_length,
                        entries,
                    }
                }
                b"Exceptions" => {
                    let number_of_exceptions = read_u16!(source);
                    let mut exception_index_table =
                        Vec::with_capacity(number_of_exceptions as usize);
                    for _ in 0..number_of_exceptions {
                        exception_index_table.push(read_u16!(source));
                    }
                    Self::Exceptions {
                        attribute_name_index,
                        attribute_length,
                        exception_index_table,
                    }
                }
                b"InnerClasses" => {
                    let number_of_classes = read_u16!(source);
                    let mut classes = Vec::with_capacity(number_of_classes as usize);
                    for _ in 0..number_of_classes {
                        classes.push(InnerClass::parse(source)?);
                    }

                    Self::InnerClasses {
                        attribute_name_index,
                        attribute_length,
                        classes,
                    }
                }
                b"EnclosingMethod" => {
                    let class_index = read_u16!(source);
                    let method_index = read_u16!(source);
                    Self::EnclosingMethod {
                        attribute_name_index,
                        attribute_length,
                        class_index,
                        method_index,
                    }
                }
                b"Synthetic" => Self::Synthetic {
                    attribute_name_index,
                    attribute_length,
                },
                b"Signature" => Self::Signature {
                    attribute_name_index,
                    attribute_length,
                    signature_index: read_u16!(source),
                },
                b"SourceFile" => Self::SourceFile {
                    attribute_name_index,
                    attribute_length,
                    sourcefile_index: read_u16!(source),
                },
                b"SourceDebugExtension" => {
                    let mut debug_extension = vec![0_u8; attribute_length as usize];
                    source
                        .read_exact(&mut debug_extension)
                        .or(Err(ClassFileParseError::UnexpectedEOF))?;
                    Self::SourceDebugExtension {
                        attribute_name_index,
                        attribute_length,
                        debug_extension,
                    }
                }
                b"LineNumberTable" => {
                    let line_number_table_length = read_u16!(source);
                    let mut line_number_table =
                        Vec::with_capacity(line_number_table_length as usize);
                    for _ in 0..line_number_table_length {
                        line_number_table.push(LineNumberTableEntry::parse(source)?);
                    }
                    Self::LineNumberTable {
                        attribute_name_index,
                        attribute_length,
                        line_number_table,
                    }
                }
                b"LocalVariableTable" => {
                    let local_variable_table_length = read_u16!(source);
                    let mut local_variable_table =
                        Vec::with_capacity(local_variable_table_length as usize);
                    for _ in 0..local_variable_table_length {
                        local_variable_table.push(LocalVariableTableEntry::parse(source)?);
                    }
                    Self::LocalVariableTable {
                        attribute_name_index,
                        attribute_length,
                        local_variable_table,
                    }
                }
                b"LocalVariableTypeTable" => {
                    let local_variable_table_length = read_u16!(source);
                    let mut local_variable_type_table =
                        Vec::with_capacity(local_variable_table_length as usize);
                    for _ in 0..local_variable_table_length {
                        local_variable_type_table.push(LocalVariableTypeTableEntry::parse(source)?);
                    }
                    Self::LocalVariableTypeTable {
                        attribute_name_index,
                        attribute_length,
                        local_variable_type_table,
                    }
                }
                b"Deprecated" => Self::Deprecated {
                    attribute_name_index,
                    attribute_length,
                },
                b"RuntimeVisibleAnnotations" => {
                    let num_annotations = read_u16!(source);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(Annotation::parse(source)?);
                    }
                    Self::RuntimeVisibleAnnotations {
                        attribute_name_index,
                        attribute_length,
                        annotations,
                    }
                }
                b"RuntimeInvisibleAnnotations" => {
                    let num_annotations = read_u16!(source);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(Annotation::parse(source)?);
                    }
                    Self::RuntimeInvisibleAnnotations {
                        attribute_name_index,
                        attribute_length,
                        annotations,
                    }
                }
                b"RuntimeVisibleParameterAnnotations" => {
                    let num_parameters = read_u8!(source);
                    let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                    for _ in 0..num_parameters {
                        let num_annotations = read_u16!(source);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        for _ in 0..num_annotations {
                            annotations.push(Annotation::parse(source)?);
                        }
                        parameter_annotations.push(annotations);
                    }
                    Self::RuntimeVisibleParameterAnnotations {
                        attribute_name_index,
                        attribute_length,
                        parameter_annotations,
                    }
                }
                b"RuntimeInvisibleParameterAnnotations" => {
                    let num_parameters = read_u8!(source);
                    let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);
                    for _ in 0..num_parameters {
                        let num_annotations = read_u16!(source);
                        let mut annotations = Vec::with_capacity(num_annotations as usize);
                        for _ in 0..num_annotations {
                            annotations.push(Annotation::parse(source)?);
                        }
                        parameter_annotations.push(annotations);
                    }
                    Self::RuntimeInvisibleParameterAnnotations {
                        attribute_name_index,
                        attribute_length,
                        parameter_annotations,
                    }
                }
                b"RuntimeVisibleTypeAnnotations" => {
                    let num_annotations = read_u16!(source);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(TypeAnnotation::parse(source)?);
                    }

                    Self::RuntimeVisibleTypeAnnotations {
                        attribute_name_index,
                        attribute_length,
                        annotations,
                    }
                }
                b"RuntimeInvisibleTypeAnnotations" => {
                    let num_annotations = read_u16!(source);
                    let mut annotations = Vec::with_capacity(num_annotations as usize);
                    for _ in 0..num_annotations {
                        annotations.push(TypeAnnotation::parse(source)?);
                    }

                    Self::RuntimeInvisibleTypeAnnotations {
                        attribute_name_index,
                        attribute_length,
                        annotations,
                    }
                }
                b"AnnotationDefault" => Self::AnnotationDefault {
                    attribute_name_index,
                    attribute_length,
                    default_value: ElementValue::parse(source)?,
                },
                b"BootstrapMethods" => {
                    let num_bootstrap_methods = read_u16!(source);
                    let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);
                    for _ in 0..num_bootstrap_methods {
                        bootstrap_methods.push(BootstrapMethod::parse(source)?);
                    }

                    Self::BootstrapMethods {
                        attribute_name_index,
                        attribute_length,
                        bootstrap_methods,
                    }
                }
                b"MethodParameters" => {
                    let attribute_name_index = read_u16!(source);
                    let attribute_length = read_u32!(source);
                    let parameters_count = read_u8!(source);
                    let mut parameters = Vec::with_capacity(parameters_count as usize);
                    for _ in 0..parameters_count {
                        parameters.push(MethodParameter::parse(source)?);
                    }
                    Self::MethodParameters {
                        attribute_name_index,
                        attribute_length,
                        parameters,
                    }
                }
                b"Module" => {
                    let attribute_name_index = read_u16!(source);
                    let attribute_length = read_u32!(source);

                    let module_name_index = read_u16!(source);
                    let module_flags = flags::ModuleFlags::from_bits_truncate(read_u16!(source));
                    let module_version_index = read_u16!(source);

                    let requires_count = read_u16!(source);
                    let mut requires = Vec::with_capacity(requires_count as usize);
                    for _ in 0..requires_count {
                        requires.push(ModuleRequires::parse(source)?);
                    }

                    let exports_count = read_u16!(source);
                    let mut exports = Vec::with_capacity(requires_count as usize);
                    for _ in 0..exports_count {
                        exports.push(ModuleExports::parse(source)?);
                    }

                    let opens_count = read_u16!(source);
                    let mut opens = Vec::with_capacity(requires_count as usize);
                    for _ in 0..opens_count {
                        opens.push(ModuleOpens::parse(source)?);
                    }

                    let uses_count = read_u16!(source);
                    let mut uses_index = Vec::with_capacity(requires_count as usize);
                    for _ in 0..uses_count {
                        uses_index.push(read_u16!(source));
                    }

                    let provides_count = read_u16!(source);
                    let mut provides = Vec::with_capacity(requires_count as usize);
                    for _ in 0..provides_count {
                        provides.push(ModuleProvides::parse(source)?);
                    }

                    Self::Module {
                        attribute_name_index,
                        attribute_length,
                        module_name_index,
                        module_flags,
                        module_version_index,
                        requires,
                        exports,
                        opens,
                        uses_index,
                        provides,
                    }
                }
                b"ModulePackages" => {
                    let package_count = read_u16!(source);
                    let mut package_index = Vec::with_capacity(package_count as usize);
                    for _ in 0..package_count {
                        package_index.push(read_u16!(source));
                    }

                    Self::ModulePackages {
                        attribute_name_index,
                        attribute_length,
                        package_index,
                    }
                }
                b"ModuleMainClass" => Self::ModuleMainClass {
                    attribute_name_index,
                    attribute_length,
                    main_class_index: read_u16!(source),
                },
                b"NestHost" => Self::NestHost {
                    attribute_name_index,
                    attribute_length,
                    host_class_index: read_u16!(source),
                },
                b"NestMembers" => {
                    let number_of_classes = read_u16!(source);
                    let mut classes = Vec::with_capacity(number_of_classes as usize);
                    for _ in 0..number_of_classes {
                        classes.push(read_u16!(source));
                    }
                    Self::NestMembers {
                        attribute_name_index,
                        attribute_length,
                        classes,
                    }
                }
                b"Record" => {
                    let components_count = read_u16!(source);
                    let mut components = Vec::with_capacity(components_count as usize);
                    for _ in 0..components_count {
                        components.push(RecordComponentInfo::parse(cp, source)?);
                    }

                    Self::Record {
                        attribute_name_index,
                        attribute_length,
                        components,
                    }
                }
                b"PermittedSubclasses" => {
                    let number_of_classes = read_u16!(source);
                    let mut classes = Vec::with_capacity(number_of_classes as usize);
                    for _ in 0..number_of_classes {
                        classes.push(read_u16!(source));
                    }
                    Self::PermittedSubclasses {
                        attribute_name_index,
                        attribute_length,
                        classes,
                    }
                }
                _ => {
                    let mut info = vec![0_u8; attribute_length as usize];
                    source
                        .read_exact(&mut info)
                        .or(Err(ClassFileParseError::UnexpectedEOF))?;

                    todo!("must be ignored silently")
                }
            }),
            _ => Err(ClassFileParseError::InvalidAttributeNameIndex),
        }
    }
}

impl RecordComponentInfo {
    pub fn parse(cp: &ConstantPool, source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let name_index = read_u16!(source);
        let descriptor_index = read_u16!(source);
        let attributes_count = read_u16!(source);
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::parse(cp, source)?);
        }
        Ok(Self {
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

impl ModuleRequires {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let requires_index = read_u16!(source);
        let requires_flags = flags::RequiresFlags::from_bits_truncate(read_u16!(source));
        let requires_version_index = read_u16!(source);

        Ok(Self {
            requires_index,
            requires_flags,
            requires_version_index,
        })
    }
}

impl ModuleExports {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let exports_index = read_u16!(source);
        let exports_flags = flags::ExportsFlags::from_bits_truncate(read_u16!(source));
        let exports_to_count = read_u16!(source);
        let mut exports_to_index = Vec::with_capacity(exports_to_count as usize);
        for _ in 0..exports_to_count {
            exports_to_index.push(read_u16!(source));
        }

        Ok(Self {
            exports_index,
            exports_flags,
            exports_to_index,
        })
    }
}

impl ModuleOpens {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let opens_index = read_u16!(source);
        let opens_flags = flags::OpensFlags::from_bits_truncate(read_u16!(source));
        let opens_to_count = read_u16!(source);
        let mut opens_to_index = Vec::with_capacity(opens_to_count as usize);
        for _ in 0..opens_to_count {
            opens_to_index.push(read_u16!(source));
        }

        Ok(Self {
            opens_index,
            opens_flags,
            opens_to_index,
        })
    }
}

impl ModuleProvides {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let provides_index = read_u16!(source);
        let provides_with_count = read_u16!(source);
        let mut provides_with_index = Vec::with_capacity(provides_with_count as usize);
        for _ in 0..provides_with_count {
            provides_with_index.push(read_u16!(source));
        }

        Ok(Self {
            provides_index,
            provides_with_index,
        })
    }
}

impl MethodParameter {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let name_index = read_u16!(source);
        let access_flags = flags::MethodParameterAccessFlags::from_bits_truncate(read_u16!(source));
        Ok(Self {
            name_index,
            access_flags,
        })
    }
}

impl BootstrapMethod {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let bootstrap_method_ref = read_u16!(source);
        let num_bootstrap_arguments = read_u16!(source);
        let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
        for _ in 0..num_bootstrap_arguments {
            bootstrap_arguments.push(read_u16!(source));
        }
        Ok(Self {
            bootstrap_method_ref,
            bootstrap_arguments,
        })
    }
}

impl TypeAnnotation {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let target_type = read_u8!(source);
        let target_info = match target_type {
            0x00 | 0x01 => TargetInfo::TypeParameter {
                type_parameter_index: read_u8!(source),
            },
            0x10 => TargetInfo::Supertype {
                supertype_index: read_u16!(source),
            },
            0x11 | 0x12 => {
                let type_parameter_index = read_u8!(source);
                let bound_index = read_u8!(source);
                TargetInfo::TypeParameterBound {
                    type_parameter_index,
                    bound_index,
                }
            }
            0x13..=0x15 => TargetInfo::Empty,
            0x16 => TargetInfo::FormalParameter {
                formal_parameter_index: read_u8!(source),
            },
            0x17 => TargetInfo::Throws {
                throws_type_index: read_u16!(source),
            },
            // ===
            0x40 | 0x41 => {
                let table_length = read_u16!(source);
                let mut table = Vec::with_capacity(table_length as usize);
                for _ in 0..table_length {
                    table.push(LocalVarTargetTableEntry::parse(source)?);
                }
                TargetInfo::Localvar { table }
            }
            0x42 => TargetInfo::Catch {
                exception_table_index: read_u16!(source),
            },
            0x43..=0x46 => TargetInfo::Offset {
                offset: read_u16!(source),
            },
            0x47..=0x4B => {
                let offset = read_u16!(source);
                let type_argument_index = read_u8!(source);
                TargetInfo::TypeArgument {
                    offset,
                    type_argument_index,
                }
            }
            _ => return Err(ClassFileParseError::InvalidTypeAnnotationTargetType),
        };
        let target_path = TypePath::parse(source)?;
        let type_index = read_u16!(source);
        let num_element_value_pairs = read_u16!(source);
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(ElementValuePair::parse(source)?);
        }
        Ok(Self {
            target_path,
            target_info,
            type_index,
            element_value_pairs,
        })
    }
}

impl TypePath {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let path_length = read_u8!(source);
        let mut path = Vec::with_capacity(path_length as usize);
        for _ in 0..path_length {
            path.push(Path::parse(source)?);
        }

        Ok(Self { path })
    }
}

impl Path {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let type_path_kind = TypePathKind::try_from(read_u8!(source))
            .or(Err(ClassFileParseError::InvalidTypePathKind))?;
        let type_path_argument_index = read_u8!(source);

        Ok(Self {
            type_path_kind,
            type_path_argument_index,
        })
    }
}

impl LocalVarTargetTableEntry {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let start_pc = read_u16!(source);
        let length = read_u16!(source);
        let index = read_u16!(source);

        Ok(Self {
            start_pc,
            length,
            index,
        })
    }
}

impl Annotation {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let type_index = read_u16!(source);
        let num_element_value_pairs = read_u16!(source);
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);
        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(ElementValuePair::parse(source)?);
        }
        Ok(Self {
            type_index,
            element_value_pairs,
        })
    }
}

impl ElementValuePair {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let element_name_index = read_u16!(source);
        let value = ElementValue::parse(source)?;
        Ok(Self {
            element_name_index,
            value,
        })
    }
}

impl ElementValue {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let tag = read_u8!(source) as char;
        match tag {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                Ok(Self::ConstValueIndex(read_u16!(source)))
            }
            'e' => {
                let type_name_index = read_u16!(source);
                let const_name_index = read_u16!(source);
                Ok(Self::EnumConstValue {
                    type_name_index,
                    const_name_index,
                })
            }
            'c' => Ok(Self::ClassInfoIndex(read_u16!(source))),
            '@' => Ok(Self::AnnotationValue(Annotation::parse(source)?)),
            '[' => {
                let num_values = read_u16!(source);
                let mut values = Vec::with_capacity(num_values as usize);
                for _ in 0..num_values {
                    values.push(ElementValue::parse(source)?);
                }
                Ok(Self::ArrayValue { values })
            }
            _ => Err(ClassFileParseError::InvalidElementValueTag),
        }
    }
}

impl LocalVariableTableEntry {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let start_pc = read_u16!(source);
        let length = read_u16!(source);
        let name_index = read_u16!(source);
        let descriptor_index = read_u16!(source);
        let index = read_u16!(source);
        Ok(Self {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        })
    }
}

impl LocalVariableTypeTableEntry {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let start_pc = read_u16!(source);
        let length = read_u16!(source);
        let name_index = read_u16!(source);
        let signature_index = read_u16!(source);
        let index = read_u16!(source);
        Ok(Self {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        })
    }
}

impl LineNumberTableEntry {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let start_pc = read_u16!(source);
        let line_number = read_u16!(source);
        Ok(Self {
            start_pc,
            line_number,
        })
    }
}

impl InnerClass {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let inner_class_info_index = read_u16!(source);
        let outer_class_info_index = read_u16!(source);
        let inner_name_index = read_u16!(source);
        let inner_class_access_flags =
            flags::InnerClassAccessFlags::from_bits_truncate(read_u16!(source));
        Ok(Self {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        })
    }
}

impl StackMapFrame {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let frame_type = read_u8!(source);
        match frame_type {
            0..=63 => Ok(StackMapFrame::Same { frame_type }),
            64..=127 => StackMapFrame::parse_same_locals_1_item_stack(source, frame_type),
            247 => StackMapFrame::parse_same_locals_1_stack_item_frame_extended(source, frame_type),
            248..=250 => StackMapFrame::parse_chop_frame(source, frame_type),
            251 => StackMapFrame::parse_same_frame_extended(source, frame_type),
            252..=254 => StackMapFrame::parse_append_frame(source, frame_type),
            255 => StackMapFrame::parse_full_frame(source, frame_type),
            _ => panic!(),
        }
    }

    fn parse_same_locals_1_item_stack(
        source: &mut impl Read,
        frame_type: u8,
    ) -> Result<Self, ClassFileParseError> {
        Ok(Self::SameLocals1StackItem {
            frame_type,
            stack: VerificationTypeInfo::parse(source)?,
        })
    }

    fn parse_same_locals_1_stack_item_frame_extended(
        source: &mut impl Read,
        frame_type: u8,
    ) -> Result<Self, ClassFileParseError> {
        let offset_delta = read_u16!(source);
        let stack = VerificationTypeInfo::parse(source)?;
        Ok(Self::SameLocals1StackItemExtended {
            frame_type,
            offset_delta,
            stack,
        })
    }

    fn parse_chop_frame(
        source: &mut impl Read,
        frame_type: u8,
    ) -> Result<Self, ClassFileParseError> {
        Ok(Self::Chop {
            frame_type,
            offset_delta: read_u16!(source),
        })
    }

    fn parse_same_frame_extended(
        source: &mut impl Read,
        frame_type: u8,
    ) -> Result<Self, ClassFileParseError> {
        Ok(Self::SameExtended {
            frame_type,
            offset_delta: read_u16!(source),
        })
    }

    fn parse_append_frame(
        source: &mut impl Read,
        frame_type: u8,
    ) -> Result<Self, ClassFileParseError> {
        let offset_delta = read_u16!(source);
        let mut locals = Vec::with_capacity(frame_type as usize - 251);
        for _ in 0..frame_type - 251 {
            locals.push(VerificationTypeInfo::parse(source)?);
        }
        Ok(Self::Append {
            frame_type,
            offset_delta,
            locals,
        })
    }

    fn parse_full_frame(
        source: &mut impl Read,
        frame_type: u8,
    ) -> Result<Self, ClassFileParseError> {
        let offset_delta = read_u16!(source);
        let number_of_locals = read_u16!(source);
        let mut locals = Vec::with_capacity(number_of_locals as usize);
        for _ in 0..number_of_locals {
            locals.push(VerificationTypeInfo::parse(source)?);
        }
        let number_of_stack_items = read_u16!(source);
        let mut stack = Vec::with_capacity(number_of_stack_items as usize);
        for _ in 0..number_of_stack_items {
            stack.push(VerificationTypeInfo::parse(source)?);
        }

        Ok(Self::Full {
            frame_type,
            offset_delta,
            locals,
            stack,
        })
    }
}

impl VerificationTypeInfo {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let tag = read_u8!(source);
        match tag {
            0 => Ok(TopVariable { tag }),
            1 => Ok(IntegerVariable { tag }),
            2 => Ok(FloatVariable { tag }),
            3 => Ok(DoubleVariable { tag }),
            4 => Ok(LongVariable { tag }),
            5 => Ok(NullVariable { tag }),
            6 => Ok(UninitializedThisVariable { tag }),
            7 => Ok(ObjectVariable {
                tag,
                cpool_index: read_u16!(source),
            }),
            8 => Ok(UninitializedVariable {
                tag,
                offset: read_u16!(source),
            }),
            _ => Err(ClassFileParseError::InvalidVerificationTypeTag),
        }
    }
}

impl ExceptionTableEntry {
    pub fn parse(source: &mut impl Read) -> Result<Self, ClassFileParseError> {
        let start_pc = read_u16!(source);
        let end_pc = read_u16!(source);
        let handler_pc = read_u16!(source);
        let catch_type = read_u16!(source);
        Ok(ExceptionTableEntry {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classfile::flags::{ClassAccessFlags, MethodAccessFlags};
    use crate::classfile::AttributeInfo::{Code, LineNumberTable, SourceFile};
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn test_simple_class_file() {
        let f = File::open("tests/resources/Foo.class").unwrap();
        let mut rd = BufReader::new(f);
        let result = ClassFile::parse(&mut rd).unwrap();
        assert_eq!(
            ClassFile {
                magic: 0xCAFEBABE,
                minor_version: 0,
                major_version: 61,
                cp_info: ConstantPool {
                    items: vec![
                        MethodrefInfo {
                            class_index: 2,
                            name_and_type_index: 3
                        },
                        ClassInfo { name_index: 4 },
                        NameAndTypeInfo {
                            name_index: 5,
                            descriptor_index: 6
                        },
                        Utf8Info {
                            length: 16,
                            bytes: "java/lang/Object".as_bytes().into()
                        },
                        Utf8Info {
                            length: 6,
                            bytes: "<init>".as_bytes().into()
                        },
                        Utf8Info {
                            length: 3,
                            bytes: "()V".as_bytes().into()
                        },
                        ClassInfo { name_index: 8 },
                        Utf8Info {
                            length: 3,
                            bytes: "Foo".as_bytes().into()
                        },
                        Utf8Info {
                            length: 4,
                            bytes: "Code".as_bytes().into()
                        },
                        Utf8Info {
                            length: 15,
                            bytes: "LineNumberTable".as_bytes().into()
                        },
                        Utf8Info {
                            length: 3,
                            bytes: "bar".as_bytes().into()
                        },
                        Utf8Info {
                            length: 10,
                            bytes: "SourceFile".as_bytes().into()
                        },
                        Utf8Info {
                            length: 8,
                            bytes: "Foo.java".as_bytes().into()
                        },
                    ]
                },
                access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::SUPER,
                this_class: 7,
                super_class: 2,
                interfaces: vec![],
                fields: vec![],
                methods: vec![
                    MethodInfo {
                        access_flags: MethodAccessFlags::PUBLIC,
                        name_index: 5,
                        descriptor_index: 6,
                        attributes: vec![Code {
                            attribute_name_index: 9,
                            attribute_length: 29,
                            max_stack: 1,
                            max_locals: 1,
                            code_length: 5,
                            code: vec![0x2A, 0xB7, 0x00, 0x01, 0xB1],
                            exception_table: vec![],
                            attributes: vec![LineNumberTable {
                                attribute_name_index: 10,
                                attribute_length: 6,
                                line_number_table: vec![LineNumberTableEntry {
                                    start_pc: 0,
                                    line_number: 1
                                }]
                            }],
                        }]
                    },
                    MethodInfo {
                        access_flags: MethodAccessFlags::PUBLIC,
                        name_index: 11,
                        descriptor_index: 6,
                        attributes: vec![Code {
                            attribute_name_index: 9,
                            attribute_length: 25,
                            max_stack: 0,
                            max_locals: 1,
                            code_length: 1,
                            code: vec![0xB1],
                            exception_table: vec![],
                            attributes: vec![LineNumberTable {
                                attribute_name_index: 10,
                                attribute_length: 6,
                                line_number_table: vec![LineNumberTableEntry {
                                    start_pc: 0,
                                    line_number: 3
                                }]
                            }],
                        }]
                    }
                ],
                attributes: vec![SourceFile {
                    attribute_name_index: 12,
                    attribute_length: 2,
                    sourcefile_index: 13
                }]
            },
            result
        );
    }
}
