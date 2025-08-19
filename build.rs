use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct LucideIconData {
    #[serde(flatten)]
    icons: HashMap<String, Vec<String>>,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(leptos_lucide_tree_generated)");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("icons.rs");

    // Download or use cached Lucide icons data
    let icons_data = fetch_lucide_icons();

    // Generate icon components
    let generated_code = generate_icon_components(&icons_data);

    // Format the code using prettyplease
    let syntax_tree = syn::parse2(generated_code).expect("Failed to parse generated code");
    let formatted_code = prettyplease::unparse(&syntax_tree);

    // Write to file
    fs::write(&dest_path, formatted_code).expect("Failed to write generated icons");

    println!("cargo:rustc-cfg=leptos_lucide_tree_generated");
}

fn fetch_lucide_icons() -> HashMap<String, String> {
    // Try to fetch from Lucide's official icon data
    let url = "https://api.github.com/repos/lucide-icons/lucide/contents/icons";

    match reqwest::blocking::get(url) {
        Ok(response) => {
            if let Ok(text) = response.text() {
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                    return parse_lucide_data(data);
                }
            }
        }
        Err(_) => eprintln!("Warning: Could not fetch latest Lucide icons, using fallback"),
    }

    // Fallback to some common icons if download fails
    get_fallback_icons()
}

fn parse_lucide_data(data: serde_json::Value) -> HashMap<String, String> {
    let mut icons = HashMap::new();

    if let Some(obj) = data.as_object() {
        for (name, icon_data) in obj {
            if let Some(svg_content) = extract_svg_content(icon_data) {
                dbg!(&svg_content);
                icons.insert(name.clone(), svg_content);
            }
        }
    }

    icons
}

fn extract_svg_content(icon_data: &serde_json::Value) -> Option<String> {
    // Extract SVG path data from the icon data structure
    // This depends on Lucide's JSON structure
    if let Some(svg) = icon_data.get("svg") {
        if let Some(svg_str) = svg.as_str() {
            return Some(svg_str.to_string());
        }
    }

    // Alternative: construct from paths if available
    if let Some(paths) = icon_data.get("paths") {
        if let Some(paths_array) = paths.as_array() {
            let path_elements: Vec<String> = paths_array
                .iter()
                .filter_map(|p| p.as_str())
                .map(|p| format!(r#"<path d="{p}"></path>"#))
                .collect();

            return Some(path_elements.join(""));
        }
    }

    None
}

fn get_fallback_icons() -> HashMap<String, String> {
    let mut icons = HashMap::new();

    // Add some common icons as fallback
    icons.insert("home".to_string(), r#"<path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path><polyline points="9,22 9,12 15,12 15,22"></polyline>"#.to_string());
    icons.insert("user".to_string(), r#"<path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path><circle cx="12" cy="7" r="4"></circle>"#.to_string());
    icons.insert("heart".to_string(), r#"<path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"></path>"#.to_string());
    icons.insert("star".to_string(), r#"<polygon points="12,2 15.09,8.26 22,9.27 17,14.14 18.18,21.02 12,17.77 5.82,21.02 7,14.14 2,9.27 8.91,8.26"></polygon>"#.to_string());
    icons.insert(
        "search".to_string(),
        r#"<circle cx="11" cy="11" r="8"></circle><path d="M21 21l-4.35-4.35"></path>"#.to_string(),
    );

    icons
}

fn generate_icon_components(icons: &HashMap<String, String>) -> TokenStream {
    let mut used_names = HashSet::new();
    let mut component_tokens = Vec::new();

    for (icon_name, svg_content) in icons {
        let component_name = to_unique_camel_case(icon_name, &mut used_names);
        let component_ident = Ident::new(&component_name, Span::call_site());
        let kebab_name = icon_name.to_case(Case::Kebab);

        let component = quote! {
            #[inline(always)]
            #[allow(non_snake_case)]
            pub fn #component_ident() -> impl leptos::IntoView {
                leptos::view! {
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
                        data-leptos-lucide={#kebab_name}
                        inner_html={#svg_content}
                    >
                    </svg>
                }
            }
        };

        component_tokens.push(component);
    }

    let icon_count = icons.len();

    quote! {
        use leptos::prelude::*;

        // Auto-generated Leptos Lucide icons
        //
        // This module contains typed components for each Leptos Lucide icon.
        // Each component is inlined for zero-cost abstractions.

        #(#component_tokens)*

        /// Get the total number of available icons
        pub const ICON_COUNT: usize = #icon_count;

        /// Macro for creating icon components with custom classes
        #[macro_export]
        macro_rules! leptos_lucide_icon {
            ($icon:ident) => {
                $icon()
            };
            ($icon:ident, class = $class:expr) => {
                leptos::view! {
                    <div class={format!("leptos-lucide-wrapper {}", $class)}>
                        {$icon()}
                    </div>
                }
            };
            ($icon:ident, size = $size:expr) => {
                leptos::view! {
                    <div style={format!("width: {}; height: {};", $size, $size)}>
                        {$icon()}
                    </div>
                }
            };
        }
    }
}

fn to_unique_camel_case(input: &str, used_names: &mut HashSet<String>) -> String {
    // Convert to PascalCase and handle Rust keyword conflicts
    let base_name = sanitize_rust_name(&input.to_case(Case::Pascal));

    // Ensure uniqueness
    let mut name = base_name.clone();
    let mut counter = 2;

    while used_names.contains(&name) {
        name = format!("{base_name}{counter}");
        counter += 1;
    }

    used_names.insert(name.clone());
    name
}

fn sanitize_rust_name(name: &str) -> String {
    // Handle Rust keywords and reserved words
    let rust_keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "try", "union", "abstract", "become",
        "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    ];

    // Handle common Rust types that might conflict
    let rust_types = [
        "String", "Vec", "Option", "Result", "Box", "Arc", "Rc", "Cell", "RefCell", "HashMap",
        "HashSet", "BTreeMap", "BTreeSet", "Path", "PathBuf", "File", "Error", "Ok", "Err", "Some",
        "None",
    ];

    let mut sanitized = name.to_string();

    // Ensure it starts with a letter or underscore
    if !sanitized.chars().next().unwrap_or('_').is_alphabetic() && !sanitized.starts_with('_') {
        sanitized = format!("Icon{sanitized}");
    }

    // Handle keywords
    if rust_keywords.contains(&sanitized.as_str()) || rust_types.contains(&sanitized.as_str()) {
        sanitized = format!("{sanitized}Icon");
    }

    // Replace any invalid characters
    sanitized = sanitized
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();

    // Ensure it doesn't end with numbers that might look confusing
    if sanitized.ends_with(char::is_numeric) {
        sanitized.push_str("Icon");
    }

    sanitized
}
