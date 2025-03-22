use leptos::*;
use crate::components::ui::button::{Button, button_variants};

#[component]
pub fn Pagination(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <nav
            role="navigation"
            aria-label="pagination"
            class=move || format!("mx-auto flex w-full justify-center {}", class_name.unwrap_or_default())
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn PaginationContent(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <ul
            class=move || format!("flex flex-row items-center gap-1 {}", class_name.unwrap_or_default())
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn PaginationItem(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <li class=class_name.unwrap_or_default()>
            {children()}
        </li>
    }
}

#[component]
pub fn PaginationLink(
    href: String,
    is_active: Option<bool>,
    size: Option<&'static str>,
    class_name: Option<String>,
) -> impl IntoView {
    let is_active = is_active.unwrap_or(false);
    let size = size.unwrap_or("icon");

    view! {
        <a
            href=href
            aria-current=move || if is_active { Some("page") } else { None }
            class=move || format!(
                "{} {}",
                button_variants(if is_active { "outline" } else { "ghost" }, size),
                class_name.unwrap_or_default()
            )
        >
            {if is_active { "Current" } else { "Link" }}
        </a>
    }
}

#[component]
pub fn PaginationPrevious(class_name: Option<String>) -> impl IntoView {
    view! {
        <PaginationLink
            href="#"
            size="default"
            class_name=move || format!("gap-1 pl-2.5 {}", class_name.unwrap_or_default())
        >
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M15 18l-6-6 6-6" />
            </svg>
            <span>"Previous"</span>
        </PaginationLink>
    }
}

#[component]
pub fn PaginationNext(class_name: Option<String>) -> impl IntoView {
    view! {
        <PaginationLink
            href="#"
            size="default"
            class_name=move || format!("gap-1 pr-2.5 {}", class_name.unwrap_or_default())
        >
            <span>"Next"</span>
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M9 18l6-6-6-6" />
            </svg>
        </PaginationLink>
    }
}

#[component]
pub fn PaginationEllipsis(class_name: Option<String>) -> impl IntoView {
    view! {
        <span
            aria-hidden="true"
            class=move || format!("flex h-9 w-9 items-center justify-center {}", class_name.unwrap_or_default())
        >
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="1" />
                <circle cx="12" cy="5" r="1" />
                <circle cx="12" cy="19" r="1" />
            </svg>
            <span class="sr-only">"More pages"</span>
        </span>
    }
}
