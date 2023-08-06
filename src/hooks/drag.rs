#![allow(non_snake_case)]

use dioxus::core::DynamicNode;
use std::marker::PhantomData;

use dioxus::prelude::*;

#[derive(Debug)]
pub struct DragEndEvent {
    pub active_id: String,
    pub over_id: String,
}

struct Context<C> {
    dragging: String,
    on_drag_end: UseRef<Box<dyn Fn(DragEndEvent)>>,
    phantom: PhantomData<C>,
}

fn use_dnd_state<'a, C: 'static>(cx: &ScopeState) -> &UseSharedState<Context<C>> {
    use_shared_state::<Context<C>>(cx).expect("You must provide DndState")
}

pub struct UseDndContext {
    on_drag_end: UseRef<Box<dyn Fn(DragEndEvent)>>,
}

#[derive(Props)]
pub struct DndContextProps<'a> {
    on_drag_end: UseRef<Box<dyn Fn(DragEndEvent)>>,
    children: Element<'a>,
}

impl UseDndContext {
    pub fn component<'a, C: 'static>(
        &self,
        cx: &'a ScopeState,
        child: Element<'a>,
    ) -> DynamicNode<'a> {
        cx.component(
            |cx| {
                let context = Context::<C> {
                    dragging: String::new(),
                    on_drag_end: cx.props.on_drag_end.clone(),
                    phantom: PhantomData,
                };
                use_shared_state_provider(cx, || context);
                render! { &cx.props.children }
            },
            DndContextProps {
                on_drag_end: self.on_drag_end.clone(),
                children: render! { child },
            },
            "DndContext",
        )
    }
}

pub fn use_dnd_context<C: 'static>(
    cx: &ScopeState,
    on_drag_end_fn: Box<dyn Fn(DragEndEvent) + 'static>,
) -> UseDndContext {
    let on_drag_end: &UseRef<Box<dyn Fn(DragEndEvent)>> = use_ref(cx, || on_drag_end_fn);
    //*on_drag_end.write_silent() = Some(Box::new(on_drag_end_fn));

    UseDndContext {
        on_drag_end: on_drag_end.clone(),
    }
}

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

pub fn use_draggable<C: 'static>(cx: &ScopeState, id: String) -> UseDraggable {
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
            // log::debug!("ondragstart");
        }),
        ondragend: {
            to_owned![state];
            cx.event_handler(move |_: DragEvent| {
                // log::debug!("ondragend");
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

pub struct UseDroppable<'a> {
    pub ondragover: EventHandler<'a, DragEvent>,
    pub ondrop: EventHandler<'a, DragEvent>,
}

pub fn use_droppable<C: 'static>(cx: &ScopeState, id: String) -> UseDroppable {
    let state = use_dnd_state::<C>(cx);

    UseDroppable {
        ondragover: { cx.event_handler(move |_: DragEvent| {}) },
        ondrop: {
            to_owned![state, id];
            cx.event_handler(move |e: DragEvent| {
                let active_id = &state.read().dragging;
                state.read().on_drag_end.read()(DragEndEvent {
                    active_id: active_id.clone(),
                    over_id: id.clone(),
                });
            })
        },
    }
}
