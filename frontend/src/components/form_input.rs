use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input_type: InputType,
    pub label: String,
    pub name: String,
    pub input_ref: NodeRef,
}

#[function_component(FormInput)]
pub fn form_input_component(props: &Props) -> Html {
    let input_type = props.input_type.to_html_string();

    html! {
        <div>
            <label for={props.name.clone()} class="block text-sm font-medium leading-6 text-gray-900">{props.label.clone()}</label>
            <div>
                <input  type={input_type.to_string()} id={props.name.clone()} name={props.name.clone()}
                        ref={props.input_ref.clone()}
                        class= "block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300
                            placeholder:text-gray-400 
                                focus:ring-2 focus:ring-inset focus:ring-indigo-600 
                                sm:text-sm sm:leading-6" />
            </div>
        </div>
    }
}

impl InputType {
    fn to_html_string(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
        }
    }
}

impl Default for InputType {
    fn default() -> Self {
        InputType::Text
    }
}
