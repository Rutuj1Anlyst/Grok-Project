use leptos::*;

#[derive(Clone)]
struct NotificationItem {
    id: String,
    title: String,
    description: String,
    time: String,
    is_read: bool,
}

#[component]
pub fn NotificationDropdown() -> impl IntoView {
    let (is_open, set_is_open) = create_signal(false);
    let (notifications, set_notifications) = create_signal(vec![
        NotificationItem {
            id: "1".to_string(),
            title: "New Market Update".to_string(),
            description: "Nifty 50 has crossed 22500 points today.".to_string(),
            time: "5 minutes ago".to_string(),
            is_read: false,
        },
        NotificationItem {
            id: "2".to_string(),
            title: "Alert Triggered".to_string(),
            description: "Your price alert for Reliance has been triggered.".to_string(),
            time: "1 hour ago".to_string(),
            is_read: false,
        },
        NotificationItem {
            id: "3".to_string(),
            title: "New Feature Available".to_string(),
            description: "Check out the new Algo Trading features!".to_string(),
            time: "2 days ago".to_string(),
            is_read: true,
        },
    ]);

    let unread_count = move || notifications.get().iter().filter(|n| !n.is_read).count();

    let mark_as_read = move |id: String| {
        set_notifications.update(|nots| {
            for not in nots.iter_mut() {
                if not.id == id {
                    not.is_read = true;
                }
            }
        });
    };

    let remove_notification = move |id: String| {
        set_notifications.update(|nots| {
            nots.retain(|not| not.id != id);
        });
    };

    view! {
        <div class="relative">
            <button
                class="flex items-center justify-center h-9 w-9 rounded-full hover:bg-gray-800 transition-colors relative"
                on:click=move |_| set_is_open.update(|val| *val = !*val)
                aria-label="Notifications"
            >
                <svg class="h-5 w-5 text-gray-300" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M12 22c1.1 0 2-.9 2-2h-4c0 1.1.9 2 2 2zm6-6v-5c0-3.1-1.6-5.6-4.5-6.3V4c0-.8-.7-1.5-1.5-1.5s-1.5.7-1.5 1.5v.7C7.6 5.4 6 7.9 6 11v5l-2 2v1h16v-1l-2-2z" />
                </svg>
                {move || if unread_count() > 0 {
                    view! {
                        <span class="absolute top-0 right-0 h-4 w-4 rounded-full bg-violet-600 flex items-center justify-center text-[10px] font-medium">
                            {unread_count()}
                        </span>
                    }
                } else {
                    None
                }}
            </button>

            {move || if is_open.get() {
                view! {
                    <div class="fixed inset-0 z-40" on:click=move |_| set_is_open(false) />
                    <div class=move || format!(
                        "absolute right-0 mt-2 w-80 bg-gray-900 border border-gray-800 rounded-lg shadow-lg z-50 transition-all transform origin-top-right {}",
                        if is_open.get() { "scale-100 opacity-100" } else { "scale-95 opacity-0 pointer-events-none" }
                    )>
                        <div class="p-4 border-b border-gray-800">
                            <div class="flex items-center justify-between">
                                <h3 class="text-sm font-medium text-white">"Notifications"</h3>
                                <span class="text-xs text-gray-400">{unread_count} " unread"</span>
                            </div>
                        </div>
                        <div class="max-h-96 overflow-y-auto">
                            {move || if notifications.get().is_empty() {
                                view! {
                                    <div class="p-4 text-center text-gray-500 text-sm">"No notifications"</div>
                                }
                            } else {
                                notifications.get().into_iter().map(|notification| {
                                    view! {
                                        <div
                                            class=move || format!(
                                                "p-4 border-b border-gray-800 last:border-0 transition-colors hover:bg-gray-800/50 {}",
                                                if !notification.is_read { "bg-gray-800/20" } else { "" }
                                            )
                                        >
                                            <div class="flex items-start gap-3">
                                                <div class="flex-shrink-0 h-9 w-9 rounded-full bg-violet-900/30 flex items-center justify-center">
                                                    <svg class="h-5 w-5 text-violet-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                        <path d="M12 12c2.2 0 4-1.8 4-4s-1.8-4-4-4-4 1.8-4 4 1.8 4 4 4zm0 2c-2.7 0-8 1.3-8 4v2h16v-2c0-2.7-5.3-4-8-4z" />
                                                    </svg>
                                                </div>
                                                <div class="flex-grow" on:click=move |_| mark_as_read(notification.id.clone())>
                                                    <div class="flex items-start justify-between gap-2">
                                                        <p class="text-sm font-medium text-white">{notification.title}</p>
                                                        <button
                                                            class="text-gray-500 hover:text-gray-300"
                                                            on:click=move |ev| {
                                                                ev.stop_propagation();
                                                                remove_notification(notification.id.clone());
                                                            }
                                                        >
                                                            <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                                <path d="M6 18L18 6M6 6l12 12" />
                                                            </svg>
                                                        </button>
                                                    </div>
                                                    <p class="text-xs text-gray-400 mt-1">{notification.description}</p>
                                                    <p class="text-xs text-gray-500 mt-1">{notification.time}</p>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>
                        <div class="p-2 border-t border-gray-800">
                            <button class="w-full text-violet-400 hover:text-violet-300 hover:bg-gray-800 px-4 py-2 rounded">
                                "View all notifications"
                            </button>
                        </div>
                    </div>
                }
            } else {
                None
            }}
        </div>
    }
}
