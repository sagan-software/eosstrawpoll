use components::Link;
use prelude::*;

pub fn view<T: Component>(routes: Vec<(Route, String)>, current: &str) -> Html<T> {
    html! {
        <nav class="breadcrumbs", >
            <span class="breadcrumb_links", >
                { for routes.iter().map(|(route, label)| {
                    html! {
                        <Link:
                            class="breadcrumb",
                            route=route,
                            text=label,
                        />
                    }
                }) }
            </span>
            <span class="breadcrumb_current", >
                { current }
            </span>
        </nav>
    }
}
