# leptos-lucide-rs

[![Rust](https://github.com/soulcorrea/leptos-lucide-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/soulcorrea/leptos-lucide-rs/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/leptos-lucide-rs.svg)](https://crates.io/crates/leptos-lucide-rs)
[![Documentation](https://docs.rs/hashbrown/badge.svg)](https://docs.rs/leptos-lucide-rs)
[![Rust](https://img.shields.io/badge/rust-1.88.0%2B-blue.svg?maxAge=3600)](https://github.com/rust-lang/leptos-lucide-rs)

A comprehensive Lucide icon library for Leptos with tree-shaking support and
zero-cost runtime overhead.

<img width="800" height="533" alt="leptos-lucide-800x600" src="https://github.com/user-attachments/assets/973379f2-c049-47f2-9891-242665c3c609" />

## 🌟 Features

- **🌳 Tree-shaking**: Only bundle the icons you actually use
- **⚡ Zero-cost abstractions**: All icon components are `#[inline(always)]`
  for maximum performance
- **🚀 Lazy loading**: SVG content is only generated when components are
  actually rendered
- **🦀 Rust-friendly**: Generated component names follow Rust conventions and
  avoid naming conflicts
- **🔒 Type-safe**: Each icon is a separate typed component with full IDE
  support
- **🎨 Customizable**: Easy styling with CSS classes, inline styles, and
  configuration
- **📦 Latest dependencies**: Uses `lucide-svg-rs` for official icon data
- **🔄 Dynamic loading**: Load any icon by name at runtime
- **📈 Build performance**: Fast compilation with efficient code generation

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
leptos-lucide-rs = "0.1"
leptos = "0.8"
lucide-svg-rs = "0.1"  # Used internally for lazy loading
```

## 🚀 Quick Start

```rust
use leptos::*;
use leptos_lucide::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex items-center gap-4">
            <Home/>
            <User/>
            <Heart/>
            <Search/>
            <Star/>
        </div>
    }
}
```

## 🎨 Styling Icons

### Basic Usage

```rust
use leptos::*;
use leptos_lucide::*;

#[component]
pub fn BasicIcons() -> impl IntoView {
    view! {
        <div>
            // Default icon
            <Home/>

            // With CSS classes
            <div class="text-blue-500 w-6 h-6">
                <User/>
            </div>
        </div>
    }
}
```

### Using the `lucide_icon!` Macro

```rust
use leptos::*;
use leptos_lucide::*;

#[component]
pub fn MacroExamples() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            // Basic icon
            {lucide_icon!(Home)}

            // With custom class
            {lucide_icon!(User, class = "text-red-500 w-8 h-8")}

            // With custom size
            {lucide_icon!(Heart, size = "32px")}
        </div>
    }
}
```

### Using Dynamic Icons

```rust
use leptos::*;
use leptos_lucide::*;

#[component]
pub fn DynamicIcons() -> impl IntoView {
    let (icon_name, set_icon_name) = create_signal("home".to_string());

    view! {
        <div class="space-y-4">
            <h2>"Dynamic Icon Loading"</h2>

            // Load icon by name at runtime
            <DynamicIcon name=icon_name()/>

            // With styling
            <DynamicIcon
                name="search"
                class=Some("text-blue-500".to_string())
                size=Some("32px".to_string())
            />

            // Using the macro
            {dynamic_icon!("heart", class = "text-red-500")}
            {dyn_icon!("star", size = "24px")}  // Alias for dynamic_icon!

            // Interactive selection
            <div class="flex gap-2">
                <button on:click=move |_| set_icon_name("home".to_string())>
                    "Home"
                </button>
                <button on:click=move |_| set_icon_name("user".to_string())>
                    "User"
                </button>
                <button on:click=move |_| set_icon_name("heart".to_string())>
                    "Heart"
                </button>
            </div>
        </div>
    }
}
```

### Using the `Icon` Component

```rust
use leptos::*;
use leptos_lucide::*;

#[component]
pub fn ConfigurableIcons() -> impl IntoView {
    view! {
        <div class="grid grid-cols-3 gap-4">
            // Using Icon component with configuration
            <Icon
                icon=|| Home()
                config=Some(IconConfig::new()
                    .class("text-green-500")
                    .size("24px")
                    .stroke_width("1.5")
                )
            />

            // With custom wrapper element
            <Icon
                icon=|| Search()
                config=Some(IconConfig::new()
                    .class("hover:text-blue-500 transition-colors")
                    .size("20px")
                )
                wrapper=Some("button".to_string())
            />

            // Multiple configurations
            <Icon
                icon=|| Heart()
                config=Some(IconConfig::new()
                    .class("text-red-500 cursor-pointer")
                    .stroke("currentColor")
                )
                wrapper=Some("span".to_string())
            />
        </div>
    }
}
```

### Using the Builder-Pattern `icon!` Macro

```rust
use leptos::*;
use leptos_lucide::*;

#[component]
pub fn BuilderIcons() -> impl IntoView {
    view! {
        <div class="flex items-center space-x-4">
            // Simple usage
            {icon!(Home)}

            // With builder methods
            {icon!(User,
                class("text-purple-500"),
                size("28px"),
                stroke_width("2")
            )}

            // With custom wrapper
            {icon!(Heart,
                wrapper = "button",
                class("text-red-500 hover:text-red-600 cursor-pointer"),
                size("32px"),
                style("transition: color 0.2s;")
            )}
        </div>
    }
}
```

## 🔧 Advanced Usage

### Custom CSS

Add custom styles for your icons:

```css
.lucide-icon {
  /* Default icon styles */
  transition: all 0.2s ease-in-out;
}

.lucide-icon:hover {
  transform: scale(1.1);
}

.lucide-wrapper {
  /* Wrapper styles for positioning */
  display: inline-flex;
  align-items: center;
  justify-content: center;
}
```

### Conditional Icons

```rust
use leptos::*;
use leptos_lucide::*;

#[component]
pub fn ConditionalIcon() -> impl IntoView {
    let (is_liked, set_liked) = create_signal(false);

    view! {
        <button
            on:click=move |_| set_liked.update(|liked| *liked = !*liked)
            class="p-2 rounded hover:bg-gray-100"
        >
            {move || if is_liked() {
                icon!(Heart,
                    class("text-red-500 fill-current"),
                    size("24px")
                )
            } else {
                icon!(Heart,
                    class("text-gray-400"),
                    size("24px")
                )
            }}
        </button>
    }
}
```

### Icon Lists and Iterations

```rust
use leptos::*;
use leptos_lucide::*;

#[component]
pub fn IconGrid() -> impl IntoView {
    // Note: In real usage, you'd import the specific icons you need
    // This is just for demonstration

    view! {
        <div class="grid grid-cols-4 gap-4 p-4">
            <div class="flex flex-col items-center p-4 border rounded">
                {icon!(Home, class("text-blue-500"), size("32px"))}
                <span class="text-sm mt-2">Home</span>
            </div>

            <div class="flex flex-col items-center p-4 border rounded">
                {icon!(User, class("text-green-500"), size("32px"))}
                <span class="text-sm mt-2">User</span>
            </div>

            <div class="flex flex-col items-center p-4 border rounded">
                {icon!(Heart, class("text-red-500"), size("32px"))}
                <span class="text-sm mt-2">Heart</span>
            </div>

            <div class="flex flex-col items-center p-4 border rounded">
                {icon!(Search, class("text-purple-500"), size("32px"))}
                <span class="text-sm mt-2">Search</span>
            </div>
        </div>
    }
}
```

## 📋 Available Icons

This library includes all Lucide icons with Rust-friendly names. Some examples:

- `Home` (from `home`)
- `User` (from `user`)
- `Heart` (from `heart`)
- `Search` (from `search`)
- `Star` (from `star`)
- `ChevronLeft` (from `chevron-left`)
- `ArrowRight` (from `arrow-right`)
- `PlusCircle` (from `plus-circle`)
- `MinusIcon` (from `minus` - renamed to avoid conflict with Rust's minus operator)
- `TypeIcon` (from `type` - renamed to avoid conflict with Rust's `type` keyword)

### Naming Convention

The library automatically converts kebab-case icon names to PascalCase Rust
component names:

- `arrow-left` → `ArrowLeft`
- `chevron-down` → `ChevronDown`
- `plus-circle` → `PlusCircle`
- `x-circle` → `XCircle`

### Conflict Resolution

When icon names would conflict with Rust keywords or common types, the library
automatically renames them:

- `type` → `TypeIcon`
- `box` → `BoxIcon`
- `move` → `MoveIcon`
- `ref` → `RefIcon`

If multiple icons would generate the same name, numbers are appended:

- First occurrence: `Icon`
- Second occurrence: `Icon2`
- Third occurrence: `Icon3`

## 🏗️ Build Process

The library uses `lucide-svg-rs` and an intelligent build script that:

1. **Fetches icon metadata** from the `lucide-svg-rs` crate
2. **Generates typed component signatures** for each icon at build time
3. **Implements lazy loading** - SVG content is generated only when components
   are used
4. **Ensures unique names** by handling Rust keyword conflicts
5. **Optimizes for performance** with `#[inline(always)]` annotations
6. **Provides fallbacks** for development when icons aren't generated yet

### Build-time Features

- **Fast compilation**: Generated components are lightweight function signatures
- **rust-analyzer friendly**: Works seamlessly with IDE tooling and autocomplete
- **Tree-shaking ready**: Only used icons affect bundle size
- **Lazy content loading**: SVG paths loaded from `lucide-svg-rs` at render time
- **Dynamic capabilities**: Can load any Lucide icon by name at runtime

## 🔧 Configuration

### Cargo Features

```toml
[dependencies.leptos-lucide-rs]
version = "0.1"
features = ["ssr"]  # For server-side rendering
# features = ["hydrate"]  # For hydration
# features = ["csr"]  # For client-side rendering only
```

### Custom Build Configuration

You can customize the build process by setting environment variables:

```bash
# Skip downloading and use cached icons
LUCIDE_OFFLINE=1 cargo build

# Use a specific Lucide version
LUCIDE_VERSION=v0.263.0 cargo build
```

## 🚀 Performance

### Zero-Cost Abstractions

All icon components are marked with `#[inline(always)]`, ensuring they have
zero call overhead in release builds:

```rust
#[inline(always)]
#[allow(non_snake_case)]
pub fn Home() -> impl leptos::IntoView {
    // Implementation is inlined directly at call site
}
```

### Tree Shaking

Only the icons you import and use will be included in your final bundle:

```rust
// Only Home and User icons will be bundled
use leptos_lucide::{Home, User};

// Not bundled - never imported
// use leptos_lucide::Heart;
```

### Performance Benefits

- **Lazy SVG generation**: Icon content is only created when the component renders
- **Efficient builds**: Build script only generates component signatures, not
  full content
- **Runtime flexibility**: Can load any icon dynamically without affecting
  bundle size
- **Memory efficient**: SVG data isn't stored in memory until needed

- **Minimal overhead**: Each icon adds ~1-2KB to your bundle
- **SVG optimization**: Icons use optimized SVG paths
- **No runtime dependencies**: Pure compile-time generation

## 🧪 Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_icon_rendering() {
        let runtime = create_runtime();

        let view = Home();
        // Test that the icon renders without panicking
        assert!(view.into_view().is_some());

        runtime.dispose();
    }

    #[test]
    fn test_icon_with_config() {
        let runtime = create_runtime();

        let config = IconConfig::new()
            .class("test-class")
            .size("24px");

        let view = Icon(|| Home(), Some(config));
        // Test configured icon rendering
        assert!(view.into_view().is_some());

        runtime.dispose();
    }
}
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/soulcorrea/leptos-lucide-rs
cd leptos-lucide

# Build the library (this will download icons and generate components)
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Lucide](https://lucide.dev/) for the beautiful icon set
- [Leptos](https://github.com/leptos-rs/leptos) for the reactive framework
- The Rust community for amazing tooling

## 📚 Examples

Check out the [examples](examples/) directory for more usage patterns:

- **Basic Usage**: Simple icon rendering
- **Styling Examples**: Various styling approaches
- **Interactive Icons**: Icons with state and interactions
- **Performance Demo**: Tree-shaking demonstration
- **SSR Example**: Server-side rendering with icons

## 🔗 Related Projects

- [leptos](https://github.com/leptos-rs/leptos) - The reactive web framework this library is built for
- [lucide](https://lucide.dev/) - The source of the beautiful icons
- [tauri-icons](https://github.com/tauri-apps/tauri-icons) - Similar icon library for Tauri apps
