use yew::prelude::*;

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <div class="flex flex-col min-h-full justify-center">
            <form action="#" mathod="POST" class="space-y-6">
                <div class="flex flex-col">
                    <label for="username" class="block text-sm font-medium leading-6 text-gray-900">{"Username"}</label>
                    <div>
                        <input type="text" id="username" name="username"
                                class= "block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300
                                    placeholder:text-gray-400 
                                        focus:ring-2 focus:ring-inset focus:ring-indigo-600 
                                        sm:text-sm sm:leading-6" />
                    </div>
                </div>
                <div class="flex flex-col">
                    <button type="submit">{"Register"}</button>
                </div>
            </form>
        </div>
    }
}
