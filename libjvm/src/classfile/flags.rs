use bitflags::bitflags;

bitflags! {
    pub struct ModuleFlags: u16 {
        const OPEN = 0x0020;
        const SYNTHETIC = 0x1000;
        const MODULE = 0x8000;
    }
}

bitflags! {
    pub struct RequiresFlags: u16 {
        const TRANSITIVE = 0x0020;
        const STATIC_PHASE = 0x0040;
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }
}

bitflags! {
    pub struct ExportsFlags: u16 {
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }
}

bitflags! {
    pub struct OpensFlags: u16 {
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }
}

bitflags! {
    /// Specified by [`$4.1-B`]
    ///
    /// [`$4.1-B`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.1-200-E.1
    pub struct ClassAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const FINAL = 0x0010;
        const SUPER = 0x0020;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
        const MODULE = 0x8000;
    }
}

bitflags! {
    /// Specified by [`$4.5-A`]
    ///
    /// [`$4.5-A`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.5-200-A.1
    pub struct FieldAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const VOLATILE = 0x0040;
        const TRANSIENT = 0x0080;
        const SYNTHETIC = 0x1000;
        const ENUM = 0x4000;
    }
}

bitflags! {
    /// Specified by [`$4.6-A`]
    ///
    /// [`$4.6-A`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.6-200-A.1
    pub struct MethodAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE = 0x0040;
        const VARARGS = 0x0080;
        const NATIVE = 0x0100;
        const ABSTRACT = 0x0400;
        const STRICT = 0x0800;
        const SYNTHETIC = 0x1000;
    }
}

bitflags! {
    /// Specified by [`$4.7.6-A`]
    ///
    /// [`$4.7.6-A`]: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.6-300-D.1-D.1
    pub struct InnerClassAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
    }
}

bitflags! {
    pub struct MethodParameterAccessFlags: u16 {
        const FINAL = 0x0010;
        const SYNTHETIC = 0x1000;
        const MANDATED = 0x8000;
    }
}
