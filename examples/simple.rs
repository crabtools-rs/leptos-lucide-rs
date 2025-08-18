use leptos::prelude::*;
use leptos_lucide::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"Leptos Lucide Icons Example"</h1>

            <div class="icon-grid">
                <div class="icon-item">
                    <Home/>
                    <span>"Home"</span>
                </div>

                <div class="icon-item">
                    <User/>
                    <span>"User"</span>
                </div>

                <div class="icon-item">
                    <Heart/>
                    <span>"Heart"</span>
                </div>

                <div class="icon-item">
                    <Search/>
                    <span>"Search"</span>
                </div>

                <div class="icon-item">
                    <Star/>
                    <span>"Star"</span>
                </div>
            </div>

            <div class="usage-examples">
                <h2>"Usage Examples"</h2>

                <div class="example">
                    <h3>"Basic Usage"</h3>
                    <Home/>
                </div>

                <div class="example">
                    <h3>"With Custom CSS Class"</h3>
                    <div class="custom-icon">
                        <User/>
                    </div>
                </div>

                <div class="example">
                    <h3>"Using the Macro"</h3>
                    {icon!(Heart)}
                </div>
            </div>

            <div class="stats">
                <p>"Total available icons: " {ICON_COUNT}</p>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
