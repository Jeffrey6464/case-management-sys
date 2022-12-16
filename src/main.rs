use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let n = use_state(|| 3);

    let a = use_state(|| 1);
    let b = use_state(|| 2);

    let onclick = {
        let n = n.clone();
        let a = a.clone();
        let b = b.clone();

        move |_| {
            b.set(*b + *a);
            a.set(*b);
            n.set(*n + 1);
        }
    };

    html! {
        <div>

        <div id="title" style="text-align:center">
            <h1>
                {"CASE MANAGEMENT SYSTEM"}
            </h1>

            <h4>
                {"by Dylan Zeilinger-Johnson and Audrey Valentine"}
            </h4>
        </div>
        
        <div id="fibbonacci">
            <p>{"The fibbonacci when:"}<b>{" n="}{ *n }</b>{" is "}<b>{ *b }</b></p> //The fibbonacci when n=<n> is <b>
            <button {onclick}>{ "n+1" }</button>
        </div>

        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}