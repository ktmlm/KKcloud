//!
//! # Resource Master
//!
//! This module act as the global controller, all resoures are under its management.
//!

use crate::model::{Env, EnvId, User, UserId, Vm, VmId};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

lazy_static! {
    /// Global entrypoint.
    pub static ref SERV: Arc<Service> = Arc::new(Service::default());
}

/// Service is a global data collection.
#[derive(Default)]
pub struct Service {
    #[allow(missing_docs)]
    pub user_to_env: Arc<RwLock<HashMap<UserId, User>>>,
    #[allow(missing_docs)]
    pub all_env: Arc<RwLock<HashMap<EnvId, Env>>>,
    #[allow(missing_docs)]
    pub all_vm: Arc<RwLock<HashMap<VmId, Vm>>>,
}
