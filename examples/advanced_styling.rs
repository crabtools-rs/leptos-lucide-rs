use leptos::prelude::*;
use leptos_lucide_rs::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <h1>"Leptos Lucide Icons - Advanced Styling"</h1>

            <div class="section">
                <h2>"Size Variations"</h2>
                <div class="size-demo">
                    <div class="size-item icon-small">
                        <Home />
                        <span>"16px"</span>
                    </div>
                    <div class="size-item icon-medium">
                        <Home />
                        <span>"24px"</span>
                    </div>
                    <div class="size-item icon-large">
                        <Home />
                        <span>"32px"</span>
                    </div>
                    <div class="size-item icon-xl">
                        <Home />
                        <span>"48px"</span>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>"Color Variations"</h2>
                <div class="color-demo">
                    <div class="color-item icon-red">
                        <Heart />
                        <span>"Red"</span>
                    </div>
                    <div class="color-item icon-blue">
                        <Heart />
                        <span>"Blue"</span>
                    </div>
                    <div class="color-item icon-green">
                        <Heart />
                        <span>"Green"</span>
                    </div>
                    <div class="color-item icon-purple">
                        <Heart />
                        <span>"Purple"</span>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>"Animations"</h2>
                <div class="animation-demo">
                    <div class="animate-item">
                        <div class="icon-spin">
                            <Loader />
                        </div>
                        <span>"Spin"</span>
                    </div>
                    <div class="animate-item">
                        <div class="icon-bounce">
                            <ArrowUp />
                        </div>
                        <span>"Bounce"</span>
                    </div>
                    <div class="animate-item">
                        <div class="icon-pulse">
                            <Heart />
                        </div>
                        <span>"Pulse"</span>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>"Stroke Width"</h2>
                <div class="stroke-demo">
                    <div class="stroke-item icon-thin">
                        <Circle />
                        <span>"1px"</span>
                    </div>
                    <div class="stroke-item icon-normal">
                        <Circle />
                        <span>"2px"</span>
                    </div>
                    <div class="stroke-item icon-thick">
                        <Circle />
                        <span>"3px"</span>
                    </div>
                </div>
            </div>

            <div class="section">
                <h2>"Icon Combinations"</h2>
                <div class="combo-demo">
                    <div class="combo-item">
                        <div class="icon-stack">
                            <div class="icon-bg">
                                <Square />
                            </div>
                            <div class="icon-fg">
                                <Heart />
                            </div>
                        </div>
                        <span>"Stacked"</span>
                    </div>

                    <div class="combo-item">
                        <div class="icon-row">
                            <Home />
                            <ChevronRight />
                            <User />
                        </div>
                        <span>"Breadcrumb"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|| view! { <App /> })
}
