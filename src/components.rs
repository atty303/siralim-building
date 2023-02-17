pub mod app;
pub mod icon;
pub mod modal;
pub mod party;

// #[derive(Props)]
// pub struct ModalProps<'a> {
//     show: bool,
//     on_request_close: EventHandler<'a, ()>,
//     children: Element<'a>,
// }
//
// pub fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element {
//     if cx.props.show {
//         cx.render(rsx! {
//             div {
//                 class: "modal-overlay",
//                 div {
//                     class: "modal",
//                     div {
//                         class: "title",
//                         button { onclick: move |_|  cx.props.on_request_close.call(()), "close" }
//                     }
//                     div {
//                         class: "content",
//                         &cx.props.children
//                     }
//                 }
//             }
//         })
//     } else {
//         None
//     }
// }
//
