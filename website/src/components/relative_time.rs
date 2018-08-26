use std::time::Duration;
use stdweb::web::Date;
use yew::prelude::*;
use yew::services::{Task, TimeoutService};

pub struct RelativeTime {
    timestamp: u32,
    simple: bool,
    base_timestamp: u32,
    service: TimeoutService,
    task: Option<Box<Task>>,
    link: ComponentLink<RelativeTime>,
}

#[derive(PartialEq, Clone, Default)]
pub struct Props {
    pub timestamp: u32,
    pub simple: bool,
    pub base_timestamp: Option<u32>,
}

#[derive(PartialEq, Clone)]
pub enum Msg {
    Update,
}

impl Component for RelativeTime {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let base_timestamp = props
            .base_timestamp
            .unwrap_or_else(|| (Date::now() / 1000.) as u32);
        let mut relative_time = RelativeTime {
            timestamp: props.timestamp,
            simple: props.simple,
            base_timestamp,
            service: TimeoutService::new(),
            task: None,
            link,
        };
        relative_time.set_timeout();
        relative_time
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update => {
                info!("UPDATING RELATIVE TIME! {}", self.timestamp);
                // self.set_timeout();
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.timestamp = props.timestamp;
        self.simple = props.simple;
        self.base_timestamp = props
            .base_timestamp
            .unwrap_or_else(|| (Date::now() / 1000.) as u32);
        true
    }
}

const SECOND: i32 = 1;
const MINUTE: i32 = SECOND * 60;
const HOUR: i32 = MINUTE * 60;
const DAY: i32 = HOUR * 24;
const WEEK: i32 = DAY * 7;
const MONTH: i32 = WEEK * 4;
const YEAR: i32 = MONTH * 12;

impl RelativeTime {
    fn wait_seconds(&self) -> u32 {
        let now = (Date::now() / 1000.) as i32;
        let timestamp = self.timestamp as i32;
        let diff = (now - timestamp).abs();

        let interval = if diff < MINUTE {
            SECOND
        } else if diff < HOUR {
            MINUTE
        } else if diff < DAY {
            HOUR
        } else if diff < WEEK {
            DAY
        } else if diff < MONTH {
            WEEK
        } else if diff < YEAR {
            MONTH
        } else {
            YEAR
        };

        let next_time = now - (now % interval) + interval;
        let wait_seconds = next_time - now;
        wait_seconds as u32
    }

    fn set_timeout(&mut self) {
        if let Some(mut task) = self.task.take() {
            task.cancel();
        }

        let wait_seconds = self.wait_seconds();
        let wait_duration = Duration::from_secs(wait_seconds as u64);
        let callback = self.link.send_back(|_| Msg::Update);
        let task = self.service.spawn(wait_duration, callback);
        info!("SETTING TIMEOUT: {}, {:#?}", wait_seconds, wait_duration);
        self.task = Some(Box::new(task));
    }
}

fn time_diff_string(timestamp: u32, base_timestamp: u32) -> String {
    let timestamp = timestamp as i32;
    let base_timestamp = base_timestamp as i32;

    let sec = ((base_timestamp - timestamp).abs() as f32).round();
    let min = (sec / 60.).round();
    let hr = (min / 60.).round();
    let day = (hr / 24.).round();
    let week = (day / 7.).round();
    let month = (week / 4.).round();
    let year = (month / 12.).round();
    if month >= 18. {
        format!("{} years", year)
    } else if month >= 12. {
        "a year".to_string()
    } else if day >= 45. {
        format!("{} months", month)
    } else if day >= 30. {
        "a month".to_string()
    } else if hr >= 36. {
        format!("{} days", day)
    } else if hr >= 24. {
        "a day".to_string()
    } else if min >= 90. {
        format!("{} hours", hr)
    } else if min >= 45. {
        "an hour".to_string()
    } else if sec >= 90. {
        format!("{} minutes", min)
    } else if sec >= 45. {
        "a minute".to_string()
    } else if sec >= 2. {
        format!("{} seconds", sec)
    } else {
        "a second".to_string()
    }
}

fn with_suffix(timestamp: u32, base_timestamp: u32) -> String {
    let diff_string = time_diff_string(timestamp, base_timestamp);
    let base_timestamp = base_timestamp as i32;
    let timestamp = timestamp as i32;
    let diff = timestamp - base_timestamp;

    if diff <= 0 {
        format!("{} ago", diff_string)
    } else {
        format!("{} from now", diff_string)
    }
}

impl<T: Component> Renderable<T> for RelativeTime {
    fn view(&self) -> Html<T> {
        let text = if self.simple {
            time_diff_string(self.timestamp, self.base_timestamp)
        } else {
            with_suffix(self.timestamp, self.base_timestamp)
        };
        html! { <time>{ text }</time> }
    }
}
