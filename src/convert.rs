use leptos::html::ElementDescriptor;
use std::ops::Deref;
use wasm_bindgen::JsCast;

pub fn cast_html_element<T, U>(el: &leptos::html::HtmlElement<T>) -> &U
where
    T: Deref + ElementDescriptor + 'static,
    T::Target: JsCast,
    U: JsCast,
{
    el.deref().unchecked_ref()
}

pub fn f64_ceil_to_u8(f: f64) -> u8 {
    f64_u8(f.ceil())
}

pub fn f64_round_to_u8(f: f64) -> u8 {
    f64_u8(f.round())
}

pub fn f64_ceil_to_i32(f: f64) -> i32 {
    f64_i32(f.ceil())
}

macro_rules! f64_conv {
    ($n:ident($t:ty)) => {
        fn $n(f: f64) -> $t {
            const MAXF: f64 = <$t>::MAX as f64;
            const MINF: f64 = <$t>::MIN as f64;

            if f.is_infinite() || f.is_nan() {
                0
            } else if f < MINF {
                <$t>::MIN
            } else if f > MAXF {
                <$t>::MAX
            } else {
                unsafe { f.to_int_unchecked() }
            }
        }
    };
}

f64_conv!(f64_u8(u8));
f64_conv!(f64_i32(i32));
