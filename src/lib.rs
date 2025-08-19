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
//! use leptos_lucide_tree::*;
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
//! use leptos_lucide_tree::*;
//!
//! #[component]
//! pub fn StyledIcons() -> impl IntoView {
//!     view! {
//!         <div>
//!             // Using the macro for custom classes
//!             {leptos_lucide_icon!(Home, class = "text-blue-500 w-6 h-6")}
//!
//!             // Using the macro for custom sizes
//!             {leptos_lucide_icon!(User, size = "32px")}
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
#[cfg(leptos_lucide_tree_generated)]
include!(concat!(env!("OUT_DIR"), "/icons.rs"));

// Fallback module when icons aren't generated yet (for IDE support)
#[cfg(not(leptos_lucide_tree_generated))]
pub mod fallback {
    use leptos::*;

    /// Fallback Home icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn Home() -> impl IntoView {
        view! {
            <svg
                class="leptos-lucide-icon"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                data-leptos-lucide="home"
                inner_html=r#"<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path><polyline points="9,22 9,12 15,12 15,22"></polyline>"#
            >
            </svg>
        }
    }

    /// Fallback User icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn User() -> impl IntoView {
        view! {
            <svg
                class="leptos-lucide-icon"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                data-leptos-lucide="user"
                inner_html=r#"<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle>"#
            >
            </svg>
        }
    }

    /// Fallback Heart icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn Heart() -> impl IntoView {
        view! {
            <svg
                class="leptos-lucide-icon"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                data-leptos-lucide="heart"
                inner_html=r#"<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path>"#
            >
            </svg>
        }
    }

    /// Fallback Search icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn Search() -> impl IntoView {
        view! {
            <svg
                class="leptos-lucide-icon"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                data-leptos-lucide="search"
                inner_html=r#"<circle cx="11" cy="11" r="8"></circle><path d="M21 21l-4.35-4.35"></path>"#
            >
            </svg>
        }
    }

    /// Fallback Star icon for development
    #[inline(always)]
    #[allow(non_snake_case)]
    pub fn Star() -> impl IntoView {
        view! {
            <svg
                class="leptos-lucide-icon"
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                data-leptos-lucide="star"
                inner_html=r#"<polygon points="12,2 15.09,8.26 22,9.27 17,14.14 18.18,21.02 12,17.77 5.82,21.02 7,14.14 2,9.27 8.91,8.26"></polygon>"#
            >
            </svg>
        }
    }

    /// Fallback icon count for development
    pub const ICON_COUNT: usize = 5;
}

// Re-export fallback icons when generated ones aren't available
#[cfg(not(leptos_lucide_tree_generated))]
pub use fallback::*;

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

/// Utility macro for quick icon creation
#[macro_export]
macro_rules! icon {
    ($icon:ident) => {
        $icon()
    };
}
