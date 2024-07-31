use crate::{api, components::prelude::*};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
    let name_input_ref = NodeRef::default();
    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let onchange = Callback::from(move |input: String| {
        gloo_console::log!("Input: {}", input);
    });

    let on_submit = {
        let name_input_ref = name_input_ref.clone();
        let email_input_ref = email_input_ref.clone();
        let password_input_ref = password_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let name_input_ref = name_input_ref.clone();
            let email_input_ref = email_input_ref.clone();
            let password_input_ref = password_input_ref.clone();

            wasm_bindgen_futures::spawn_local(async move {
                event.prevent_default();

                let name = name_input_ref.cast::<HtmlInputElement>().unwrap().value();
                let email = email_input_ref.cast::<HtmlInputElement>().unwrap().value();
                let password = password_input_ref
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();

                let user = api::user::new_user::CreateUser {
                    username: name,
                    email,
                    password,
                };

                match api::user::new_user::register_user(&user).await {
                    Ok(user) => {
                        gloo_console::log!(
                            "User: {}",
                            serde_wasm_bindgen::to_value(&user).unwrap()
                        );
                    }
                    Err(err) => {}
                };
            });
        })
    };

    html! {
        <div class="flex flex-col min-h-full justify-center">
            <form onsubmit={on_submit} class="space-y-6">
                <div class="flex flex-col mx-4 lg:mx-40">
                    <FormInput onchange={onchange.clone()} input_ref={name_input_ref} label="Username" name="username" />
                    <FormInput onchange={onchange.clone()} input_ref={email_input_ref} input_type={InputType::Email} label="Email" name="email" />
                    <FormInput onchange={onchange.clone()} input_ref={password_input_ref} input_type={InputType::Password} label="Password" name="password" />
                </div>
                <div class="flex flex-col">
                    <button type="submit">{"Register"}</button>
                </div>
            </form>
        </div>
    }
}
