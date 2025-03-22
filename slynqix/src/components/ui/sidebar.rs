use leptos::*;
use leptos::ev::KeyboardEvent;
use std::rc::Rc;

// Constants
const SIDEBAR_WIDTH: &str = "16rem";
const SIDEBAR_WIDTH_MOBILE: &str = "18rem";
const SIDEBAR_WIDTH_ICON: &str = "3rem";
const SIDEBAR_KEYBOARD_SHORTCUT: &str = "b";

// Sidebar Context
#[derive(Clone)]
struct SidebarContext {
    state: ReadSignal<&'static str>,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    open_mobile: ReadSignal<bool>,
    set_open_mobile: WriteSignal<bool>,
    is_mobile: ReadSignal<bool>,
    toggle_sidebar: Callback<()>,
}

#[component]
pub fn SidebarProvider(
    children: Children,
    #[prop(default = true)] default_open: bool,
    #[prop(optional)] open: Option<Rc<ReadSignal<bool>>>,
    #[prop(optional)] set_open: Option<Rc<WriteSignal<bool>>>,
    class_name: Option<String>,
) -> impl IntoView {
    let is_mobile = create_signal(false); // Simplified; replace with actual mobile detection logic
    let (open_mobile, set_open_mobile) = create_signal(false);
    let (internal_open, internal_set_open) = create_signal(default_open);
    let open_signal = open.map_or(internal_open, |o| *o);
    let set_open_signal = set_open.map_or(internal_set_open, |s| *s);

    let toggle_sidebar = Callback::new(move |_| {
        if is_mobile.0.get() {
            set_open_mobile.update(|val| *val = !*val);
        } else {
            set_open_signal.update(|val| *val = !*val);
        }
    });

    // Keyboard shortcut effect
    create_effect(move |_| {
        let handle_keydown = move |ev: KeyboardEvent| {
            if (ev.meta_key() || ev.ctrl_key()) && ev.key() == SIDEBAR_KEYBOARD_SHORTCUT {
                ev.prevent_default();
                toggle_sidebar.call(());
            }
        };
        leptos_dom::on(document(), "keydown", handle_keydown);
        on_cleanup(|| leptos_dom::remove_event_listener(document(), "keydown", handle_keydown));
    });

    let state = create_memo(move |_| if open_signal.get() { "expanded" } else { "collapsed" });
    let context = SidebarContext {
        state,
        open: open_signal,
        set_open: set_open_signal,
        open_mobile,
        set_open_mobile,
        is_mobile: is_mobile.0,
        toggle_sidebar,
    };
    provide_context(context);

    view! {
        <div
            class=move || format!(
                "group/sidebar-wrapper flex min-h-screen w-full has-[[data-variant=inset]]:bg-gray-100 dark:bg-gray-800 {}",
                class_name.unwrap_or_default()
            )
            style=format!("--sidebar-width: {}; --sidebar-width-icon: {};", SIDEBAR_WIDTH, SIDEBAR_WIDTH_ICON)
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Sidebar(
    children: Children,
    #[prop(default = "left")] side: &'static str,
    #[prop(default = "sidebar")] variant: &'static str,
    #[prop(default = "offcanvas")] collapsible: &'static str,
    class_name: Option<String>,
) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("Sidebar must be used within SidebarProvider");

    if collapsible == "none" {
        return view! {
            <div
                class=move || format!(
                    "flex h-full w-[--sidebar-width] flex-col bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-white {}",
                    class_name.unwrap_or_default()
                )
            >
                {children()}
            </div>
        };
    }

    if context.is_mobile.get() {
        view! {
            <Sheet open=context.open_mobile set_open=context.set_open_mobile side=Some(side)>
                <div
                    class="w-[--sidebar-width] bg-gray-100 dark:bg-gray-800 p-0 text-gray-900 dark:text-white flex h-full flex-col"
                    style=format!("--sidebar-width: {};", SIDEBAR_WIDTH_MOBILE)
                >
                    {children()}
                </div>
            </Sheet>
        }
    } else {
        view! {
            <div
                class="group peer hidden md:block text-gray-900 dark:text-white"
                data-state=move || context.state.get()
                data-collapsible=move || if context.state.get() == "collapsed" { collapsible } else { "" }
                data-variant=variant
                data-side=side
            >
                <div
                    class=move || format!(
                        "duration-200 relative h-screen w-[--sidebar-width] bg-transparent transition-[width] ease-linear {} {}",
                        if collapsible == "offcanvas" && context.state.get() == "collapsed" { "w-0" } else { "" },
                        if side == "right" { "rotate-180" } else { "" }
                    )
                />
                <div
                    class=move || format!(
                        "duration-200 fixed inset-y-0 z-10 hidden h-screen w-[--sidebar-width] transition-[left,right,width] ease-linear md:flex {} {} {}",
                        if side == "left" {
                            if collapsible == "offcanvas" && context.state.get() == "collapsed" { "left-[calc(var(--sidebar-width)*-1)]" } else { "left-0" }
                        } else {
                            if collapsible == "offcanvas" && context.state.get() == "collapsed" { "right-[calc(var(--sidebar-width)*-1)]" } else { "right-0" }
                        },
                        if variant == "floating" || variant == "inset" {
                            format!("p-2 {}", if collapsible == "icon" && context.state.get() == "collapsed" { "w-[calc(var(--sidebar-width-icon)+theme(spacing.4))]" } else { "" })
                        } else {
                            format!("{}", if collapsible == "icon" && context.state.get() == "collapsed" { "w-[--sidebar-width-icon]" } else { "" })
                        },
                        class_name.unwrap_or_default()
                    )
                >
                    <div
                        class=move || format!(
                            "flex h-full w-full flex-col bg-gray-100 dark:bg-gray-800 {}",
                            if variant == "floating" { "rounded-lg border border-gray-300 dark:border-gray-700 shadow" } else { "" }
                        )
                    >
                        {children()}
                    </div>
                </div>
            </div>
        }
    }
}

#[component]
pub fn SidebarTrigger(class_name: Option<String>) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarTrigger must be used within SidebarProvider");

    view! {
        <Button
            variant="ghost"
            size="icon"
            class=move || format!("h-7 w-7 {}", class_name.unwrap_or_default())
            on:click=move |_| context.toggle_sidebar.call(())
        >
            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M3 6h18M3 12h18M3 18h18" />
            </svg>
            <span class="sr-only">"Toggle Sidebar"</span>
        </Button>
    }
}

