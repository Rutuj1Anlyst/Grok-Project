// slynqix/src/components.rs
use leptos::*;

#[component]
pub fn Card(cx: Scope, children: Children) -> impl IntoView {
    view! { cx, <div class="glass-card">{children}</div> }
}

#[component]
pub fn CardHeader(cx: Scope, children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! { cx, <div class=format!("{} p-4", class)>{children}</div> }
}

#[component]
pub fn CardContent(cx: Scope, children: Children) -> impl IntoView {
    view! { cx, <div class="p-4">{children}</div> }
}

#[component]
pub fn CardTitle(cx: Scope, children: Children, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! { cx, <h2 class=format!("text-xl font-medium {}", class)>{children}</h2> }
}

#[component]
pub fn CardDescription(cx: Scope, children: Children) -> impl IntoView {
    view! { cx, <p class="text-gray-600 dark:text-gray-400">{children}</p> }
}

#[component]
pub fn Button(
    cx: Scope,
    children: Children,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] variant: &'static str,
    #[prop(optional)] size: &'static str,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let base_class = "transition-all duration-200 ease-in-out";
    let variant_class = match variant {
        "outline" => "border border-gray-300 hover:bg-gray-100 dark:border-gray-700 dark:hover:bg-gray-800",
        "ghost" => "hover:bg-gray-100 dark:hover:bg-gray-800",
        _ => "bg-accent hover:bg-accent/90 text-white",
    };
    let size_class = match size {
        "sm" => "text-sm px-3 py-1",
        "lg" => "text-lg px-6 py-3",
        _ => "px-4 py-2",
    };
    let disabled_class = if disabled { "opacity-50 cursor-not-allowed" } else { "" };

    view! { cx,
        <button
            class=format!("{} {} {} {} {}", base_class, variant_class, size_class, disabled_class, class)
            on:click=move |ev| if !disabled { if let Some(cb) = on_click { cb.call(ev) } }
            disabled=disabled
        >
            {children}
        </button>
    }
}

#[component]
pub fn Switch(cx: Scope, #[prop(optional)] id: String) -> impl IntoView {
    let (checked, set_checked) = create_signal(cx, false);
    view! { cx,
        <input
            type="checkbox"
            id=id
            class="form-input w-10 h-5 rounded-full"
            checked=checked
            on:change=move |_| set_checked.update(|val| *val = !*val)
        />
    }
}

#[derive(Clone)]
pub struct AnalysisData {
    pub label: String,
    pub value: String,
    pub status: String,
}

