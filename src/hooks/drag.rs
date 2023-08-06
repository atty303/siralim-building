#![allow(non_snake_case)]

use std::marker::PhantomData;

use dioxus::prelude::*;
use gloo_events::EventListener;

pub struct Context<C> {
    pub dragging: String,
    phantom: PhantomData<C>,
}

impl<C> Default for Context<C> {
    fn default() -> Self {
        Self {
            dragging: String::new(),
            phantom: PhantomData,
        }
    }
}

fn use_dnd_state<'a, C: 'static>(cx: &ScopeState) -> &UseSharedState<Context<C>> {
    use_shared_state::<Context<C>>(cx).expect("You must provide DndState")
}

#[derive(Props)]
pub struct DndContextProps<'a> {
    children: Element<'a>,
}

pub fn DndContext<'a, C: 'static>(cx: Scope<'a, DndContextProps<'a>>) -> Element<'a> {
    use_shared_state_provider(cx, || Context::<C>::default());
    render! {
        &cx.props.children
    }
}

//#[derive(Clone)]
pub struct UseDraggable<'a, C: 'static> {
    id: &'a str,
    state: UseSharedState<Context<C>>,
    node_ref: UseRef<Option<web_sys::Element>>,
    pub class: UseRef<&'static str>,
    pub draggable: UseRef<bool>,
    pub onmounted: EventHandler<'a, MountedEvent>,
    pub onmousedown: EventHandler<'a, MouseEvent>,
}

pub fn use_draggable<'a, C: 'static>(cx: &'a ScopeState, id: &'a str) -> UseDraggable<'a, C> {
    let state = use_dnd_state::<C>(cx);
    let node_ref: &UseRef<Option<web_sys::Element>> = use_ref(cx, || None);
    let class = use_ref(cx, || "");
    let draggable = use_ref(cx, || false);

    use_effect(cx, &state.read().dragging, |dragging| {
        to_owned![id, dragging, class, draggable];
        async move {
            if dragging == id {
                //class.set("");
                draggable.set(true);
            } else {
                //class.set("");
                draggable.set(false);
            }
        }
    });

    UseDraggable {
        id,
        state: state.clone(),
        node_ref: node_ref.clone(),
        class: class.clone(),
        draggable: draggable.clone(),
        onmounted: cx.event_handler(|e: MountedEvent| {
            let el = e
                .get_raw_element()
                .expect("expecting raw element")
                .downcast_ref::<web_sys::Element>()
                .expect("expecting Element");
            node_ref.write().replace(el.clone());
        }),
        onmousedown: cx.event_handler(|e: MouseEvent| {
            e.stop_propagation();
            state.write().dragging = id.to_string();
            to_owned![state];
            EventListener::once(&gloo_utils::body(), "mouseup", move |_e| {
                log::debug!("mouseup");
                state.write().dragging = "".to_string();
            })
            .forget();
        }),
    }
}

#[derive(Props)]
pub struct DraggableProps<'a> {
    draggable_id: &'a str,
    children: Element<'a>,
}

pub fn Draggable<'a, C: 'static>(cx: Scope<'a, DraggableProps<'a>>) -> Element<'a> {
    let draggable = use_draggable::<C>(cx, cx.props.draggable_id);

    let x = render! {
        div {
            class: "{draggable.class.read()}",
            draggable: *draggable.draggable.read(),
            onmounted: move |e| {
                draggable.onmounted.call(e);
            },
            onmousedown: move |e| {
                draggable.onmousedown.call(e);
            },
            &cx.props.children
        }
    };
    x
}

// #[inline_props]
// pub fn DragHandle<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
//     let state = use_dnd_state(cx);
//     render! {
//         div {
//             onmousedown: move |_| {
//                 state.write().dragging = true;
//                 let s = state.clone();
//                 EventListener::once(&body(), "mouseup", move |_e| {
//                     log::debug!("mouseup");
//                     s.write().dragging = false;
//                 }).forget();
//             },
//             children
//         }
//     }
// }

// #[inline_props]
// pub fn Droppable<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
//     cx.render(rsx!(div {
//         onmounted: move |e| {
//             log::debug!(
//                 "{:?}",
//                 e.get_raw_element()
//                     .unwrap()
//                     .downcast_ref::<web_sys::Element>()
//             );
//         },
//         ondragenter: move |e| {
//             e.stop_propagation();
//         },
//         ondragover: move |e| {
//             e.stop_propagation();
//         },
//         children
//     }))
// }

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
