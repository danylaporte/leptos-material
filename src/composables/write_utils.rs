use leptos::{event_target_checked, event_target_value, SignalSet};
use wasm_bindgen::JsCast;
use web_sys::Event;

pub fn write_input<S>(signal: S) -> impl FnMut(Event) + 'static
where
    S: SignalSet<String> + 'static,
{
    move |e| signal.set(event_target_value(&e))
}

pub fn write_checked<S>(signal: S) -> impl FnMut(Event) -> () + 'static
where
    S: SignalSet<bool> + 'static,
{
    move |e| signal.set(event_target_checked(e.unchecked_ref::<Event>()))
}
