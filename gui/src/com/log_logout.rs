use yew::prelude::*;
use yew::function_component;
use yew::html;
//use wasm_bindgen_futures::spawn_local;
//use gloo_net::http::Request;

#[function_component(LoginLogoutComponent)]
pub fn login_logout_component() -> Html {
    let is_logged_in = use_state(|| false);

    // let on_login = {
    //     let is_logged_in = is_logged_in.clone();
    //     Callback::from(move |_| {
    //         let is_logged_in = is_logged_in.clone();
    //         spawn_local(async move {
    //             let response = Request::post("http://127.0.0.1:3000/login")
    //                 .header("Content-Type", "application/json")
    //                 .send()
    //                 .await;
    //
    //             match response {
    //                 Ok(res) => {
    //                     if res.status() == 200 {
    //                         is_logged_in.set(true);
    //                         gloo::console::log!("Login successful!");
    //                     } else {
    //                         gloo::console::log!("Login failed!");
    //                     }
    //                 },
    //                 Err(err) => {
    //                     gloo::console::log!("Request error:", err.to_string());
    //                 }
    //             }
    //         });
    //     })
    // };

    let on_logout = {
        let is_logged_in = is_logged_in.clone();
        Callback::from(move |_| is_logged_in.set(false))
    };

    html! {
        html! {
        <div class="flex justify-center items-center min-h-screen bg-gray-100">
            <div class="w-80 p-4 shadow-lg bg-white rounded-lg">
                <div class="text-center">
                    <h2 class="text-xl font-bold mb-4">
                        { if *is_logged_in { "Witaj, Użytkowniku!" } else { "Nie jesteś zalogowany" } }
                    </h2>
                    {
                        if *is_logged_in {
                            html! {
                                <button onclick={on_logout} class="w-full bg-red-500 text-white p-2 rounded-lg">{"Wyloguj"}</button>
                            }
                        } else {
                            html! {
                                <a href="http://127.0.0.1:3000/login" class="w-full bg-green-500 text-white p-2 rounded-lg hover:bg-green-600 text-center block">
                                    {"Login"}
                                </a>
                            }
                        }
                    }
                </div>
            </div>
        </div>
    }
    }
}