use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(leptos_lucide_generated)");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("icons.rs");

    // Get list of available icon names
    let icon_names = get_available_icon_names();

    // Generate icon components
    let generated_code = generate_icon_components(&icon_names);

    // Write to file
    fs::write(&dest_path, generated_code.to_string()).expect("Failed to write generated icons");

    println!("cargo:rustc-cfg=leptos_lucide_generated");
    println!("Generated {} icon components", icon_names.len());
}

/// Get a deduplicated list of common Lucide icon names
fn get_available_icon_names() -> Vec<String> {
    let mut unique_names = HashSet::new();

    let client = lucide_svg_rs::LucideClient::default();
    let mut names: Vec<String> = Vec::new();
    if let Ok((_, n)) = client.download_all_icons(lucide_svg_rs::ICONS_DIR) {
        names.extend(n)
    } else {
        names.extend(
            vec![
                // Navigation & UI
                "home",
                "user",
                "settings",
                "menu",
                "search",
                "filter",
                "more-horizontal",
                "more-vertical",
                // Arrows & Directions
                "arrow-left",
                "arrow-right",
                "arrow-up",
                "arrow-down",
                "chevron-left",
                "chevron-right",
                "chevron-up",
                "chevron-down",
                "arrow-up-right",
                "arrow-down-left",
                "corner-down-left",
                // Actions
                "plus",
                "minus",
                "x",
                "check",
                "edit",
                "trash",
                "copy",
                "download",
                "upload",
                "share",
                "external-link",
                // Content & Media
                "file",
                "folder",
                "image",
                "video",
                "music",
                "camera",
                "mic",
                "volume-2",
                "play",
                "pause",
                "skip-forward",
                "skip-back",
                // Communication
                "mail",
                "message-circle",
                "phone",
                "bell",
                "heart",
                "thumbs-up",
                "star",
                // Status & Feedback
                "alert-circle",
                "alert-triangle",
                "info",
                "help-circle",
                "check-circle",
                "x-circle",
                "loader",
                "refresh-cw",
                // Tools & Objects
                "calendar",
                "clock",
                "map-pin",
                "shopping-cart",
                "credit-card",
                "key",
                "lock",
                "unlock",
                "shield",
                "eye",
                "eye-off",
                // Text & Formatting
                "type",
                "bold",
                "italic",
                "underline",
                "align-left",
                "align-center",
                "align-right",
                // Shapes & Graphics
                "circle",
                "square",
                "triangle",
                "hexagon",
                "sun",
                "moon",
                "cloud",
                "zap",
                // Social & Branding
                "github",
                "twitter",
                "linkedin",
                "instagram",
                "facebook",
                "youtube",
                "twitch",
                // Business & Finance
                "briefcase",
                "building",
                "trending-up",
                "trending-down",
                "pie-chart",
                "bar-chart",
                "activity",
                // Development & Code
                "code",
                "terminal",
                "git-branch",
                "git-commit",
                "git-merge",
                "database",
                "server",
                "wifi",
                // Layout & Design
                "layout",
                "sidebar",
                "panels",
                "maximize",
                "minimize",
                "move",
                "rotate-cw",
                "flip-horizontal",
            ]
            .into_iter()
            .map(|s| s.to_owned()),
        );
    }

    // Add to HashSet to remove duplicates
    for name in names {
        unique_names.insert(name.to_string());
    }

    // Convert back to Vec and sort for consistent ordering
    let mut result: Vec<String> = unique_names.into_iter().collect();
    result.sort();
    result
}

fn generate_icon_components(icon_names: &[String]) -> TokenStream {
    let mut components = Vec::new();

    for name in icon_names {
        let component_name = to_component_name(name);
        let component_ident = Ident::new(&component_name, Span::call_site());

        let component = quote! {
            #[inline(always)]
            #[allow(non_snake_case)]
            pub fn #component_ident() -> impl leptos::IntoView {
                use leptos::*;

                let svg_content = move || {
                    let client = lucide_svg_rs::LucideClient::default();
                    match client.get_icon_content(&format!("{}.svg", #name)) {
                        Ok(svg_content) => {
                            // Extract content between <svg> tags
                            if let (Some(start), Some(end)) = (svg_content.find('>'), svg_content.rfind("</svg>")) {
                                if start < end {
                                    return svg_content[start + 1..end].to_string();
                                }
                            }
                            // Fallback if parsing fails
                            r#"<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path>"#.to_string()
                        }
                        Err(_) => {
                            // Fallback icon
                            r#"<path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path>"#.to_string()
                        }
                    }
                };

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
                        data-lucide=#name
                        inner_html=svg_content
                    />
                }
            }
        };

        components.push(component);
    }

    // Generate the load_icon function
    let load_icon_match_arms = icon_names.iter().map(|name| {
        let component_name = to_component_name(name);
        let component_ident = Ident::new(&component_name, Span::call_site());
        quote! {
            #name => #component_ident().into_any(),
        }
    });

    let load_icon_function = quote! {
        /// Load an icon by name at runtime
        pub fn load_icon(name: &str) -> leptos::prelude::AnyView {
            match name {
                #(#load_icon_match_arms)*
                _ => {
                    // Fallback for unknown icons
                    let client = lucide_svg_rs::LucideClient::default();
                    match client.get_icon_content(&format!("{}.svg", name)) {
                        Ok(svg_content) => {
                            if let (Some(start), Some(end)) = (svg_content.find('>'), svg_content.rfind("</svg>")) {
                                if start < end {
                                    let inner = svg_content[start + 1..end].to_string();
                                    return view! {
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
                                            inner_html=move || inner.clone()
                                        />
                                    }.into_any();
                                }
                            }
                        }
                        Err(_) => {}
                    }

                    // Ultimate fallback
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
                        >
                            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
                        </svg>
                    }.into_any()
                }
            }
        }
    };

    quote! {
        // Generated icon components
        #(#components)*

        #load_icon_function
    }
}

fn to_component_name(icon_name: &str) -> String {
    // Convert kebab-case to PascalCase
    icon_name.to_case(Case::Pascal)
}
