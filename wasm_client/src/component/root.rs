use gloo::events::EventListener;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::{
    dto::User,
    service::HistoryService,
    utils::{location_path, PageData},
    widget::{self, ButtonStyle, TableCell, TableRow},
};

pub struct Root {
    user_name: String,
    current_page: Page,
    link: ComponentLink<Self>,
    _back_listener: EventListener,
}

enum Page {
    Dashboard,
    NewWiki,
    Wiki(String),
}

pub enum Msg {
    NewWiki,
    AddWiki(String),
    ToWiki(String),
    ToHistoryBack,
}

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let path = location_path();
        if path == "/" {
            HistoryService::change_url("/user");
        };
        let callback = link.callback(|_| Msg::ToHistoryBack);
        let _back_listener = HistoryService::subscribe_to_back(move |_| callback.emit(()));

        Self {
            user_name: "user".to_string(),
            current_page: Page::Dashboard,
            link,
            _back_listener,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewWiki => {
                self.current_page = Page::NewWiki;
                true
            }
            Msg::ToWiki(wiki) => {
                goto_wiki(&wiki);
                self.current_page = Page::Wiki(wiki);
                true
            }
            Msg::ToHistoryBack => {
                let PageData { wiki, .. } = PageData::from_location_path();
                if let Some(wiki) = wiki {
                    self.current_page = Page::Wiki(wiki);
                } else {
                    self.current_page = Page::Dashboard;
                }
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let PageData { path, wiki, .. } = PageData::from_location_path();

        let page = if let Some(wiki) = wiki {
            self.view_wiki(&wiki)
        } else {
            match &self.current_page {
                Page::Dashboard => self.view_dashboard(),
                Page::NewWiki => self.view_new_wiki(),
                Page::Wiki(wiki) => self.view_wiki(&wiki),
            }
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
                                <span class = "mdc-top-app-bar__title">{ "Witeka " } { path }</span>
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

        let table = widget::table(
            "wiki_list",
            "Wikis",
            &[TableCell::text("Name"), TableCell::text("Update")],
            &wikis,
            Some(|row: &TableRow| {
                let wiki: String = row[0].content().to_string();
                self.link.callback(move |_| Msg::ToWiki(wiki.clone()))
            }),
        );

        html! {
            <div class = "content">
                <h1 class = "mdc-typography--headline4">{ "Wikis" }</h1>
                <div class = "new-wiki-button">
                    { widget::button("button", "New wiki", ButtonStyle::Text, self.link.callback(|_| Msg::NewWiki)) }
                </div>
                { table }
            </div>
        }
    }

    fn view_new_wiki(&self) -> Html {
        html! {
            <div class = "content">
                <h1 class = "mdc-typography--headline4">{ "Add wiki" }</h1>
                { widget::text_field("new-wiki-name", "Wiki name") }
                <div class = "right-button">
                    { widget::button("create-wiki", "Create", ButtonStyle::Raised, self.link.callback(|_| Msg::AddWiki("".to_string()))) }
                </div>
            </div>
        }
    }

    fn view_wiki(&self, wiki: &str) -> Html {
        html! {
            <div class = "content">
                <h1 class = "mdc-typography--headline4">{ wiki }</h1>
                <lew::SimpleEditor id = "wiki-editor" />
            </div>
        }
    }
}

fn goto_wiki(wiki: impl AsRef<str>) {
    let wiki = wiki.as_ref();
    let path = location_path();
    let path = path.trim_end_matches(|c| c == '/');
    if !path.ends_with(wiki) {
        HistoryService::change_url(format!("{}/{}", path, wiki));
    } else {
        HistoryService::change_url(path);
    }
}
