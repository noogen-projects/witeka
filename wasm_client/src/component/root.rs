use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::{
    dto::User,
    widget::{self, TableCell, TableRow},
};

pub struct Root {
    user_name: String,
    current_page: Page,
    link: ComponentLink<Self>,
}

enum Page {
    Dashboard,
    NewWiki,
    Wiki(String),
}

pub enum Msg {
    NewWiki,
    AddWiki,
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            user_name: "Vasia".to_string(),
            current_page: Page::Dashboard,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewWiki => {
                self.current_page = Page::NewWiki;
                true
            }
            _ => false,
        }
    }

    fn view(&self) -> Html {
        let page = match &self.current_page {
            Page::Dashboard => self.view_dashboard(),
            Page::NewWiki => todo!(),
            Page::Wiki(wiki) => todo!(),
        };

        html! {
            <div class = "drawer-frame-root">
                <aside class = "mdc-drawer mdc-drawer--dismissible">
                    <div class = "mdc-drawer__header">
                        <h3 class = "mdc-drawer__title">{ "Mail" }</h3>
                        <h6 class = "mdc-drawer__subtitle">{ "email@material.io" }</h6>
                    </div>
                    <div class = "mdc-drawer__content">
                        <nav class = "mdc-list">
                            <a class = "mdc-list-item mdc-list-item--activated" href = "#" aria-current = "page">
                                <i class = "material-icons mdc-list-item__graphic" aria-hidden = "true">{ "inbox" }</i>
                                <span class = "mdc-list-item__text">{ "Inbox" }</span>
                            </a>
                            <a class = "mdc-list-item" href = "#">
                                <i class = "material-icons mdc-list-item__graphic" aria-hidden = "true">{ "send" }</i>
                                <span class = "mdc-list-item__text">{ "Outgoing" }</span>
                            </a>
                            <a class = "mdc-list-item" href = "#">
                                <i class = "material-icons mdc-list-item__graphic" aria-hidden = "true">{ "drafts" }</i>
                                <span class = "mdc-list-item__text">{ "Drafts" }</span>
                            </a>
                        </nav>
                    </div>
                </aside>
                <div class = "mdc-drawer-app-content">
                    <header id = "app-bar" class = "mdc-top-app-bar mdc-top-app-bar--fixed drawer-top-app-bar">
                        <div class = "mdc-top-app-bar__row">
                            <section class = "mdc-top-app-bar__section mdc-top-app-bar__section--align-start">
                                <button class = "material-icons mdc-top-app-bar__navigation-icon mdc-icon-button">{ "menu" }</button>
                                <span class = "mdc-top-app-bar__title">{ "Witeka" }</span>
                            </section>
                        </div>
                    </header>
                    <main id = "main-content" class = "drawer-main-content">
                        <div class = "mdc-top-app-bar--fixed-adjust">
                            { page }
                        </div>
                    </main>
                </div>
                <script>{ r"
                    const drawer = mdc.drawer.MDCDrawer.attachTo(document.querySelector('.mdc-drawer'));
                    const topAppBar = mdc.topAppBar.MDCTopAppBar.attachTo(document.getElementById('app-bar'));
                    topAppBar.setScrollTarget(document.getElementById('main-content'));
                    topAppBar.listen('MDCTopAppBar:nav', () => {
                        drawer.open = !drawer.open;
                    });
                " }</script>
             </div>
        }
    }
}

impl Root {
    fn view_dashboard(&self) -> Html {
        let wikis = [TableRow::new(vec![
            TableCell::text("Foo"),
            TableCell::text("2020-05-20"),
        ])];

        html! {
            <div class = "content">
                <h1 class = "mdc-typography--headline4">{ "Wikis" }</h1>
                <div class = "new-wiki-button">
                    { widget::button("button", "New wiki", self.link.callback(|_| Msg::NewWiki)) }
                </div>
                { widget::table("wiki_list", "Wikis", &[TableCell::text("Name"), TableCell::text("Update")], &wikis) }
            </div>
        }
    }
}
