//! # Leptos Lucide Icons
//!
//! A comprehensive Lucide icon library for Leptos with tree-shaking support.
//!
//! ## Features
//!
//! - 🌳 **Tree-shaking**: Only bundle the icons you actually use
//! - ⚡ **Zero-cost**: All icon components are `#[inline(always)]` for maximum performance
//! - 🦀 **Rust-friendly**: Generated component names follow Rust conventions
//! - 🔒 **Type-safe**: Each icon is a separate typed component
//! - 🎨 **Customizable**: Easy styling with CSS classes and inline styles
//!
//! ## Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_lucide_rs::*;
//!
//! #[component]
//! pub fn MyComponent() -> impl IntoView {
//!     view! {
//!         <div>
//!             <Home/>
//!             <User/>
//!             <Heart/>
//!         </div>
//!     }
//! }
//! ```
//!
//! ## Custom Styling
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_lucide_rs::*;
//!
//! #[component]
//! pub fn StyledIcons() -> impl IntoView {
//!     view! {
//!         <div>
//!             // Using the macro for custom classes
//!             // TODO:
//!             // {icon!(Home, class = "text-blue-500 w-6 h-6")}
//!
//!             // Using the macro for custom sizes
//!             // TODO:
//!             // {icon!(User, size = "32px")}
//!
//!             // Direct component with custom attributes
//!             <div class="icon-wrapper">
//!                 <Search/>
//!             </div>
//!         </div>
//!     }
//! }
//! ```

use leptos::prelude::*;

// Include the generated icons module
#[cfg(leptos_lucide_generated)]
include!(concat!(env!("OUT_DIR"), "/icons.rs"));

// Fallback module when icons aren't generated yet (for IDE support)
#[cfg(not(leptos_lucide_generated))]
pub mod fallback {
    use leptos::*;

    /// Fallback Home icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn Home() -> impl IntoView {
        load_icon_fallback("home")
    }

    /// Fallback User icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn User() -> impl IntoView {
        load_icon_fallback("user")
    }

    /// Fallback Heart icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn Heart() -> impl IntoView {
        load_icon_fallback("heart")
    }

    /// Fallback Search icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn Search() -> impl IntoView {
        load_icon_fallback("search")
    }

    /// Fallback Star icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn Star() -> impl IntoView {
        load_icon_fallback("star")
    }

    /// Generic fallback icon loader using lucide-svg-rs
    fn load_icon_fallback(name: &str) -> impl IntoView {
        view! {
            <svg
                class="lucide-icon"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                data-lucide=name
                inner_html=move || {
                    let client = lucide_svg_rs::LucideClient::default();
                    match client.get_icon_content(&format!("{}.svg", name)) {
                        Ok(svg_content) => {
                            // Extract content between <svg> tags
                            if let (Some(start), Some(end)) = (svg_content.find('>'), svg_content.rfind("</svg>")) {
                                if start < end {
                                    return svg_content[start + 1..end].to_string();
                                }
                            }
                            // Ultimate fallback if parsing fails
                            r#"<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path>"#.to_string()
                        }
                        Err(_) => {
                            // Ultimate fallback
                            r#"<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path>"#.to_string()
                        }
                    }
                }
            >
            </svg>
        }
    }

    /// Dynamic icon loader (same as generated version)
    pub fn load_icon(name: &str) -> impl IntoView {
        load_icon_fallback(name)
    }
}

// Re-export fallback icons when not generated
#[cfg(not(leptos_lucide_generated))]
pub use fallback::*;

/// Re-export the load_icon function for dynamic loading
#[cfg(not(leptos_lucide_generated))]
pub use fallback::load_icon;

/// Icon configuration for customizing appearance
#[derive(Clone, Debug, Default)]
pub struct IconConfig {
    /// CSS class to apply to the icon
    pub class: Option<String>,
    /// Inline style string
    pub style: Option<String>,
    /// Icon size (width and height)
    pub size: Option<String>,
    /// Custom stroke width
    pub stroke_width: Option<String>,
    /// Custom stroke color
    pub stroke: Option<String>,
    /// Fill color
    pub fill: Option<String>,
}

impl IconConfig {
    /// Create a new icon configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the CSS class
    pub fn class<S: Into<String>>(mut self, class: S) -> Self {
        self.class = Some(class.into());
        self
    }

    /// Set the inline style
    pub fn style<S: Into<String>>(mut self, style: S) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Set the icon size
    pub fn size<S: Into<String>>(mut self, size: S) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set the stroke width
    pub fn stroke_width<S: Into<String>>(mut self, width: S) -> Self {
        self.stroke_width = Some(width.into());
        self
    }

    /// Set the stroke color
    pub fn stroke<S: Into<String>>(mut self, stroke: S) -> Self {
        self.stroke = Some(stroke.into());
        self
    }

    /// Set the fill color
    pub fn fill<S: Into<String>>(mut self, fill: S) -> Self {
        self.fill = Some(fill.into());
        self
    }
}

