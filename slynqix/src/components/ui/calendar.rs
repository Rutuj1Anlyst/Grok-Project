use leptos::*;
use chrono::{Datelike, NaiveDate, Duration};

#[component]
pub fn Calendar(
    selected: Option<NaiveDate>,
    on_select: Callback<NaiveDate>,
    class_name: Option<String>,
) -> impl IntoView {
    let (current_month, set_current_month) = create_signal(NaiveDate::from_ymd_opt(2025, 3, 1).unwrap());
    let days_in_month = move || {
        let next_month = current_month.get() + Duration::days(32);
        (next_month - Duration::days(next_month.day() as i64)).day()
    };
    let first_day = move || current_month.get().weekday().num_days_from_sunday();

    view! {
        <div class=move || format!("p-3 {}", class_name.unwrap_or_default())>
            <div class="flex justify-center pt-1 relative items-center">
                <button
                    class="absolute left-1 h-7 w-7 bg-transparent p-0 opacity-50 hover:opacity-100 border border-gray-300 dark:border-gray-700 rounded"
                    on:click=move |_| set_current_month.update(|d| *d = *d - Duration::days(d.day() as i64 + 1))
                >
                    <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M15 18l-6-6 6-6" />
                    </svg>
                </button>
                <span class="text-sm font-medium">
                    {move || current_month.get().format("%B %Y").to_string()}
                </span>
                <button
                    class="absolute right-1 h-7 w-7 bg-transparent p-0 opacity-50 hover:opacity-100 border border-gray-300 dark:border-gray-700 rounded"
                    on:click=move |_| set_current_month.update(|d| *d = *d + Duration::days(days_in_month() as i64))
                >
                    <svg class="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M9 18l6-6-6-6" />
                    </svg>
                </button>
            </div>
            <table class="w-full border-collapse space-y-1">
                <thead>
                    <tr class="flex">
                        {["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].iter().map(|day| {
                            view! {
                                <th class="text-gray-500 dark:text-gray-400 rounded-md w-9 font-normal text-[0.8rem]">
                                    {day}
                                </th>
                            }
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody>
                    {(0..6).map(|week| {
                        view! {
                            <tr class="flex w-full mt-2">
                                {(0..7).map(|day| {
                                    let day_num = week * 7 + day - first_day() + 1;
                                    let date = current_month.get() + Duration::days((day_num - 1) as i64);
                                    let is_selected = selected.map_or(false, |s| s == date);
                                    let is_today = date == chrono::Local::now().date_naive();
                                    view! {
                                        <td class=move || format!(
                                            "h-9 w-9 text-center text-sm p-0 relative {} {}",
                                            if is_selected { "bg-blue-500 text-white hover:bg-blue-500 focus:bg-blue-500" } else if is_today { "bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-white" } else { "hover:bg-gray-100 dark:hover:bg-gray-800" },
                                            if day_num <= 0 || day_num > days_in_month() as i32 { "text-gray-500 dark:text-gray-400 opacity-50" } else { "" }
                                        )>
                                            <button
                                                class="h-9 w-9 p-0 font-normal"
                                                on:click=move |_| if day_num > 0 && day_num <= days_in_month() as i32 { on_select(date) }
                                            >
                                                {if day_num > 0 && day_num <= days_in_month() as i32 { day_num.to_string() } else { "".to_string() }}
                                            </button>
                                        </td>
                                    }
                                }).collect::<Vec<_>>()}
                            </tr>
                        }
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}
