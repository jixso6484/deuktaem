
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagenation {
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PagenationResult<T> {
    pub data: Vec<T>,  // vec<T> -> Vec<T>
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}