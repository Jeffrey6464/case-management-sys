use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let n = use_state(|| 0);

    let a = use_state(|| 1);
    let b = use_state(|| 1);
    let c = use_state(|| 2);

    let onclick = {
        let n = n.clone();
        let a = a.clone();
        let b = b.clone();
        let c = c.clone();

        move |_| {
            c.set(*c + *b);
            a.set(*b);
            b.set(*c);

            n.set(*n + 1);
        }
    };

    html! {
        <div>
            <p>{"Click this for the fibbonacci when: n="}{ *n }</p>
            <button {onclick}>{ "+1" }</button>
            <p>{"a="}{ *a }</p>
            <p>{"b="}{ *b }</p>
            <p>{"c="}{ *c }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}