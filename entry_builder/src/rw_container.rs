/// Marker that defines whether an Operation performs a `READ` or a `WRITE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RW {
    /// Marks op as READ.
    READ,
    /// Marks op as WRITE.
    WRITE,
}

impl RW {
    /// Returns true if the RW corresponds internally to a [`READ`](RW::READ).
    pub const fn is_read(&self) -> bool {
        matches!(self, RW::READ)
    }
    /// Returns true if the RW corresponds internally to a [`WRITE`](RW::WRITE).
    pub const fn is_write(&self) -> bool {
        matches!(self, RW::WRITE)
    }
}

// new Memory or Regisger ops for u64 value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RwOp {
    pub global_clk: u64,
    pub rwc: u64,
    pub rw: RW,
    /// Memory or Register address
    pub address: u64,
    /// Value
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RwContainer {
    /// Operations of memory and register
    pub rw_ops: Vec<RwOp>,
}

impl Default for RwContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl RwContainer {
    pub fn new() -> Self {
        Self { rw_ops: Vec::new() }
    }

    pub fn push_read_op(&mut self, gc: u64, address: u64, value: u64) {
        let read_op = RwOp {
            global_clk: gc,
            rwc: self.rw_ops.len() as u64,
            rw: RW::READ,
            address,
            value,
        };
        self.rw_ops.push(read_op);
    }

    pub fn push_write_op(&mut self, gc: u64, address: u64, value: u64) {
        let write_op = RwOp {
            global_clk: gc,
            rwc: self.rw_ops.len() as u64,
            rw: RW::WRITE,
            address,
            value,
        };
        self.rw_ops.push(write_op);
    }
}