#[component]
pub fn AnalysisTable(cx: Scope, title: String, data: Vec<AnalysisData>, #[prop(optional)] class: &'static str) -> impl IntoView {
    view! { cx,
        <div class=format!("glass-card rounded-xl p-6 {}", class)>
            <h2 class="text-xl font-medium mb-4">{title}</h2>
            <table class="w-full">
                <tbody>
                    {data.into_iter().map(|item| {
                        view! { cx,
                            <tr>
                                <td class="py-2 text-sm font-medium">{item.label}</td>
                                <td class="py-2 text-sm text-right">{item.value}</td>
                                <td class="py-2 text-right">
                                    <span class=format!("inline-block w-2 h-2 rounded-full {}",
                                        match item.status.as_str() {
                                            "positive" => "bg-green-500",
                                            "negative" => "bg-red-500",
                                            _ => "bg-gray-500",
                                        }
                                    )></span>
                                </td>
                            </tr>
                        }
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn PictureInPicture(
    cx: Scope,
    title: String,
    is_open: ReadSignal<bool>,
    on_close: impl Fn() + 'static,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <Show when=move || is_open.get()>
            <div class="fixed bottom-4 right-4 w-80 bg-gray-900 rounded-xl p-4 shadow-lg z-50">
                <div class="flex justify-between items-center mb-2">
                    <h3 class="text-sm font-medium">{title}</h3>
                    <button class="text-gray-400 hover:text-gray-200" on:click=move |_| on_close()>"X"</button>
                </div>
                {children}
            </div>
        </Show>
    }
}

#[component]
pub fn MarketCard(cx: Scope, symbol: String, price: f64, change: f64, volume: i32) -> impl IntoView {
    view! { cx,
        <div class="glass-card rounded-xl p-4">
            <h3 class="text-lg font-medium">{symbol}</h3>
            <p class="text-2xl font-bold">{format!("{:.2}", price)}</p>
            <p class=format!("text-sm {}", if change >= 0.0 { "text-green-500" } else { "text-red-500" })>
                {format!("{:+.2}%", change)}
            </p>
            <p class="text-sm text-gray-500">"Volume: " {volume.to_string()}</p>
        </div>
    }
}

#[component]
pub fn StatCard(
    cx: Scope,
    title: String,
    value: String,
    icon: impl IntoView,
    description: String,
    #[prop(optional)] trend: &'static str,
) -> impl IntoView {
    view! { cx,
        <div class="glass-card rounded-xl p-4">
            <div class="flex items-center justify-between">
                <p class="text-sm font-medium">{title}</p>
                {icon}
            </div>
            <p class="text-2xl font-bold mt-2">{value}</p>
            <p class="text-sm text-gray-500">{description}</p>
            {match trend {
                "up" => view! { cx, <span class="text-green-500">↑</span> },
                _ => view! { cx, <span></span> },
            }}
        </div>
    }
}

#[component]
pub fn JournalForm(cx: Scope) -> impl IntoView {
    let (symbol, set_symbol) = create_signal(cx, String::new());
    let (quantity, set_quantity) = create_signal(cx, 0);
    let (buy_price, set_buy_price) = create_signal(cx, 0.0);
    let (sell_price, set_sell_price) = create_signal(cx, 0.0);
    let (action, set_action) = create_signal(cx, "Buy".to_string());
    let (fees, set_fees) = create_signal(cx, 0.0);
    let (notes, set_notes) = create_signal(cx, String::new());

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        log::info!(
            "Submitted: symbol={}, quantity={}, buy_price={}, sell_price={}, action={}, fees={}, notes={}",
            symbol.get(), quantity.get(), buy_price.get(), sell_price.get(), action.get(), fees.get(), notes.get()
        );
    };

    view! { cx,
        <div class="glass-card rounded-xl p-6 animate-fade-in">
            <h2 class="text-xl font-medium mb-4">"Add New Entry"</h2>
            <form on:submit=on_submit>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <input
                        type="text"
                        placeholder="Symbol"
                        class="form-input"
                        on:input=move |ev| set_symbol.set(event_target_value(&ev))
                    />
                    <input
                        type="number"
                        placeholder="Quantity"
                        class="form-input"
                        on:input=move |ev| set_quantity.set(event_target_value(&ev).parse().unwrap_or(0))
                    />
                    <input
                        type="number"
                        placeholder="Buy Price"
                        class="form-input"
                        on:input=move |ev| set_buy_price.set(event_target_value(&ev).parse().unwrap_or(0.0))
                    />
                    <input
                        type="number"
                        placeholder="Sell Price"
                        class="form-input"
                        on:input=move |ev| set_sell_price.set(event_target_value(&ev).parse().unwrap_or(0.0))
                    />
                    <select
                        class="form-input"
                        on:change=move |ev| set_action.set(event_target_value(&ev))
                    >
                        <option value="Buy">"Buy"</option>
                        <option value="Sell">"Sell"</option>
                    </select>
                    <input
                        type="number"
                        placeholder="Fees"
                        class="form-input"
                        on:input=move |ev| set_fees.set(event_target_value(&ev).parse().unwrap_or(0.0))
                    />
                </div>
                <textarea
                    placeholder="Notes"
                    class="form-input mt-4"
                    rows=3
                    on:input=move |ev| set_notes.set(event_target_value(&ev))
                ></textarea>
                <button
                    type="submit"
                    class="mt-4 bg-accent hover:bg-accent/90 rounded-full w-full py-2"
                >
                    "Add Entry"
                </button>
            </form>
        </div>
    }
}

#[component]
pub fn Separator(cx: Scope, class: &'static str) -> impl IntoView {
    view! { cx, <hr class=class/> }
}