// main.rs
use ribir::prelude::*;

fn main() {
  App::run(fn_widget! { @Text { text: "Hello World!" }});
}