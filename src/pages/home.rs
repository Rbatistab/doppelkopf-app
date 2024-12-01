use yew::{html, Component, Context, Html, Properties};

enum HomePageMsg {}

#[derive(PartialEq, Properties)]
struct HomePageProps {}

struct HomePage {}

impl Component for HomePage {
    type Message = HomePageMsg;
    type Properties = HomePageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: Context<Self>) -> Html {
        html! {
            <></>
        }
    }
}
