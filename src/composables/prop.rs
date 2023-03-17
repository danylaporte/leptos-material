use leptos::{RwSignal, Signal, SignalGet, SignalSet, SignalUpdate, SignalWith};

#[derive(Clone, Copy)]
pub enum Prop<T: 'static> {
    Signal(Signal<T>),
    RwSignal(RwSignal<T>),
    Static(T),
}

impl<T: Default> Default for Prop<T> {
    fn default() -> Self {
        Self::Static(T::default())
    }
}

impl<T> From<RwSignal<T>> for Prop<T> {
    fn from(v: RwSignal<T>) -> Self {
        Self::RwSignal(v)
    }
}

impl<T> From<Signal<T>> for Prop<T> {
    fn from(v: Signal<T>) -> Self {
        Self::Signal(v)
    }
}

impl<T> From<T> for Prop<T> {
    fn from(v: T) -> Self {
        Self::Static(v)
    }
}

impl From<&str> for Prop<String> {
    fn from(value: &str) -> Self {
        Self::Static(value.into())
    }
}

impl<T: Clone> SignalGet<T> for Prop<T> {
    fn get(&self) -> T {
        match self {
            Self::Signal(v) => v.get(),
            Self::RwSignal(v) => v.get(),
            Self::Static(v) => v.clone(),
        }
    }

    fn try_get(&self) -> Option<T> {
        match self {
            Self::Signal(v) => v.try_get(),
            Self::RwSignal(v) => v.try_get(),
            Self::Static(v) => Some(v.clone()),
        }
    }
}

impl<T: Clone> SignalSet<T> for Prop<T> {
    #[track_caller]
    fn set(&self, new_value: T) {
        match self {
            Self::RwSignal(s) => s.set(new_value),
            Self::Static(_) | Self::Signal(_) => {}
        }
    }

    #[track_caller]
    fn try_set(&self, new_value: T) -> Option<T> {
        match self {
            Self::RwSignal(s) => s.try_set(new_value),
            Self::Static(_) | Self::Signal(_) => Some(new_value),
        }
    }
}

impl<T> SignalUpdate<T> for Prop<T> {
    fn update(&self, f: impl FnOnce(&mut T)) {
        match self {
            Self::RwSignal(s) => s.update(f),
            Self::Signal(_) | Self::Static(_) => {}
        }
    }

    fn try_update<O>(&self, f: impl FnOnce(&mut T) -> O) -> Option<O> {
        match self {
            Self::RwSignal(s) => s.try_update(f),
            Self::Signal(_) | Self::Static(_) => None,
        }
    }
}

impl<T> SignalWith<T> for Prop<T> {
    fn with<O>(&self, f: impl FnOnce(&T) -> O) -> O {
        match self {
            Self::RwSignal(s) => s.with(f),
            Self::Signal(s) => s.with(f),
            Self::Static(s) => f(s),
        }
    }

    fn try_with<O>(&self, f: impl FnOnce(&T) -> O) -> Option<O> {
        match self {
            Self::RwSignal(s) => s.try_with(f),
            Self::Signal(s) => s.try_with(f),
            Self::Static(s) => Some(f(s)),
        }
    }
}

#[cfg(not(feature = "stable"))]
impl<T: Clone> FnOnce<()> for Prop<T> {
    type Output = T;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.get()
    }
}

#[cfg(not(feature = "stable"))]
impl<T: Clone> FnMut<()> for Prop<T> {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        self.get()
    }
}

#[cfg(not(feature = "stable"))]
impl<T: Clone> Fn<()> for Prop<T> {
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        self.get()
    }
}
