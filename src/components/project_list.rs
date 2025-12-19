use leptos::prelude::*;
use lucide_leptos::{ArrowRightFromLine, MoveLeft};

#[derive(Clone)]
struct ProjectData {
    name: &'static str,
    desc: &'static str,
    link: &'static str,
}

#[component]
pub fn ProjectList() -> impl IntoView {
    let projects = vec![
        ProjectData {
            name: "Family Graph",
            desc: "A small program that takes an .xls file with a specific formatting, to create a family tree in SVG format.",
            link: "https://github.com/jakobolason/family-graph",
        },
        ProjectData {
            name: "Website for family graph",
            desc: "Using the previously mentioned graph generator, Loco and Nuxt was used to create a password protected website to display the family graph.",
            link: "https://tree.jakobolason.dk",
        },
        ProjectData {
            name: "KitchenGuard",
            desc: "A group project to watch over a kitchen using sensors and actuators, to keep the resident safe.",
            link: "https://github.com/jakobolason/KitchenGuard",
        },
        ProjectData {
            name: "Samsø Ferry countdown",
            desc: "To my brother, a countdown timer for an Arduino board that uses 'Rejseplanen' to get the soonest departure time for his Ferry from Samsø.",
            link: "https://github.com/jakobolason/sams_ferry_countdown",
        },
    ];

    view! {
        <div class="poster-layout">

            <div style="width: 100%; margin-bottom: var(--size-6)">
                <a href="/" style="color: var(--gray-9); text-decoration: none; font-weight: bold;">
                    <div class="btn-arrow" style="display: flex; align-items: center; gap: var(--size-2); width: 120px;">
                        <MoveLeft />
                        " Back"
                    </div>
                </a>
                <h1 class="name-title" style="font-size: var(--font-size-7); margin-top: var(--size-3)">
                    "Selected Work"
                </h1>
            </div>

            // Project List
            <div class="project-list">
                {projects.into_iter().map(|p| view! {
                    <div class="project-row">
                        <div class="project-info">
                            <span class="project-title">{p.name}</span>
                            <span class="project-desc">{p.desc}</span>
                        </div>

                            <arrow-right-from-line acolor="red" size=48 />
                        <a href={p.link} target="_blank" class="btn-arrow" title="View Repo">
                            <div style="display: flex; align-items: center; gap: var(--size-2);">
                                "View "
                                <ArrowRightFromLine />
                            </div>
                        </a>
                    </div>
                }).collect_view()}
            </div>
        </div>
    }
}
