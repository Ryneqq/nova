fn main() {
    let args = app_args::get_app_args();
}

mod app_args {
    use clap::{App, Arg, ArgMatches};

    const ARG_CREATE_NAME:      &str = "create";
    const ARG_PATHS_NAME:       &str = "exclude events starting from this time";

    pub fn get_app_args() -> AppArgs {
        let matches = App::new("nova")
            .args(&AppArgs::create_args())
            .get_matches();
        AppArgs::new(matches)
    }

    #[derive(Debug)]
    pub struct AppArgs {
        pub create:     String,
        pub paths:      String,
    }

    impl AppArgs {
        pub fn create_args() -> Vec<Arg<'static, 'static>> {
            vec![
                Arg::with_name(ARG_CREATE_NAME)
                    .long("create")
                    .required(true),
                Arg::with_name(ARG_PATHS_NAME)
                    .short("p")
                    .long("paths")
                    .multiple(true)
                    .takes_value(true)
                    .use_delimiter(false)
                    .required(false),
            ]
        }

        fn new<'a>(matches: ArgMatches<'a>) -> Self {
            let create = matches.is_present(ARG_CREATE_NAME);
            let paths = matches.values_of(ARG_PATHS_NAME)
                .unwrap()
                .map(|s| s.to_string())
                .collect();

            AppArgs { create, paths }
        }
    }
}
