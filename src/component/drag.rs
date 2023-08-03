#![allow(non_snake_case)]

use dioxus::prelude::*;
use gloo_events::EventListener;
use gloo_utils::body;

pub struct State {
    pub dragging: bool,
}

impl Default for State {
    fn default() -> Self {
        Self { dragging: false }
    }
}

pub fn use_dnd_state<'a>(cx: &ScopeState) -> &UseSharedState<State> {
    use_shared_state::<State>(cx).expect("You must provide DndState")
}

#[inline_props]
pub fn DndState<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    use_shared_state_provider(cx, || State::default());
    render! {
        children
    }
}

#[inline_props]
pub fn Draggable<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    let state = use_dnd_state(cx);
    render! {
        div {
            draggable: state.read().dragging,
            children
        }
    }
}

#[inline_props]
pub fn DragHandle<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    let state = use_dnd_state(cx);
    render! {
        div {
            onmousedown: move |_| {
                state.write().dragging = true;
                let s = state.clone();
                EventListener::once(&body(), "mouseup", move |_e| {
                    log::debug!("mouseup");
                    s.write().dragging = false;
                }).forget();
            },
            children
        }
    }
}

#[inline_props]
pub fn Droppable<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx!(div {
        onmounted: move |e| {
            log::debug!(
                "{:?}",
                e.get_raw_element()
                    .unwrap()
                    .downcast_ref::<web_sys::Element>()
            );
        },
        ondragenter: move |e| {
            e.stop_propagation();
        },
        ondragover: move |e| {
            e.stop_propagation();
        },
        children
    }))
}

// pub struct UseDrag<'a> {
//     dragRef: UseRef<Option<web_sys::Element>>,
//     pub onmousedown: EventHandler<'a, MouseEvent>,
// }
//
// pub fn use_drag(cx: &ScopeState) -> UseDrag {
//     let dragRef = use_ref(cx, move || None);
//
//     UseDrag {
//         dragRef: dragRef.clone(),
//         onmousedown: cx.event_handler(move |_: MouseEvent| {
//             dioxus::prelude::con
//             use_dnd_state(cx.consume_context()).write().dragging = true;
//         }),
//     }
// }

// impl UseDrag<'_> {
//     pub fn set_drag_ref(&self, e: MountedEvent) {
//         let el = e
//             .get_raw_element()
//             .expect("no raw element")
//             .downcast_ref::<web_sys::Element>()
//             .expect("no element");
//         log::debug!("{:?}", el);
//         self.dragRef.set(Some(el.clone()));
//     }
// }
