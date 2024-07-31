use common::User;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::api::users::users_get;

#[derive(Debug, Properties, PartialEq)]
struct UserListProps {
    users: Vec<User>,
}

#[function_component(UserList)]
fn user_list(props: &UserListProps) -> Html {
    props
        .users
        .iter()
        .map(|user| {
            html! {
                <li>{&user.username}</li>
            }
        })
        .collect::<Html>()
}

#[function_component(Admin)]
pub fn admin() -> Html {
    let users = use_state(|| Vec::<User>::new());

    {
        let users = users.clone();
        use_effect_with((), move |_| {
            let users = users.clone();
            spawn_local(async move {
                let fetched_users = users_get().await.unwrap();
                users.set(fetched_users);
            });
        });
    }

    html! {
        <div>
            <UserList users={(*users).clone()} />
        </div>
    }
}
