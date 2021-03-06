use super::sqlite::SqlTable;
use crate::{
    hash::{Hash, HashPrimitive},
    table::general::Bucket,
    DataPoint, DataPointSlice, HashTables, Result,
};
use fnv::FnvHashSet;
use std::ops::{Deref, DerefMut};
use std::path::Path;

/// In memory Sqlite backend for [LSH](struct.LSH.html).
pub struct SqlTableMem {
    sql_table: SqlTable,
}

impl SqlTableMem {
    pub fn to_db<P: AsRef<Path>>(&mut self, db_path: P) -> Result<()> {
        let mut new_con = rusqlite::Connection::open(db_path)?;
        {
            let backup = rusqlite::backup::Backup::new(&self.conn, &mut new_con)?;
            backup.step(-1)?;
        }
        self.conn = new_con;
        self.committed.set(true);
        Ok(())
    }
}

impl Deref for SqlTableMem {
    type Target = SqlTable;

    fn deref(&self) -> &SqlTable {
        &self.sql_table
    }
}

impl DerefMut for SqlTableMem {
    fn deref_mut(&mut self) -> &mut SqlTable {
        &mut self.sql_table
    }
}

impl HashTables for SqlTableMem {
    fn new(n_hash_tables: usize, only_index_storage: bool, _db_path: &str) -> Result<Box<Self>> {
        let conn = rusqlite::Connection::open_in_memory()?;
        let sql_table = SqlTable::init_from_conn(n_hash_tables, only_index_storage, conn)?;
        Ok(Box::new(SqlTableMem { sql_table }))
    }

    /// # Arguments
    ///
    /// * `hash` - hashed vector.
    /// * `d` - Vector to store in the buckets.
    /// * `hash_table` - Number of the hash_table to store the vector. Ranging from 0 to L.
    fn put(&mut self, hash: Hash, d: &DataPointSlice, hash_table: usize) -> Result<u32> {
        self.sql_table.put(hash, d, hash_table)
    }

    fn delete(&mut self, hash: &Hash, d: &DataPointSlice, hash_table: usize) -> Result<()> {
        self.sql_table.delete(hash, d, hash_table)
    }

    /// Query the whole bucket
    fn query_bucket(&self, hash: &Hash, hash_table: usize) -> Result<Bucket> {
        self.sql_table.query_bucket(hash, hash_table)
    }

    fn idx_to_datapoint(&self, idx: u32) -> Result<&DataPoint> {
        self.sql_table.idx_to_datapoint(idx)
    }

    fn describe(&self) -> Result<String> {
        self.sql_table.describe()
    }

    fn get_unique_hash_int(&self) -> FnvHashSet<HashPrimitive> {
        self.sql_table.get_unique_hash_int()
    }
}
