use yew::prelude::*;
use yewdux::prelude::*;

pub struct VmRemoteControl {
    props: VmRemoteControlProps,
    is_dragging: bool,
    position: (i32, i32),
    last_mouse_position: (i32, i32),
}

#[derive(Properties, Clone, PartialEq)]
pub struct VmRemoteControlProps {
    pub children: Children,
}

pub enum VmRemoteControlMessage {
    MouseDown((i32, i32)),
    MouseMove((i32, i32)),
    MouseUp,
}

impl Component for VmRemoteControl {
    type Message = VmRemoteControlMessage;
    type Properties = VmRemoteControlProps;

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
            VmRemoteControlMessage::MouseDown(coords) => {
                self.is_dragging = true;
                self.last_mouse_position = coords;
            }
            VmRemoteControlMessage::MouseMove(coords) => {
                if self.is_dragging {
                    let dx = coords.0 - self.last_mouse_position.0;
                    let dy = coords.1 - self.last_mouse_position.1;
                    self.position.0 -= dx;
                    self.position.1 -= dy;
                    self.last_mouse_position = coords;
                }
            }
            VmRemoteControlMessage::MouseUp => {
                self.is_dragging = false;
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _props: &VmRemoteControlProps) -> bool {
        self.props = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div
                class="cursor-grab border rounded border-gray-300 p-4 w-64 min-w-48 max-w-96 h-24 select-none bg-white shadow-md"
                style={format!("position: absolute; right: {}px; bottom: {}px;", self.position.0, self.position.1)}
                onmousedown={ctx.link().callback(|e: MouseEvent| VmRemoteControlMessage::MouseDown((e.client_x(), e.client_y())))}
                onmousemove={ctx.link().callback(|e: MouseEvent| VmRemoteControlMessage::MouseMove((e.client_x(), e.client_y())))}
                onmouseup={ctx.link().callback(|_| VmRemoteControlMessage::MouseUp)}
            >
                    { self.props.children.clone() }
            </div>
        }
    }
}