#[component]
pub fn SidebarHeader(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("flex flex-col gap-2 p-2 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarFooter(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("flex flex-col gap-2 p-2 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarContent(children: Children, class_name: Option<String>) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarContent must be used within SidebarProvider");

    view! {
        <div
            class=move || format!(
                "flex min-h-0 flex-1 flex-col gap-2 overflow-auto {} {}",
                if context.state.get() == "collapsed" && context.collapsible == "icon" { "overflow-hidden" } else { "" },
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroup(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("relative flex w-full min-w-0 flex-col p-2 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarGroupLabel(children: Children, class_name: Option<String>) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarGroupLabel must be used within SidebarProvider");

    view! {
        <div
            class=move || format!(
                "duration-200 flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium text-gray-600 dark:text-gray-400 outline-none transition-[margin,opacity] ease-linear {} {}",
                if context.state.get() == "collapsed" && context.collapsible == "icon" { "-mt-8 opacity-0" } else { "" },
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenu(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <ul class=move || format!("flex w-full min-w-0 flex-col gap-1 {}", class_name.unwrap_or_default())>
            {children()}
        </ul>
    }
}

#[component]
pub fn SidebarMenuItem(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <li class=move || format!("group/menu-item relative {}", class_name.unwrap_or_default())>
            {children()}
        </li>
    }
}

#[component]
pub fn SidebarMenuButton(
    children: Children,
    #[prop(default = false)] is_active: bool,
    class_name: Option<String>,
) -> impl IntoView {
    let context = use_context::<SidebarContext>().expect("SidebarMenuButton must be used within SidebarProvider");

    view! {
        <button
            class=move || format!(
                "flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left text-sm outline-none transition-[width,height,padding] hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-white focus-visible:ring-2 focus-visible:ring-blue-500 active:bg-gray-200 dark:active:bg-gray-700 active:text-gray-900 dark:active:text-white disabled:pointer-events-none disabled:opacity-50 {} {} {}",
                if is_active { "bg-gray-200 dark:bg-gray-700 font-medium text-gray-900 dark:text-white" } else { "" },
                if context.state.get() == "collapsed" && context.collapsible == "icon" { "size-8 p-2" } else { "h-8" },
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </button>
    }
}
