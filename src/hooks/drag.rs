#![allow(non_snake_case)]

use std::marker::PhantomData;

use dioxus::prelude::*;

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
pub struct UseDraggable<'a> {
    pub class: UseRef<&'static str>,
    pub draggable: UseRef<bool>,
    pub onmounted: EventHandler<'a, MountedEvent>,
    pub onmousedown: EventHandler<'a, MouseEvent>,
    pub ondragstart: EventHandler<'a, DragEvent>,
    pub ondragend: EventHandler<'a, DragEvent>,
    pub activator: UseDraggableActivator<'a>,
}

pub struct UseDraggableActivator<'a> {
    pub onmounted: EventHandler<'a, MountedEvent>,
    pub onmousedown: EventHandler<'a, MouseEvent>,
}

pub fn use_draggable<'a, C: 'static>(cx: &'a ScopeState, id: String) -> UseDraggable<'a> {
    let state = use_dnd_state::<C>(cx);
    let node_ref: &UseRef<Option<web_sys::Element>> = use_ref(cx, || None);
    let activator_node_ref: &UseRef<Option<web_sys::Element>> = use_ref(cx, || None);
    let class = use_ref(cx, || "");
    let draggable = use_ref(cx, || false);

    use_effect(cx, &state.read().dragging, |dragging| {
        to_owned![id, dragging, draggable];
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

    to_owned![id];

    UseDraggable {
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
        onmousedown: {
            to_owned![id, state];
            cx.event_handler(move |e: MouseEvent| {
                if activator_node_ref.read().is_none() {
                    e.stop_propagation();
                    if node_ref.read().is_some() {
                        state.write().dragging = id.to_string();
                    }
                }
            })
        },
        ondragstart: cx.event_handler(move |_: DragEvent| {
            log::debug!("ondragstart");
        }),
        ondragend: {
            to_owned![state];
            cx.event_handler(move |_: DragEvent| {
                log::debug!("ondragend");
                state.write().dragging = "".to_string();
            })
        },
        activator: UseDraggableActivator {
            onmounted: cx.event_handler(|e: MountedEvent| {
                let el = e
                    .get_raw_element()
                    .expect("expecting raw element")
                    .downcast_ref::<web_sys::Element>()
                    .expect("expecting Element");
                activator_node_ref.write().replace(el.clone());
            }),
            onmousedown: {
                to_owned![id, state, node_ref];
                cx.event_handler(move |e: MouseEvent| {
                    e.stop_propagation();
                    if node_ref.read().is_some() {
                        state.write().dragging = id.to_string();
                    }
                })
            },
        },
    }
}

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
