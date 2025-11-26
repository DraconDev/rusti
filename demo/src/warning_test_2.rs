#[must_use = "Unused CSS classes: foo, bar, baz"]
struct CSS_Validation_Warning;

pub fn test() {
    // This should trigger the warning
    CSS_Validation_Warning;
}
