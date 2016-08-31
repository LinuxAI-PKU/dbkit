use super::error::DBError;
use super::allocator::Allocator;

use super::block::RefView;
use super::row::RowOffset;
use super::schema::Schema;

pub enum CursorChunk<'a> {
    /// Next chunk
    Next(RefView<'a>),
    /// End of stream
    End,

    // TODO: Next for off memory data (GPU)
}

/// Materialized operation cursor stream results from previous operations.
///
/// A cursor know it output and (optionally) input schema.
pub trait Cursor<'a> {
    fn schema(&self) -> Schema;

    // Can't quite be an iterator, we can want different batch sizes in subsequent calls
    fn next(&'a mut self, rows: RowOffset) -> Result<CursorChunk<'a>, DBError>;
}

/// Operation that's part of the operation AST
pub trait Operation<'a> {
    /// Convert operation AST a bound Cursor
    fn bind(&'a self, &Allocator) -> Result<Box<Cursor<'a> + 'a>, DBError>;
}

pub mod scan_view;
pub use self::scan_view::ScanView;

