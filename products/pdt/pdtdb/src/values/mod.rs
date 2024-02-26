mod microblock;
mod transaction;

// we encode bytes in rust as base64 strings, but postgres treats these strings
// as text and thus stores bytes that decode to base64 strings instead of the
// actual data, which is why we have separate structs for postgres

pub type BQTransaction = transaction::BQTransaction;
pub type PSQLTransaction = transaction::PSQLTransaction;
pub type ZILTransactionBody = transaction::ZILTransactionBody;
pub type BQMicroblock = microblock::BQMicroblock;
pub type PSQLMicroblock = microblock::PSQLMicroblock;
