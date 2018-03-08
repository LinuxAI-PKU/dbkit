
use std::convert::{AsRef, From};
use std::fmt;
use std::mem;
use std::slice;
use std::str;

use super::error::DBError;

pub trait NullInfo {
    const NULLABLE: bool;
}

pub struct Nullable;
pub struct NotNullable;

impl NullInfo for Nullable {
    const NULLABLE: bool = true;
}

impl NullInfo for NotNullable {
    const NULLABLE: bool = false;
}

/// "Native" type storing `Column` data for VARLEN columns
#[derive(Clone, Copy)]
pub struct RawData {
    // This cannot me a &[u8] slice because slices cannot be have a nullptr
    pub data: *mut u8,
    pub size: usize,
}

impl AsRef<str> for RawData {
    fn as_ref(&self) -> &str {
        unsafe {
            let a = slice::from_raw_parts(self.data, self.size);
            str::from_utf8_unchecked(a)
        }
    }
}

/// "Symbolic" Type of a `Column` `Attribute`
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Type {
    UINT32,
    UINT64,
    INT32,
    INT64,
    FLOAT32,
    FLOAT64,
    BOOLEAN,
    TEXT,
    BLOB,
}

/// Trait providing higher level metadata about types
pub trait TypeInfo {
    /// The native Rust type backing the column vector
    type Store;

    /// Symbolic type
    const ENUM: Type;

    /// Do this value require deep copying of data (stored in the `Column' arena)
    const DEEP_COPY: bool = true;

    const VARLEN: bool = false;

    const SIZE: usize = mem::size_of::<Self::Store>();
}

pub struct UInt32;
pub struct UInt64;
pub struct Int32;
pub struct Int64;
pub struct Float32;
pub struct Float64;
pub struct Boolean;
pub struct Text;
pub struct Blob;

impl TypeInfo for UInt32 {
    type Store = u32;
    const ENUM: Type = Type::UINT32;
}

impl TypeInfo for UInt64 {
    type Store = u64;
    const ENUM: Type = Type::UINT64;
}

impl TypeInfo for Int32 {
    type Store = i32;
    const ENUM: Type = Type::INT32;
}

impl TypeInfo for Int64 {
    type Store = i64;
    const ENUM: Type = Type::INT64;
}

impl TypeInfo for Float32 {
    type Store = f32;
    const ENUM: Type = Type::FLOAT32;
}

impl TypeInfo for Float64 {
    type Store = f64;
    const ENUM: Type = Type::FLOAT64;
}

impl TypeInfo for Boolean {
    type Store = bool;
    const ENUM: Type = Type::BOOLEAN;
}

impl TypeInfo for Text {
    type Store = RawData;
    const ENUM: Type = Type::TEXT;
    const DEEP_COPY: bool = true;
    const VARLEN: bool = true;
}

impl TypeInfo for Blob {
    type Store = RawData;
    const ENUM: Type = Type::BLOB;
    const VARLEN: bool = true;
}

static UINT32: UInt32 = UInt32{};
static UINT64: UInt64 = UInt64{};
static INT32: Int32 = Int32{};
static INT64: Int64 = Int64{};
static FLOAT32: Float32 = Float32{};
static FLOAT64: Float64 = Float64{};
static BOOLEAN: Boolean = Boolean{};
static TEXT: Text = Text{};
static BLOB: Blob = Blob{};

impl Type {
    pub fn name(self) -> &'static str {
        match self {
            Type::UINT32  => "UINT32",
            Type::UINT64  => "UINT64",
            Type::INT32   => "INT32",
            Type::INT64   => "INT64",
            Type::FLOAT32 => "FLOAT32",
            Type::FLOAT64 => "FLOAT64",
            Type::BOOLEAN => "BOOLEAN",
            Type::TEXT    => "TEXT",
            Type::BLOB    => "BLOB",
        }
    }

    // RUST is frustrating
    // There's no implementation specialization,
    // and can't use a associated trait type (defaulted or not) in an expression.
    // So we have to keep repeating ourselves
    pub fn size_of(self) -> usize {
        match self {
            Type::UINT32    => UInt32::SIZE,
            Type::UINT64    => UInt64::SIZE,
            Type::INT32     => Int32::SIZE,
            Type::INT64     => Int64::SIZE,
            Type::FLOAT32   => Float32::SIZE,
            Type::FLOAT64   => Float64::SIZE,
            Type::BOOLEAN   => Boolean::SIZE,
            Type::TEXT      => Text::SIZE,
            Type::BLOB      => Blob::SIZE,
        }
    }
}

impl str::FromStr for Type {
    type Err = DBError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UINT32"  => Ok(Type::UINT32),
            "UINT64"  => Ok(Type::UINT64),
            "INT32"   => Ok(Type::INT32),
            "INT64"   => Ok(Type::INT64),
            "FLOAT32" => Ok(Type::FLOAT32),
            "FLOAT64" => Ok(Type::FLOAT64),
            "BOOLEAN" => Ok(Type::BOOLEAN),
            "TEXT"    => Ok(Type::TEXT),
            "BLOB"    => Ok(Type::BLOB),
            _         => Err(DBError::UnknownType(String::from(s)))
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsRef<[u8]> for RawData {
    fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.data, self.size) }
    }
}

impl ToString for RawData {
    fn to_string(&self) -> String {
        let str: &str = self.as_ref();
        String::from(str)
    }
}

/// Value representing the null database column value
pub struct NullType { }
pub const NULL_VALUE: NullType = NullType {};

/// Container storing any kind of value
pub enum Value<'a> {
    NULL,
    UINT32(u32),
    UINT64(u64),
    INT32(i32),
    INT64(i64),
    FLOAT32(f32),
    FLOAT64(f64),
    BOOLEAN(bool),
    TEXT(&'a str),
    BLOB(&'a [u8]),
}

impl<'a> From<NullType> for Value<'a> {
    fn from(_: NullType) -> Self {
        Value::NULL
    }
}

impl<'a> From<u32> for Value<'a> {
    fn from(v: u32) -> Self {
        Value::UINT32(v)
    }
}

impl<'a> From<u64> for Value<'a> {
    fn from(v: u64) -> Self {
        Value::UINT64(v)
    }
}

impl<'a> From<i32> for Value<'a> {
    fn from(v: i32) -> Self {
        Value::INT32(v)
    }
}

impl<'a> From<i64> for Value<'a> {
    fn from(v: i64) -> Self {
        Value::INT64(v)
    }
}

impl<'a> From<f32> for Value<'a> {
    fn from(v: f32) -> Self {
        Value::FLOAT32(v)
    }
}

impl<'a> From<f64> for Value<'a> {
    fn from(v: f64) -> Self {
        Value::FLOAT64(v)
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(v: &'a str) -> Self {
        Value::TEXT(v)
    }
}

impl<'a> From<&'a [u8]> for Value<'a> {
    fn from(v: &'a [u8]) -> Self {
        Value::BLOB(v)
    }
}
