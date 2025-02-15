use std::collections::HashMap;

use husky_check_utils::should;
use husky_vm::{EntityUid, __Register, __VMResult};

use super::*;

#[derive(Default, Debug)]
pub struct EvalSheet<'eval> {
    values: Mutex<HashMap<EvalKey, __VMResult<__Register<'eval>>>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EvalKey {
    Feature(FeaturePtr),
    StructDerivedField {
        this: *const c_void,
        field_uid: EntityUid,
    },
}

unsafe impl Send for EvalKey {}

unsafe impl Sync for EvalKey {}

impl<'eval> EvalSheet<'eval> {
    pub(crate) fn cached_value(&self, eval_key: EvalKey) -> Option<__VMResult<__Register<'eval>>> {
        self.values
            .lock()
            .unwrap()
            .get(&eval_key)
            .map(|v| unsafe { share_cached(v) })
    }

    pub(crate) fn try_cache(
        &self,
        eval_key: EvalKey,
        mut value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
        let mut values = self.values.lock().unwrap();
        if !values.contains_key(&eval_key) {
            let result = unsafe { cache_raw_eval_value(&mut value) };
            assert!(values.insert(eval_key, value).is_none());
            result
        } else {
            unsafe { share_cached(&values[&eval_key]) }
        }
    }

    pub(crate) fn cache(
        &self,
        eval_key: EvalKey,
        mut value: __VMResult<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
        let result = unsafe { cache_raw_eval_value(&mut value) };
        should!(
            self.values
                .lock()
                .unwrap()
                .insert(eval_key, value)
                .is_none(),
            "eval_key {eval_key:?}"
        );
        result
    }
}

unsafe fn cache_raw_eval_value<'eval>(
    raw: &mut __VMResult<__Register<'eval>>,
) -> __VMResult<__Register<'eval>> {
    match raw {
        Ok(ref mut value) => value.cache_eval(),
        Err(_) => (),
    }
    share_cached(raw)
}

unsafe fn share_cached<'eval>(
    cached: &__VMResult<__Register<'eval>>,
) -> __VMResult<__Register<'eval>> {
    match cached {
        Ok(value) => Ok(value.share_cached()),
        Err(error) => Err(error.clone()),
    }
}

pub trait HasFeatureSheet<'cache> {
    fn feature_sheet(&self, idx: usize) -> &EvalSheet<'cache>;
}
