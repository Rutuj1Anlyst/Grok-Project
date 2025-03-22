use leptos::*;

#[component]
pub fn AspectRatio(children: Children, ratio: Option<f64>) -> impl IntoView {
    let ratio = ratio.unwrap_or(1.0);

    view! {
        <div style=move || format!("position: relative; width: 100%; padding-bottom: {}%;", 100.0 / ratio)>
            <div style="position: absolute; top: 0; left: 0; width: 100%; height: 100%;">
                {children()}
            </div>
        </div>
    }
}
