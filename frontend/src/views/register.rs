use crate::components::prelude::*;
use yew::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
    let on_submit = { Callback::from(move |event: SubmitEvent| {}) };

    html! {
        <div class="flex flex-col min-h-full justify-center">
            <form action="/register" mathod="POST" class="space-y-6">
                <div class="flex flex-col mx-4 lg:mx-40">
                    <FormInput label="Username" name="username" />
                    <FormInput input_type={InputType::Email} label="Email" name="email" />
                    <FormInput input_type={InputType::Password} label="Password" name="password" />
                </div>
                <div class="flex flex-col">
                    <button type="submit">{"Register"}</button>
                </div>
            </form>
        </div>
    }
}
