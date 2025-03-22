use leptos::*;

#[component]
pub fn RadioGroup(
    value: RwSignal<String>,
    options: Vec<(String, String)>, // (value, label)
    class_name: Option<String>,
) -> impl IntoView {
    view! {
        <div class=move || format!("grid gap-2 {}", class_name.unwrap_or_default())>
            {options.into_iter().map(|(val, label)| {
                view! {
                    <label class="flex items-center">
                        <input
                            type="radio"
                            name="radio-group"
                            value=val.clone()
                            checked=move || value.get() == val
                            on:change=move |ev| if event_target_checked(&ev) { value.set(val.clone()) }
                            class="aspect-square h-4 w-4 rounded-full border border-blue-500 text-blue-500 ring-offset-gray-950 focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                        />
                        <Show when=move || value.get() == val>
                            <svg class="h-2.5 w-2.5 fill-current text-current absolute left-1" viewBox="0 0 24 24">
                                <circle cx="12" cy="12" r="6" />
                            </svg>
                        </Show>
                        <span class="ml-2">{label}</span>
                    </label>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
