# leptos-lucide-tree

A comprehensive Lucide icon library for Leptos with tree-shaking support and
zero-cost runtime overhead.

## 🌟 Features

- **🌳 Tree-shaking**: Only bundle the icons you actually use
- **⚡ Zero-cost**: All icon components are `#[inline(always)]` for maximum
  performance
- **🦀 Rust-friendly**: Generated component names follow Rust conventions and
  avoid naming conflicts
- **🔒 Type-safe**: Each icon is a separate typed component with full IDE
  support
- **🎨 Customizable**: Easy styling with CSS classes, inline styles, and
  configuration
- **📦 Latest dependencies**: Uses the latest stable versions of all dependency
  crates
- **🔄 Auto-generated**: Icons are generated at build time from the latest
  Lucide icon set

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
leptos-lucide-tree = "0.1.4"
leptos = "0.8.6"
```

## 🚀 Quick Start

```rust
use leptos::*;
use leptos_lucide_tree::*;

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
use leptos_lucide_tree::*;

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

### Using the `leptos_lucide_icon!` Macro

```rust
use leptos::*;
use leptos_lucide_tree::*;

#[component]
pub fn MacroExamples() -> impl IntoView {
    view! {
        <div class="flex gap-4">
            // Basic icon
            {leptos_lucide_icon!(Home)}

            // With custom class
            {leptos_lucide_icon!(User, class = "text-red-500 w-8 h-8")}

            // With custom size
            {leptos_lucide_icon!(Heart, size = "32px")}
        </div>
    }
}
```

### Using the `Icon` Component

```rust
use leptos::*;
use leptos_lucide_tree::*;

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
                    .stroke_width("1.5"))
            />

            // Multiple configurations
            <Icon
                icon=|| Search()
                config=Some(IconConfig::new()
                    .class("hover:text-blue-500 transition-colors")
                    .size("20px")
                    .stroke("currentColor"))
            />
        </div>
    }
}
```

### Using the Builder-Pattern `icon!` Macro

```rust
use leptos::*;
use leptos_lucide_tree::*;

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

            // Complex styling
            {icon!(Heart,
                class("text-red-500 hover:text-red-600"),
                size("32px"),
                stroke_width("1.5"),
                style("cursor: pointer; transition: color 0.2s;")
            )}
        </div>
    }
}
```

## 🔧 Advanced Usage

### Custom CSS

Add custom styles for your icons:

```css
.leptos-lucide-icon {
  /* Default icon styles */
  transition: all 0.2s ease-in-out;
}

.leptos-lucide-icon:hover {
  transform: scale(1.1);
}

.leptos-lucide-wrapper {
  /* Wrapper styles for positioning */
  display: inline-flex;
  align-items: center;
  justify-content: center;
}
```

### Conditional Icons

```rust
use leptos::*;
use leptos_lucide_tree::*;

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
use leptos_lucide_tree::*;

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
- `MinusIcon` (from `minus` - renamed to avoid conflict with Rust's minus
  operator)
- `TypeIcon` (from `type` - renamed to avoid conflict with Rust's `type`
  keyword)

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

The library uses a build script that:

1. **Downloads latest Lucide icons** from the official repository
2. **Generates typed components** for each icon at build time
3. **Ensures unique names** by handling Rust keyword conflicts
4. **Optimizes for performance** with `#[inline(always)]` annotations
5. **Provides fallbacks** for development when icons aren't generated yet

### Build-time Features

- **Fast compilation**: Generated code is optimized for quick builds
- **rust-analyzer friendly**: Works seamlessly with IDE tooling
- **Tree-shaking ready**: Only used icons are included in the final bundle
- **Zero runtime overhead**: All components are inlined

## 🔧 Configuration

### Cargo Features

```toml
[dependencies.leptos-lucide-tree]
version = "0.1.4"
features = ["ssr"]  # For server-side rendering
# features = ["hydrate"]  # For hydration
# features = ["csr"]  # For client-side rendering only
```

### Custom Build Configuration

You can customize the build process by setting environment variables:

````bash # Skip downloading and use cached icons LUCIDE_OFFLINE=1 cargo build

# Use a specific Lucide version LUCIDE_VERSION=v0.263.0 cargo build ```

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
````

### Tree Shaking

Only the icons you import and use will be included in your final bundle:

```rust
// Only Home and User icons will be bundled
use leptos_lucide_tree::{Home, User};

// Not bundled - never imported
// use leptos_lucide_tree::Heart;
```

### Bundle Size Optimization

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

This project uses [Just](https://github.com/casey/just) as a command runner for development tasks. Install it first:

```bash
# Install Just (command runner)
cargo install just

# Clone the repository
git clone https://github.com/soulcorrea/leptos-lucide-tree
cd leptos-lucide-tree

# Setup development environment (installs tools and dependencies)
just setup

# See all available commands
just help
```

### Development Commands

#### Building and Testing

```bash
# Build the library (downloads icons and generates components)
just build

# Run all tests
just test

# Run tests with verbose output
just test-verbose

# Build examples and verify they compile
just test-examples
```

#### Code Quality

```bash
# Format code
just fmt

# Run clippy linter
just clippy

# Run all linting tools
just lint

# Run all pre-commit checks (fmt, clippy, test, targets)
just pre-commit
```

#### Examples

```bash
# Build all examples
just examples

# Run simple example (opens browser at localhost:8080)
just run-simple

# Run advanced styling example (opens browser at localhost:8081)
just run-advanced

# Build examples for production
just build-simple
just build-advanced
```

#### Development Workflow

```bash
# Start development server for simple example
just dev

# Start development server for advanced example
just dev-advanced

# Watch for changes and rebuild automatically
just watch

# Watch for changes and run tests automatically
just watch-test

# Quick development cycle (build + test)
just dev-cycle

# Full development workflow (clean + build + test + examples)
just dev-full
```

#### Icon Management

```bash
# Force regeneration of icon components
just generate-icons

# Get list of available icons
just icon-list

# Validate generated icon code
just validate
```

#### Documentation

```bash
# Generate and open documentation
just docs

# Generate docs including private items
just docs-private
```

#### Maintenance

```bash
# Clean build artifacts
just clean

# Update dependencies
just update

# Check for outdated dependencies (requires cargo-outdated)
just outdated

# Audit dependencies for security issues (requires cargo-audit)
just audit

# Generate test coverage report (requires cargo-tarpaulin)
just coverage
```

#### Release

```bash
# Prepare for release (runs all checks)
just release-prep

# Release patch version (requires cargo-release)
just release-patch

# Release minor version (requires cargo-release)
just release-minor

# Release major version (requires cargo-release)
just release-major
```

#### Verification

```bash
# Verify library works correctly (build + test + examples)
just verify

# Check build on multiple targets
just check-targets

# Run performance tests
just perf
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.

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

- [leptos](https://github.com/leptos-rs/leptos) - The reactive web framework
  this library is built for
- [lucide](https://lucide.dev/) - The source of the beautiful icons
- [tauri-icons](https://github.com/tauri-apps/tauri-icons) - Similar icon
  library for Tauri apps

```

```
