This is an example app built using [Leptos] with Bevy ECS as a data layer.

- `cargo make watch` serves the App with watch mode and hot-reload enabled. Run this one if you're getting started!
- `cargo make build` builds the project in release. The output will be in the `dist` directory and the command will not serve it, but quit instead.
- `cargo make fmt` formats with `rustfmt` and `leptosfmt`.
- `cargo make e2e` runs the end-to-end tests from the `end2end` directory using [Playwright].