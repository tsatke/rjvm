#[derive(PartialEq, Clone, Debug)]
pub enum NativeValue {
    Boolean(bool),
    Byte(i8),
    Char(u16),
    Short(i16),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Reference(usize),
    ReturnAddress(usize),
}
