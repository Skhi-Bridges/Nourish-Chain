use crate::type_safe_components::{TailwindClass, ResponsiveClass};

pub fn golden_gradient() -> TailwindClass {
    TailwindClass::new()
        .add("bg-gradient-to-r")
        .add("from-yellow-500")
        .add("to-amber-400")
}

pub fn golden_button() -> TailwindClass {
    TailwindClass::new()
        .add("bg-amber-500")
        .add("hover:bg-amber-600")
        .add("text-white")
        .add("font-bold")
        .add("rounded-lg")
        .add("transition-colors")
        .add("duration-300")
}

pub fn golden_text() -> TailwindClass {
    TailwindClass::new()
        .add("font-bold")
        .add("text-amber-600")
        .add("dark:text-yellow-400")
}

pub fn kombech_theme(dark_mode: bool) -> TailwindClass {
    let mut class = TailwindClass::new();
    if dark_mode {
        class.add("bg-gray-900").add("text-yellow-100");
    } else {
        class.add("bg-amber-50").add("text-gray-900");
    }
    class
}
