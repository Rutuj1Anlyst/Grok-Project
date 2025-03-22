use leptos::*;

#[derive(Clone)]
pub struct TableData {
    label: String,
    value: String,
    status: Option<&'static str>, // "positive", "negative", "neutral", or None
}

#[component]
pub fn AnalysisTable(title: String, data: Vec<TableData>, class_name: Option<String>) -> impl IntoView {
    view! {
        <div class=move || format!(
            "glass-card rounded-xl p-4 {}",
            class_name.unwrap_or_default()
        )>
            <h3 class="text-lg font-medium mb-4">{title}</h3>
            <div class="overflow-hidden">
                <table class="min-w-full">
                    <tbody class="divide-y divide-gray-200 dark:divide-gray-800">
                        {data.into_iter().enumerate().map(|(index, row)| {
                            view! {
                                <tr key=index class="transition-colors hover:bg-gray-50 dark:hover:bg-gray-900/50">
                                    <td class="py-3 text-sm font-medium text-gray-700 dark:text-gray-300">
                                        {row.label}
                                    </td>
                                    <td class=move || format!(
                                        "py-3 text-sm text-right {}",
                                        match row.status {
                                            Some("positive") => "text-green-600 dark:text-green-400",
                                            Some("negative") => "text-red-600 dark:text-red-400",
                                            Some("neutral") => "text-gray-600 dark:text-gray-400",
                                            _ => "text-gray-600 dark:text-gray-400",
                                        }
                                    )>
                                        {row.value}
                                    </td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}
