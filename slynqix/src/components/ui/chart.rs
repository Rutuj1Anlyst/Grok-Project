use leptos::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ChartConfig {
    label: Option<String>,
    color: Option<String>,
}

#[component]
pub fn ChartContainer(
    children: Children,
    config: HashMap<String, ChartConfig>,
    class_name: Option<String>,
) -> impl IntoView {
    provide_context(config.clone());

    view! {
        <div
            class=move || format!(
                "flex aspect-video justify-center text-xs [&_.recharts-cartesian-axis-tick_text]:fill-gray-500 dark:fill-gray-400 [&_.recharts-cartesian-grid_line[stroke='#ccc']]:stroke-gray-500/50 [&_.recharts-curve.recharts-tooltip-cursor]:stroke-gray-500 [&_.recharts-dot[stroke='#fff']]:stroke-transparent [&_.recharts-layer]:outline-none [&_.recharts-polar-grid_[stroke='#ccc']]:stroke-gray-500 [&_.recharts-radial-bar-background-sector]:fill-gray-200 dark:fill-gray-700 [&_.recharts-rectangle.recharts-tooltip-cursor]:fill-gray-200 dark:fill-gray-700 [&_.recharts-reference-line_[stroke='#ccc']]:stroke-gray-500 [&_.recharts-sector[stroke='#fff']]:stroke-transparent [&_.recharts-sector]:outline-none [&_.recharts-surface]:outline-none {}",
                class_name.unwrap_or_default()
            )
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ChartTooltipContent(
    active: ReadSignal<bool>,
    payload: Vec<(String, f64, Option<String>)>, // (name, value, color)
    label: Option<String>,
    hide_label: Option<bool>,
    hide_indicator: Option<bool>,
    indicator: Option<&'static str>, // "line", "dot", "dashed"
    class_name: Option<String>,
) -> impl IntoView {
    let hide_label = hide_label.unwrap_or(false);
    let hide_indicator = hide_indicator.unwrap_or(false);
    let indicator = indicator.unwrap_or("dot");

    view! {
        <Show when=move || active.get() && !payload.is_empty()>
            <div
                class=move || format!(
                    "grid min-w-[8rem] items-start gap-1.5 rounded-lg border border-gray-500/50 bg-white dark:bg-gray-900 px-2.5 py-1.5 text-xs shadow-xl {}",
                    class_name.unwrap_or_default()
                )
            >
                {move || if !hide_label && label.is_some() {
                    view! {
                        <div class="font-medium">{label.clone().unwrap()}</div>
                    }
                } else {
                    None
                }}
                <div class="grid gap-1.5">
                    {payload.into_iter().map(|(name, value, color)| {
                        let indicator_color = color.unwrap_or("#000".to_string());
                        view! {
                            <div class="flex w-full flex-wrap items-stretch gap-2 [&>svg]:h-2.5 [&>svg]:w-2.5 [&>svg]:text-gray-500 dark:[&>svg]:text-gray-400">
                                {move || if !hide_indicator {
                                    view! {
                                        <div
                                            class=move || format!(
                                                "shrink-0 rounded-[2px] border-[{}] bg-[{}] {}",
                                                indicator_color,
                                                indicator_color,
                                                match indicator {
                                                    "dot" => "h-2.5 w-2.5",
                                                    "line" => "w-1 h-full",
                                                    "dashed" => "w-0 border-[1.5px] border-dashed bg-transparent",
                                                    _ => "h-2.5 w-2.5",
                                                }
                                            )
                                        />
                                    }
                                } else {
                                    None
                                }}
                                <div class="flex flex-1 justify-between leading-none items-center">
                                    <span class="text-gray-500 dark:text-gray-400">{name}</span>
                                    <span class="font-mono font-medium tabular-nums text-gray-900 dark:text-white">
                                        {value.to_string()}
                                    </span>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </Show>
    }
}
