use leptos::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
struct JournalEntry {
    symbol: String,
    quantity: f64,
    buy_price: f64,
    sell_price: f64,
    action: String, // "Buy" or "Sell"
    fees: f64,
    notes: String,
}

#[component]
pub fn JournalForm() -> impl IntoView {
    let (entry, set_entry) = create_signal(JournalEntry {
        symbol: "".to_string(),
        quantity: 0.0,
        buy_price: 0.0,
        sell_price: 0.0,
        action: "Buy".to_string(),
        fees: 0.0,
        notes: "".to_string(),
    });
    let (pnl, set_pnl) = create_signal(0.0);

    let calculate_pnl = move || {
        let e = entry.get();
        let calculated_pnl = (e.sell_price - e.buy_price) * e.quantity - e.fees;
        set_pnl(calculated_pnl);
        calculated_pnl
    };

    let handle_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let calculated_pnl = calculate_pnl();

        // Simulate backend save (replace with actual API call later)
        let entry_with_pnl = {
            let mut e = entry.get();
            e.notes = format!("PnL: {}", calculated_pnl); // Temporary placeholder
            e
        };
        log::info!("Saving entry: {:?}", entry_with_pnl);

        // Reset form
        set_entry(JournalEntry {
            symbol: "".to_string(),
            quantity: 0.0,
            buy_price: 0.0,
            sell_price: 0.0,
            action: "Buy".to_string(),
            fees: 0.0,
            notes: "".to_string(),
        });
        set_pnl(0.0);

        // Toast placeholder (log for now)
        log::info!("Journal entry saved successfully");
    };

    view! {
        <div class="glass-card rounded-xl p-6 animate-fade-in">
            <h3 class="text-xl font-medium mb-6">"New Journal Entry"</h3>
            <form on:submit=handle_submit class="space-y-6">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div>
                        <label for="symbol" class="block text-sm font-medium mb-1">"Symbol"</label>
                        <input
                            id="symbol"
                            name="symbol"
                            value=move || entry.get().symbol
                            on:input=move |ev| set_entry.update(|e| e.symbol = event_target_value(&ev))
                            placeholder="e.g. NIFTY50"
                            class="form-input w-full dark:bg-gray-800 dark:text-white border-gray-300 dark:border-gray-700"
                            required
                        />
                    </div>
                    <div>
                        <label for="quantity" class="block text-sm font-medium mb-1">"Quantity"</label>
                        <input
                            id="quantity"
                            name="quantity"
                            type="number"
                            value=move || entry.get().quantity.to_string()
                            on:input=move |ev| set_entry.update(|e| e.quantity = event_target_value(&ev).parse().unwrap_or(0.0))
                            placeholder="0"
                            min="0"
                            class="form-input w-full dark:bg-gray-800 dark:text-white border-gray-300 dark:border-gray-700"
                            required
                        />
                    </div>
                    <div>
                        <label for="buyPrice" class="block text-sm font-medium mb-1">"Buy Price"</label>
                        <input
                            id="buyPrice"
                            name="buyPrice"
                            type="number"
                            value=move || entry.get().buy_price.to_string()
                            on:input=move |ev| set_entry.update(|e| e.buy_price = event_target_value(&ev).parse().unwrap_or(0.0))
                            placeholder="0.00"
                            step="0.01"
                            min="0"
                            class="form-input w-full dark:bg-gray-800 dark:text-white border-gray-300 dark:border-gray-700"
                            required
                        />
                    </div>
                    <div>
                        <label for="sellPrice" class="block text-sm font-medium mb-1">"Sell Price"</label>
                        <input
                            id="sellPrice"
                            name="sellPrice"
                            type="number"
                            value=move || entry.get().sell_price.to_string()
                            on:input=move |ev| set_entry.update(|e| e.sell_price = event_target_value(&ev).parse().unwrap_or(0.0))
                            placeholder="0.00"
                            step="0.01"
                            min="0"
                            class="form-input w-full dark:bg-gray-800 dark:text-white border-gray-300 dark:border-gray-700"
                            required
                        />
                    </div>
                    <div>
                        <label for="action" class="block text-sm font-medium mb-1">"Action"</label>
                        <select
                            id="action"
                            name="action"
                            value=move || entry.get().action
                            on:change=move |ev| set_entry.update(|e| e.action = event_target_value(&ev))
                            class="form-input w-full dark:bg-gray-800 dark:text-white border-gray-300 dark:border-gray-700"
                            required
                        >
                            <option value="Buy">"Buy"</option>
                            <option value="Sell">"Sell"</option>
                        </select>
                    </div>
                    <div>
                        <label for="fees" class="block text-sm font-medium mb-1">"Fees"</label>
                        <input
                            id="fees"
                            name="fees"
                            type="number"
                            value=move || entry.get().fees.to_string()
                            on:input=move |ev| set_entry.update(|e| e.fees = event_target_value(&ev).parse().unwrap_or(0.0))
                            placeholder="0.00"
                            step="0.01"
                            min="0"
                            class="form-input w-full dark:bg-gray-800 dark:text-white border-gray-300 dark:border-gray-700"
                        />
                    </div>
                </div>
                <div>
                    <label for="notes" class="block text-sm font-medium mb-1">"Notes"</label>
                    <textarea
                        id="notes"
                        name="notes"
                        value=move || entry.get().notes
                        on:input=move |ev| set_entry.update(|e| e.notes = event_target_value(&ev))
                        rows=4
                        placeholder="Add your trade notes here..."
                        class="form-input w-full dark:bg-gray-800 dark:text-white border-gray-300 dark:border-gray-700"
                    />
                </div>
                <div class="flex items-center justify-between pt-4 border-t border-gray-200 dark:border-gray-800">
                    <div class="flex items-baseline">
                        <span class="text-sm font-medium mr-2">"PnL:"</span>
                        <span class=move || format!(
                            "text-xl font-medium {}",
                            if pnl.get() > 0.0 { "text-green-600 dark:text-green-400" }
                            else if pnl.get() < 0.0 { "text-red-600 dark:text-red-400" }
                            else { "" }
                        )>
                            {move || format!("{:.2}", pnl.get())}
                        </span>
                    </div>
                    <div class="flex gap-2">
                        <button
                            type="button"
                            class="border border-gray-300 dark:border-gray-700 px-4 py-2 rounded hover:bg-gray-100 dark:hover:bg-gray-900"
                            on:click=move |_| {
                                set_entry(JournalEntry {
                                    symbol: "".to_string(),
                                    quantity: 0.0,
                                    buy_price: 0.0,
                                    sell_price: 0.0,
                                    action: "Buy".to_string(),
                                    fees: 0.0,
                                    notes: "".to_string(),
                                });
                                set_pnl(0.0);
                            }
                        >
                            "Cancel"
                        </button>
                        <button
                            type="button"
                            class="bg-gray-500 text-white px-4 py-2 rounded hover:bg-gray-600"
                            on:click=move |_| { calculate_pnl(); }
                        >
                            "Calculate"
                        </button>
                        <button
                            type="submit"
                            class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-500/90"
                        >
                            "Save"
                        </button>
                    </div>
                </div>
            </form>
        </div>
    }
}