/// Helper component for rendering icons with configuration (enhanced version)
#[component]
pub fn Icon<F, V>(
    /// The icon component function
    icon: F,

    /// Optional configuration
    #[prop(optional)]
    config: Option<IconConfig>,

    /// Set wrapper element (defaults to "div")
    #[prop(optional)]
    wrapper: Option<String>,
) -> leptos::prelude::AnyView
where
    F: Fn() -> V + 'static,
    V: IntoView,
{
    let config = config.unwrap_or_default();
    let wrapper_tag = wrapper.unwrap_or_else(|| "div".to_string());

    let mut wrapper_class = "lucide-wrapper".to_string();
    if let Some(ref class) = config.class {
        wrapper_class.push(' ');
        wrapper_class.push_str(class);
    }

    let mut wrapper_style = String::new();
    if let Some(ref size) = config.size {
        wrapper_style.push_str(&format!("width: {size}; height: {size};"));
    }
    if let Some(ref style) = config.style {
        if !wrapper_style.is_empty() {
            wrapper_style.push(' ');
        }
        wrapper_style.push_str(style);
    }

    // Create the wrapper with custom tag

    match wrapper_tag.as_str() {
        "span" => view! {
            <span
                class=wrapper_class.clone()
                style=move || if wrapper_style.is_empty() {
                    None
                } else {
                    Some(wrapper_style.clone())
                }
            >
                {icon()}
            </span>
        }
        .into_any(),

        "button" => view! {
            <button
                class=wrapper_class.clone()
                style=move || if wrapper_style.is_empty() {
                    None
                } else {
                    Some(wrapper_style.clone())
                }
            >
                {icon()}
            </button>
        }
        .into_any(),

        _ => view! {
            <div
                class=wrapper_class.clone()
                style=move || if wrapper_style.is_empty() {
                    None
                } else {
                    Some(wrapper_style.clone())
                }
            >
                {icon()}
            </div>
        }
        .into_any(),
    }
}

// #[component]
// pub fn Icon<F>(
//     /// The icon component function
//     icon: F,
//     /// Optional configuration
//     #[prop(optional)]
//     config: Option<IconConfig>,
//     /// Set wrapper element (defaults to "div")
//     #[prop(optional)]
//     wrapper: Option<String>,
// ) -> impl IntoView
// where
//     F: Fn() -> impl IntoView + 'static,
// {
//     let config = config.unwrap_or_default();
//     let wrapper_tag = wrapper.unwrap_or_else(|| "div".to_string());
//
//     let mut wrapper_class = "lucide-wrapper".to_string();
//     if let Some(ref class) = config.class {
//         wrapper_class.push(' ');
//         wrapper_class.push_str(class);
//     }
//
//     let mut wrapper_style = String::new();
//     if let Some(ref size) = config.size {
//         wrapper_style.push_str(&format!("width: {}; height: {};", size, size));
//     }
//     if let Some(ref style) = config.style {
//         if !wrapper_style.is_empty() {
//             wrapper_style.push(' ');
//         }
//         wrapper_style.push_str(style);
//     }
//
//     // Create the wrapper with custom tag
//     match wrapper_tag.as_str() {
//         "span" => view! {
//             <span
//                 class=&wrapper_class
//                 style=move || if wrapper_style.is_empty() { None } else { Some(wrapper_style.clone()) }
//             >
//                 {icon()}
//             </span>
//         }.into_view(),
//         "button" => view! {
//             <button
//                 class=&wrapper_class
//                 style=move || if wrapper_style.is_empty() { None } else { Some(wrapper_style.clone()) }
//             >
//                 {icon()}
//             </button>
//         }.into_view(),
//         _ => view! {
//             <div
//                 class=&wrapper_class
//                 style=move || if wrapper_style.is_empty() { None } else { Some(wrapper_style.clone()) }
//             >
//                 {icon()}
//             </div>
//         }.into_view(),
//     }
// }

/// Utility macro for quick icon creation with builder pattern (enhanced)
#[macro_export]
macro_rules! icon {
    ($icon:ident) => {
        $icon()
    };
    ($icon:ident, $($method:ident($value:expr)),+ $(,)?) => {{
        let config = leptos_lucide_rs::IconConfig::new()$(.$method($value))+;
        leptos_lucide_rs::Icon(|| $icon(), Some(config), None)
    }};
    ($icon:ident, wrapper = $wrapper:expr, $($method:ident($value:expr)),+ $(,)?) => {{
        let config = leptos_lucide_rs::IconConfig::new()$(.$method($value))+;
        leptos_lucide_rs::Icon(|| $icon(), Some(config), Some($wrapper.to_string()))
    }};
}

/// Simple macro to create dynamic icons by using the load_icon function
#[macro_export]
macro_rules! dynamic_icon {
    ($name:expr) => {
        #[cfg(leptos_lucide_generated)]
        load_icon($name)

        #[cfg(not(leptos_lucide_generated))]
        fallback::load_icon($name)
    };
}

/// Convenient alias for dynamic icons
pub use dynamic_icon as dyn_icon;
