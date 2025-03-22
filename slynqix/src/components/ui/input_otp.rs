use leptos::*;

#[component]
pub fn InputOTP(
    value: RwSignal<String>,
    max_length: usize,
    class_name: Option<String>,
) -> impl IntoView {
    let (active_index, set_active_index) = create_signal(0);

    view! {
        <div class=move || format!("flex items-center gap-2 {}", class_name.unwrap_or_default())>
            {(0..max_length).map(|i| {
                let is_active = move || active_index.get() == i;
                view! {
                    <input
                        type="text"
                        maxlength="1"
                        value=move || value.get().chars().nth(i).unwrap_or(' ').to_string()
                        on:input=move |ev| {
                            let input = event_target_value(&ev);
                            if !input.is_empty() {
                                let mut current = value.get();
                                if current.len() < i { current.push_str(&" ".repeat(i - current.len())); }
                                if current.len() == i { current.push_str(&input); } else { current.replace_range(i..i+1, &input); }
                                value.set(current);
                                if i < max_length - 1 { set_active_index(i + 1); }
                            }
                        }
                        on:focus=move |_| set_active_index(i)
                        class=move || format!(
                            "relative flex h-10 w-10 items-center justify-center border-y border-r border-gray-300 dark:border-gray-700 text-sm transition-all first:rounded-l-md first:border-l last:rounded-r-md {}",
                            if is_active() { "z-10 ring-2 ring-blue-500 ring-offset-gray-950" } else { "" }
                        )
                    />
                    {move || if i < max_length - 1 {
                        view! {
                            <div role="separator">
                                <svg class="h-4 w-4" viewBox="0 0 24 24" fill="currentColor">
                                    <circle cx="12" cy="12" r="2" />
                                </svg>
                            </div>
                        }.into()
                    } else { None }}
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
