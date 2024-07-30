use views::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod views;

#[derive(Clone, Routable, PartialEq, Debug)]
enum Route {
    #[at("/")]
    Home,
    #[at("/scripts")]
    Scripts,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <header>
                <nav class="flex bg-white border-gray-200 px-4 lg:px-6">
                    <div class="flex">
                        <a href="/" class="flex items-center py-4 px-2">
                            {"Home"}
                        </a>
                        <a href="/scripts" class="flex items-center py-4 px-2">
                            {"Scripts"}
                        </a>
                    </div>
                    <div class="flex ml-auto">
                        <a href="/login" class="flex items-center py-4 px-2">
                            {"Login"}
                        </a>
                        <a href="/register" class="flex items-center py-4 px-2">
                            {"Register"}
                        </a>
                    </div>
                </nav>
            </header>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Scripts => html! { <div>{"Scripts not implemented"}</div> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
