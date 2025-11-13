use std::fmt;

/// Trait for formatting protobuf values for printing.
///
/// This trait is used by generated code to format field values
/// when printing protobuf messages.
pub trait PbPrint {
    /// Format the value for protobuf printing.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

// Implement PbPrint for basic types
impl PbPrint for u32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PbPrint for u64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PbPrint for i32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PbPrint for i64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PbPrint for f32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PbPrint for f64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PbPrint for bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PbPrint for str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PbPrint for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl PbPrint for [u8] {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format bytes as hex or quoted string
        use protobuf_support::text_format::quote_bytes_to;
        let mut buf = String::new();
        quote_bytes_to(self, &mut buf);
        f.write_str(&buf)
    }
}

impl PbPrint for Vec<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

// Implement PbPrint for Timestamp
impl PbPrint for crate::well_known_types::timestamp::Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the Display implementation which formats via text_format
        fmt::Display::fmt(self, f)
    }
}

// Implement PbPrint for Chars (when bytes feature is enabled)
#[cfg(feature = "bytes")]
impl PbPrint for crate::chars::Chars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Chars derefs to str, so use Display
        fmt::Display::fmt(self, f)
    }
}

#[cfg(feature = "bytes")]
impl PbPrint for ::bytes::Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format bytes as hex or quoted string
        use protobuf_support::text_format::quote_bytes_to;
        let mut buf = String::new();
        quote_bytes_to(self.as_ref(), &mut buf);
        f.write_str(&buf)
    }
}

