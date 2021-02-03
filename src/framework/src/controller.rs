//!
//! # Resource Master
//!
//! This module act as the global controller, all resoures are under its management.
//!

use crate::model::{Env, EnvId, User, UserId, Vm, VmEngine, VmId, VmTemplate};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use ruc::{err::*, *};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

lazy_static! {
    /// Global entrypoint.
    pub static ref SERV: Arc<Service> = Arc::new(Service::default());
    /// Collections of vm-engines, can NOT be changed in runtime.
    pub static ref ENGINES: Arc<EngineMap> = pnk!(init_engines(None));
    /// Collections of vm-templates, can be updated in runtime.
    pub static ref TEMPLATES: TemplateMap = TemplateMap::default();
}

/// Service is a global data collection.
#[derive(Default)]
pub struct Service {
    #[allow(missing_docs)]
    pub all_user: Arc<RwLock<HashMap<UserId, User>>>,
    #[allow(missing_docs)]
    pub all_env: Arc<RwLock<HashMap<EnvId, Env>>>,
    #[allow(missing_docs)]
    pub all_vm: Arc<RwLock<HashMap<VmId, Vm>>>,
}

/// {Vm Engine Name} => {Vm Engine Object}
pub type EngineMap = HashMap<String, Arc<dyn VmEngine>>;

/// The container of vm templates,
/// {Vm Template Name} => {Vm Template Object}
#[derive(Default)]
pub struct TemplateMap(Arc<RwLock<HashMap<String, VmTemplate>>>);

impl TemplateMap {
    /// Replace the whole data with a new one.
    #[inline(always)]
    pub fn reinit(&mut self, t: HashMap<String, VmTemplate>) {
        *self.0.write() = t;
    }

    /// Add all given elements to current data.
    #[inline(always)]
    pub fn update(&mut self, t: HashMap<String, VmTemplate>) {
        let mut ts = self.0.write();
        t.into_iter().for_each(|(k, v)| {
            ts.insert(k, v);
        })
    }

    /// Similar to `update`, but ensure none of existing templetes will be replaced.
    #[inline(always)]
    pub fn update_safe(&mut self, t: HashMap<String, VmTemplate>) -> Result<()> {
        if self.0.read().keys().any(|k| t.get(k).is_some()) {
            return Err(eg!());
        }
        self.update(t);
        Ok(())
    }

    /// Remove all given templates from current data.
    #[inline(always)]
    pub fn remove(&mut self, t: HashSet<String>) {
        let mut ts = self.0.write();
        t.iter().for_each(|t| {
            ts.remove(t);
        })
    }
}

/// Caller(user) uses this function to init [ENGINES](self::ENGINES).
pub fn init_engines(em: Option<EngineMap>) -> Option<Arc<EngineMap>> {
    static mut EM: Option<Arc<EngineMap>> = None;

    unsafe {
        if let Some(ref e) = EM {
            Some(Arc::clone(e))
        } else if let Some(e) = em {
            let ret = Arc::new(e);
            EM = Some(Arc::clone(&ret));
            Some(ret)
        } else {
            None
        }
    }
}
