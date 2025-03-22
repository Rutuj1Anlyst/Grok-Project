use leptos::*;
use std::collections::VecDeque;
use std::rc::Rc;

const TOAST_LIMIT: usize = 1;
const TOAST_REMOVE_DELAY: i32 = 1_000_000;

#[derive(Clone)]
pub struct ToasterToast {
    id: String,
    title: Option<String>,
    description: Option<String>,
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
}

pub fn use_toast() -> (ReadSignal<VecDeque<ToasterToast>>, Rc<dyn Fn(String, Option<String>, Option<String>)>) {
    let (toasts, set_toasts) = create_signal(VecDeque::new());

    let toast = Rc::new(move |id: String, title: Option<String>, description: Option<String>| {
        let (open, set_open) = create_signal(true);

        // Auto-remove after delay
        set_timeout(
            move || {
                set_open.set(false);
                set_toasts.update(|t| {
                    t.retain(|toast| toast.id != id);
                });
            },
            std::time::Duration::from_millis(TOAST_REMOVE_DELAY as u64),
        );

        let new_toast = ToasterToast {
            id: id.clone(),
            title,
            description,
            open,
            set_open,
        };

        set_toasts.update(|t| {
            t.push_front(new_toast);
            while t.len() > TOAST_LIMIT {
                t.pop_back();
            }
        });
    }) as Rc<dyn Fn(String, Option<String>, Option<String>)>;

    let dismiss = move |toast_id: Option<String>| {
        if let Some(id) = toast_id {
            set_toasts.update(|t| {
                if let Some(toast) = t.iter().find(|t| t.id == id) {
                    toast.set_open.set(false);
                }
            });
        } else {
            set_toasts.update(|t| {
                for toast in t.iter() {
                    toast.set_open.set(false);
                }
            });
        }
    };

    let toast_fn = Rc::new(move |title: String, description: String| {
        let id = uuid::Uuid::new_v4().to_string();
        toast(id.clone(), Some(title), Some(description));
        dismiss(Some(id));
    });

    (toasts, toast_fn)
}

pub fn toast(title: String, description: String) {
    let (toasts, toast_fn) = use_toast();
    toast_fn(title, Some(description));
}
