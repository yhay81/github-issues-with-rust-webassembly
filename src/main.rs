extern crate github_issues_with_rust_webassembly;
extern crate yew;

use github_issues_with_rust_webassembly::Model;
use yew::prelude::*;
use yew::services::fetch::FetchService;

struct Context {
    web: FetchService,
}

impl AsMut<FetchService> for Context {
    fn as_mut(&mut self) -> &mut FetchService {
        &mut self.web
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        web: FetchService::new(),
    };
    let app: App<Context, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
