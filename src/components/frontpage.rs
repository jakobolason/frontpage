use leptos::prelude::*;

#[component]
pub fn FrontPage() -> impl IntoView {
    view! {
        <div class="poster-layout">

            <h1 class="name-title">"Jakob Olason"</h1>

            <div class="content-block">

                <div style="margin-bottom: var(--size-4)">
                    <h3>"Love building with " <span class="code-tag">"Rust"</span></h3>
                    <p style="margin-top: var(--size-3); font-size: var(--font-size-3); opacity: 1.5;">
                        "Computer Engineering student at Aarhus."
                    </p>
                </div>

                <div class="button-row">
                    <a href="https://github.com/jakobolason" target="_blank" class="btn btn-dark">
                        "My Projects"
                    </a>

                    <a href="mailto:jakobolason@proton.me" class="btn btn-outline-dark">
                        "Contact Me"
                    </a>
                </div>

                <div style="margin-top: var(--size-6); display: flex; gap: var(--size-4);">
                     <a href="https://linkedin.com" class="btn-text" style="color: white; text-decoration: underline;">"LinkedIn"</a>
                     <a href="https://github.com" class="btn-text" style="color: white; text-decoration: underline;">"GitHub"</a>
                </div>
            </div>

        </div>
    }
}
