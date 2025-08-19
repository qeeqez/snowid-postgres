#![allow(unexpected_cfgs)]
#![allow(unused_imports)]

use heapless::FnvIndexMap;
use pgrx::atomics::PgAtomic;
use pgrx::lwlock::PgLwLock;
use pgrx::pg_shmem_init;
use pgrx::prelude::*;
use pgrx::shmem::PGRXSharedMemory;
use pgrx::shmem::AssertPGRXSharedMemory;
use snowid::SnowID;
use std::ffi::CStr;
use std::sync::atomic::{AtomicI16, Ordering};