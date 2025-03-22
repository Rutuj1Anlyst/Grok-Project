use leptos::*;
use crate::components::ui::label::Label;

#[component]
pub fn Form(children: Children, on_submit: Callback<HashMap<String, String>>) -> impl IntoView {
    let (form_data, set_form_data) = create_signal(HashMap::<String, String>::new());

    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();
                on_submit(form_data.get());
            }
        >
            <div class="space-y-2">
                {children()}
            </div>
        </form>
    }
}

#[component]
pub fn FormField(
    name: String,
    children: ChildrenFn,
    form_data: RwSignal<HashMap<String, String>>,
) -> impl IntoView {
    let id = format!("form-item-{}", name);

    view! {
        <div>
            {children((
                id.clone(),
                name.clone(),
                format!("{}-description", id),
                format!("{}-message", id),
                form_data,
            ))}
        </div>
    }
}

#[component]
pub fn FormItem(children: Children, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!("space-y-2 {}", class_name.unwrap_or_default())>
            {children()}
        </div>
    }
}

#[component]
pub fn FormLabel(children: Children, html_for: String, error: Option<String>, class_name: Option<String>) -> impl IntoView {
    view! {
        <Label
            html_for=html_for
            class_name=move || format!(
                "{} {}",
                if error.is_some() { "text-red-600" } else { "" },
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </Label>
    }
}

#[component]
pub fn FormControl(
    id: String,
    name: String,
    form_data: RwSignal<HashMap<String, String>>,
    children: Children,
    error: Option<String>,
    description_id: String,
    message_id: String,
) -> impl IntoView {
    view! {
        <div
            id=id
            aria-describedby=move || if error.is_none() { description_id.clone() } else { format!("{} {}", description_id, message_id) }
            aria-invalid=move || error.is_some()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn FormDescription(children: Children, id: String, class_name: Option<String>) -> impl IntoView {
    view! {
        <p
            id=id
            class=move || format!("text-sm text-gray-500 dark:text-gray-400 {}", class_name.unwrap_or_default())
        >
            {children()}
        </p>
    }
}

#[component]
pub fn FormMessage(children: Children, id: String, error: Option<String>, class_name: Option<String>) -> impl IntoView {
    let body = error.clone().unwrap_or_default();

    view! {
        <Show when=move || !body.is_empty() || !children().nodes.is_empty()>
            <p
                id=id
                class=move || format!("text-sm font-medium text-red-600 {}", class_name.unwrap_or_default())
            >
                {if !body.is_empty() { body.clone() } else { children().nodes.into_iter().collect::<Vec<_>>()[0].to_string() }}
            </p>
        </Show>
    }
}
