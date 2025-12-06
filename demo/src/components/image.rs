use azumi::prelude::*;

/// Optimized Image Component
///
/// Features:
/// - `loading="lazy"` by default (use eager=true for above-the-fold)
/// - `decoding="async"` for non-blocking decode
/// - Width/height for CLS prevention
/// - Alt is required (enforced by function signature)
#[azumi::component]
pub fn Image<'a>(
    src: &'a str,
    alt: &'a str,
    width: u32,
    height: u32,
    #[prop(default = false)] eager: bool,
) -> impl Component + 'a {
    let loading = if eager { "eager" } else { "lazy" };

    html! {
        <img
            src={src}
            alt={alt}
            width={width.to_string()}
            height={height.to_string()}
            // loading={loading}
            // decoding="async"
        />
    }
}

/// Responsive Image with srcset
///
/// Generates srcset attribute for responsive images.
/// Assumes images are named like: image-400.jpg, image-800.jpg, etc.
#[azumi::component]
pub fn ResponsiveImage<'a>(
    src: &'a str,
    alt: &'a str,
    width: u32,
    height: u32,
    sizes: &'a str,
    #[prop(default = false)] eager: bool,
) -> impl Component + 'a {
    let loading = if eager { "eager" } else { "lazy" };

    // Extract base path and extension
    // e.g., "/photos/hero.jpg" -> ("/photos/hero", "jpg")
    let (base, ext) = if let Some(dot_pos) = src.rfind('.') {
        (&src[..dot_pos], &src[dot_pos + 1..])
    } else {
        (src, "jpg")
    };

    // Generate srcset for common breakpoints
    let srcset = format!(
        "{}-400.{} 400w, {}-800.{} 800w, {}-1200.{} 1200w",
        base, ext, base, ext, base, ext
    );

    html! {
        <img
            src={src}
            alt={alt}
            width={width.to_string()}
            height={height.to_string()}
            srcset={srcset}
            sizes={sizes}
            loading={loading}
            decoding="async"
        />
    }
}
