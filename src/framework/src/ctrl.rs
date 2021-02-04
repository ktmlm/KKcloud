//!
//! # Resource Controller
//!
//! All resoures are under this moduler's management.
//!

use crate::{
    err::*,
    model::{Env, EnvId, User, UserId, Vm, VmEngine, VmId, VmTemplate},
};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use ruc::{err::*, *};
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::Arc,
};

lazy_static! {
    /// Global entrypoint.
    pub static ref SERV: ServCtrl = Arc::new(Service::default());
    /// Collections of vm-engines, can NOT be changed in runtime.
    pub static ref ENGINE: EngineCtrl = pnk!(EngineCtrl::init(None));
    /// Collections of vm-templates, can be updated in runtime.
    pub static ref TEMPLATE: TemplateCtrl = TemplateCtrl::default();
}

type ServCtrl = Arc<Service>;

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
#[derive(Clone)]
pub struct EngineCtrl(Arc<HashMap<String, Arc<dyn VmEngine>>>);

impl EngineCtrl {
    /// Caller(user) uses this function to init [ENGINES](self::ENGINES).
    pub fn init(em: Option<Vec<Arc<dyn VmEngine>>>) -> Option<EngineCtrl> {
        static mut EM: Option<EngineCtrl> = None;

        unsafe {
            if let Some(e) = EM.as_ref() {
                Some(e.clone())
            } else if let Some(e) = em {
                let ret = EngineCtrl(Arc::new(
                    e.into_iter().map(|ve| (ve.name().to_owned(), ve)).collect(),
                ));
                EM = Some(ret.clone());
                Some(ret)
            } else {
                None
            }
        }
    }
}

impl Deref for EngineCtrl {
    type Target = Arc<HashMap<String, Arc<dyn VmEngine>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The container of vm templates,
/// {Vm Template Name} => {Vm Template Object}
#[derive(Default)]
pub struct TemplateCtrl(Arc<RwLock<HashMap<String, Arc<VmTemplate>>>>);

impl TemplateCtrl {
    /// Replace the whole data with a new one.
    #[inline(always)]
    pub fn reinit(&mut self, t: HashMap<String, VmTemplate>) {
        *self.write() = t.into_iter().map(|(k, v)| (k, Arc::new(v))).collect();
    }

    /// Add all given elements to current data.
    #[inline(always)]
    pub fn add(&mut self, t: HashMap<String, VmTemplate>) {
        let mut ts = self.write();
        t.into_iter().for_each(|(k, v)| {
            ts.insert(k, Arc::new(v));
        })
    }

    /// Similar to `add`, but ensure none of existing templetes will be replaced.
    #[inline(always)]
    pub fn add_safe(&mut self, t: HashMap<String, VmTemplate>) -> Result<()> {
        if self.read().keys().any(|k| t.get(k).is_some()) {
            return Err(e!(ERR_KK_CTRL_UPDATE_TEMPLATE).into());
        }
        self.add(t);
        Ok(())
    }

    /// Delete all given templates from current data.
    #[inline(always)]
    pub fn del(&mut self, t: HashSet<String>) {
        let mut ts = self.write();
        t.iter().for_each(|t| {
            ts.remove(t);
        })
    }

    /// Get an reference of `VmTemplate` from a given name.
    pub fn get(&self, name: &str) -> Option<Arc<VmTemplate>> {
        self.read().get(name).map(|t| Arc::clone(t))
    }
}

impl Deref for TemplateCtrl {
    type Target = Arc<RwLock<HashMap<String, Arc<VmTemplate>>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
