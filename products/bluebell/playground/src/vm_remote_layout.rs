use yew::prelude::*;

pub struct VmRemoteControlLayout {
    props: VmRemoteControlLayoutProps,
    is_dragging: bool,
    position: (i32, i32),
    last_mouse_position: (i32, i32),
}

#[derive(Properties, Clone, PartialEq)]
pub struct VmRemoteControlLayoutProps {
    pub children: Children,
}

pub enum VmRemoteControlLayoutMessage {
    MouseDown((i32, i32)),
    MouseMove((i32, i32)),
    MouseUp,
}

impl Component for VmRemoteControlLayout {
    type Message = VmRemoteControlLayoutMessage;
    type Properties = VmRemoteControlLayoutProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self {
            props,
            is_dragging: false,
            position: (300, 100),
            last_mouse_position: (0, 0),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            VmRemoteControlLayoutMessage::MouseDown(coords) => {
                self.is_dragging = true;
                self.last_mouse_position = coords;
            }
            VmRemoteControlLayoutMessage::MouseMove(coords) => {
                if self.is_dragging {
                    let dx = coords.0 - self.last_mouse_position.0;
                    let dy = coords.1 - self.last_mouse_position.1;
                    self.position.0 -= dx;
                    self.position.1 -= dy;
                    self.last_mouse_position = coords;
                }
            }
            VmRemoteControlLayoutMessage::MouseUp => {
                self.is_dragging = false;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _props: &VmRemoteControlLayoutProps) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let is_dragging = self.is_dragging;
        html! {
            <div
                class="cursor-grab border rounded-lg bg-gray-700  border-black p-5 w-72 min-w-56 max-w-100 min-h-28 max-h-64 shadow-xl transition-transform ease-in-out duration-300"
                style={format!("position: absolute; right: {}px; bottom: {}px;", self.position.0, self.position.1)}
                onmousedown={ctx.link().callback(|e: MouseEvent| {
                    e.prevent_default();
                    VmRemoteControlLayoutMessage::MouseDown((e.client_x(), e.client_y()))
                })}
                onmousemove={ctx.link().callback(move |e: MouseEvent| {
                    if is_dragging {
                        e.prevent_default();
                        VmRemoteControlLayoutMessage::MouseMove((e.client_x(), e.client_y()))
                    } else {
                        VmRemoteControlLayoutMessage::MouseUp
                    }
                })}
                onmouseup={ctx.link().callback(|_| VmRemoteControlLayoutMessage::MouseUp)}
            >
                { self.props.children.clone() }
            </div>
        }

        /*
        html! {
            <div
                class="cursor-grab border rounded border-gray-300 p-4 w-64 min-w-48 max-w-96 h-24 select-none bg-white shadow-md"
                style={format!("position: absolute; right: {}px; bottom: {}px;", self.position.0, self.position.1)}
                onmousedown={ctx.link().callback(|e: MouseEvent| VmRemoteControlLayoutMessage::MouseDown((e.client_x(), e.client_y())))}
                onmousemove={ctx.link().callback(|e: MouseEvent| VmRemoteControlLayoutMessage::MouseMove((e.client_x(), e.client_y())))}
                onmouseup={ctx.link().callback(|_| VmRemoteControlLayoutMessage::MouseUp)}
            >
                    { self.props.children.clone() }
            </div>
        }
        */
    }
}
