use derive_new::new;
use std::sync::{Arc, Weak};
use tokio::sync::Mutex;
// -------------------- STRONG CONTEXT -------------------- //
#[derive(new, Clone)]
pub struct StrongContext<T> {
    pub context: Arc<Mutex<T>>,
}

impl<T> StrongContext<T> {
    pub fn downgrade(self) -> WeakContext<T> {
        WeakContext::from(self)
    }
}

impl<T> PartialEq for StrongContext<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let self_data = self.context.blocking_lock();
        let other_data = other.context.blocking_lock();
        *self_data == *other_data
    }
}

impl<T> From<T> for StrongContext<T> {
    fn from(t: T) -> Self {
        Self::new(Arc::new(Mutex::new(t)))
    }
}

impl<T> From<WeakContext<T>> for Option<StrongContext<T>> {
    fn from(weak: WeakContext<T>) -> Option<StrongContext<T>> {
        if let Some(c) = weak.context.upgrade() {
            return Some(StrongContext::new(c));
        }
        None
    }
}

// -------------------- WEAK CONTEXT -------------------- //
#[derive(new, Clone)]
pub struct WeakContext<T> {
    pub context: Weak<Mutex<T>>,
}
impl<T> WeakContext<T> {
    pub fn upgrade(self) -> Option<StrongContext<T>> {
        Option::<StrongContext<T>>::from(self)
    }
}
impl<T> PartialEq for WeakContext<T>
where
    T: PartialEq + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        if let Some(self_ctx) = self.clone().upgrade() {
            if let Some(other_ctx) = other.clone().upgrade() {
                return self_ctx == other_ctx;
            }
        }
        false
    }
}
impl<T> From<StrongContext<T>> for WeakContext<T> {
    fn from(other: StrongContext<T>) -> Self {
        Self::new(Arc::downgrade(&other.context))
    }
}
