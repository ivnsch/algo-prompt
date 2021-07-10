use algonaut::transaction::url::LinkableTransaction;
use qrcode::{render::svg, EcLevel, QrCode, Version};
use yew::{
    html, web_sys::Element, Component, ComponentLink, Html, NodeRef, Properties, ShouldRender,
};

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct AlgoPromptProps {
    pub transaction: LinkableTransaction,
}

pub struct AlgoPrompt {
    props: AlgoPromptProps,
    node_ref: NodeRef,
}

impl Component for AlgoPrompt {
    type Message = ();
    type Properties = AlgoPromptProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unreachable!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div ref=self.node_ref.clone()/>
        }
    }

    // https://github.com/yewstack/yew/issues/1281#issuecomment-637508696
    fn rendered(&mut self, _first_render: bool) {
        let el = self.node_ref.cast::<Element>().unwrap();
        el.set_inner_html(&qr_code_svg(&self.props.transaction));
    }
}

// TODO derive qrcode::Version from URL length and qrcode::EcLevel.
// currently hardcoded to 11, tested with u64::MAX asset id/amount and 100 characters note/label.
fn qr_code_svg(transaction: &LinkableTransaction) -> String {
    let code = QrCode::with_version(
        transaction.as_url().to_string(),
        Version::Normal(11),
        EcLevel::L,
    );

    match &code {
        Ok(code) => render(code),
        Err(e) => {
            // TODO show error in UI.
            panic!("Error: {:?}", e)
        }
    }
}

fn render(code: &QrCode) -> String {
    code.render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
}
