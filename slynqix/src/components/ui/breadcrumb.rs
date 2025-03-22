use leptos::*;
use leptos_router::A;

#[component]
pub fn Breadcrumb(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <nav
            aria-label="breadcrumb"
            class=move || class_name.clone().unwrap_or_default()
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn BreadcrumbList(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <ol
            class=move || format!(
                "flex flex-wrap items-center gap-1.5 break-words text-sm text-gray-500 dark:text-gray-400 sm:gap-2.5 {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </ol>
    }
}

#[component]
pub fn BreadcrumbItem(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <li
            class=move || format!("inline-flex items-center gap-1.5 {}", class_name.unwrap_or_default())
        >
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbLink(children: Children, href: String, class_name: Option<String>) -> impl IntoView {
    view! {
        <A
            href=href
            class=move || format!("transition-colors hover:text-gray-900 dark:hover:text-white {}", class_name.unwrap_or_default())
        >
            {children()}
        </A>
    }
}

#[component]
pub fn BreadcrumbPage(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <span
            role="link"
            aria-disabled="true"
            aria-current="page"
            class=move || format!("font-normal text-gray-900 dark:text-white {}", class_name.unwrap_or_default())
        >
            {children()}
        </span>
    }
}

#[component]
pub fn BreadcrumbSeparator(children: Option<Children>, class_name: Option<String>) -> impl IntoView {
    view! {
        <li
            role="presentation"
            aria-hidden="true"
            class=move || format!("[&>svg]:size-3.5 {}", class_name.unwrap_or_default())
        >
            {match children {
                Some(children) => children(),
                None => view! {
                    <svg class="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M9 5l7 7-7 7" />
                    </svg>
                }.into(),
            }}
        </li>
    }
}

#[component]
pub fn BreadcrumbEllipsis(class_name: Option<String>) -> impl IntoView {
    view! {
        <span
            role="presentation"
            aria-hidden="true"
            class=move || format!("flex h-9 w-9 items-center justify-center {}", class_name.unwrap_or_default())
        >
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M5 12h.01M12 12h.01M19 12h.01" />
            </svg>
            <span class="sr-only">"More"</span>
        </span>
    }
}
