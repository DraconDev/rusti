#[deprecated(note = "Unused CSS classes: foo, bar")]
fn deprecated_warning() {}

#[must_use = "Unused CSS classes: foo, bar"]
fn must_use_warning() -> i32 {
    0
}

pub fn test() {
    deprecated_warning();
    must_use_warning();
}
