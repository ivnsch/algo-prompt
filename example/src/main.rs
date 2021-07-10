use algo_prompt::AlgoPrompt;
use algonaut::{
    core::MicroAlgos,
    transaction::url::{LinkableTransaction, LinkableTransactionBuilder, Note},
};
use yew::prelude::*;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <AlgoPrompt transaction=some_transaction()></AlgoPrompt>
            </div>
        }
    }
}

fn some_transaction() -> LinkableTransaction {
    LinkableTransactionBuilder::payment(
        "VL5LJ6O75TV4Q4I54NKJ4EDNQDZX4TWXSL3CSQOA25DQJ4WQ3QBQJNJUQU"
            .parse()
            .unwrap(),
        MicroAlgos(1_000_000),
    )
    .label("My transaction name")
    .note(Note::Editable("Hello from Rust 🦀".to_owned()))
    .build()
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
