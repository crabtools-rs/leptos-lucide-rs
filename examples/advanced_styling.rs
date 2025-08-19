use leptos::prelude::*;
use leptos_lucide_tree::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"Advanced Styling Examples"</h1>

            <div class="section">
                <h2>"Size Variations"</h2>
                <div class="size-demo">
                    <div class="size-item">
                        <div class="icon-small">
                            <Home/>
                        </div>
                        <span>"Small (16px)"</span>
                    </div>
                    <div class="size-item">
                        <div class="icon-medium">
                            <Home/>
                        </div>
                        <span>"Medium (24px)"</span>
                    </div>
                    <div class="size-item">
                        <div class="icon-large">
                            <Home/>
                        </div>
                        <span>"Large (32px)"</span>
                    </div>
                    <div class="size-item">
                        <div class="icon-xl">
                            <Home/>
                        </div>
                        <span>"Extra Large (48px)"</span>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>"Color Variations"</h2>
                <div class="color-demo">
                    <div class="color-item">
                        <div class="icon-red">
                            <Heart/>
                        </div>
                        <span>"Red Heart"</span>
                    </div>
                    <div class="color-item">
                        <div class="icon-blue">
                            <User/>
                        </div>
                        <span>"Blue User"</span>
                    </div>
                    <div class="color-item">
                        <div class="icon-green">
                            <Search/>
                        </div>
                        <span>"Green Search"</span>
                    </div>
                    <div class="color-item">
                        <div class="icon-purple">
                            <Star/>
                        </div>
                        <span>"Purple Star"</span>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>"Interactive Icons"</h2>
                <div class="interactive-demo">
                    <button class="icon-button">
                        <Home/>
                        <span>"Home Button"</span>
                    </button>
                    <button class="icon-button">
                        <User/>
                        <span>"Profile"</span>
                    </button>
                    <button class="icon-button">
                        <Search/>
                        <span>"Search"</span>
                    </button>
                </div>
            </div>

            <div class="section">
                <h2>"Animated Icons"</h2>
                <div class="animation-demo">
                    <div class="animate-item">
                        <div class="icon-spin">
                            <Star/>
                        </div>
                        <span>"Spinning Star"</span>
                    </div>
                    <div class="animate-item">
                        <div class="icon-bounce">
                            <Heart/>
                        </div>
                        <span>"Bouncing Heart"</span>
                    </div>
                    <div class="animate-item">
                        <div class="icon-pulse">
                            <Search/>
                        </div>
                        <span>"Pulsing Search"</span>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>"Custom Stroke Styles"</h2>
                <div class="stroke-demo">
                    <div class="stroke-item">
                        <div class="icon-thin">
                            <Home/>
                        </div>
                        <span>"Thin Stroke"</span>
                    </div>
                    <div class="stroke-item">
                        <div class="icon-normal">
                            <Home/>
                        </div>
                        <span>"Normal Stroke"</span>
                    </div>
                    <div class="stroke-item">
                        <div class="icon-thick">
                            <Home/>
                        </div>
                        <span>"Thick Stroke"</span>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>"Icon Combinations"</h2>
                <div class="combo-demo">
                    <div class="combo-item">
                        <div class="icon-stack">
                            <div class="icon-bg">
                                <Star/>
                            </div>
                            <div class="icon-fg">
                                <Heart/>
                            </div>
                        </div>
                        <span>"Stacked Icons"</span>
                    </div>
                    <div class="combo-item">
                        <div class="icon-row">
                            <User/>
                            <Heart/>
                            <Star/>
                        </div>
                        <span>"Icon Row"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
