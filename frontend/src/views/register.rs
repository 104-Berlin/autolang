use crate::{api, components::prelude::*, Route};
use common::CreateUserForm;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();

    let name_input_ref = NodeRef::default();
    let email_input_ref = NodeRef::default();
    let password_input_ref = NodeRef::default();

    let on_submit = {
        let name_input_ref = name_input_ref.clone();
        let email_input_ref = email_input_ref.clone();
        let password_input_ref = password_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();

            let name_input_ref = name_input_ref.clone();
            let email_input_ref = email_input_ref.clone();
            let password_input_ref = password_input_ref.clone();

            wasm_bindgen_futures::spawn_local(async move {
                event.prevent_default();

                let name_input = name_input_ref.cast::<HtmlInputElement>().unwrap();
                let email_input = email_input_ref.cast::<HtmlInputElement>().unwrap();
                let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();

                let name = name_input.value();
                let email = email_input.value();
                let password = password_input.value();

                let user = CreateUserForm {
                    username: name,
                    email,
                    password,
                };

                name_input.set_value("");
                email_input.set_value("");
                password_input.set_value("");

                match api::register::register_post(&user).await {
                    Ok(user) => {
                        gloo_console::log!(
                            "User: {}",
                            serde_wasm_bindgen::to_value(&user).unwrap()
                        );
                        navigator.push(&Route::Home);
                    }
                    Err(_) => {
                        //panic!("Failed to register user")
                        gloo_console::error!("Failed to register user");
                    }
                };
            });
        })
    };

    html! {
        <div class="flex flex-col min-h-full justify-center">
            <form onsubmit={on_submit} class="space-y-6">
                <div class="flex flex-col mx-4 lg:mx-40">
                    <FormInput input_ref={name_input_ref} label="Username" name="username" />
                    <FormInput input_ref={email_input_ref} input_type={InputType::Email} label="Email" name="email" />
                    <FormInput input_ref={password_input_ref} input_type={InputType::Password} label="Password" name="password" />
                </div>
                <div class="flex flex-col">
                    <button type="submit">{"Register"}</button>
                </div>
            </form>
        </div>
    }
}
