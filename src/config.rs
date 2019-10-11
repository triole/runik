use olib_argparse::Argparse;

#[derive(Debug)]
pub struct Config {
    pub input: Option<String>,
    pub count: bool,
    pub threshold: u64,
    pub max_items: u64,
    pub sort_by: SortingOrder,
    pub separator: String,
}

#[derive(Debug)]
pub enum SortingOrder {
    Count,
    Key,
    None,
}

impl Config {
    pub fn init(args: &Argparse) -> Config {
        let mut config = Config {
            input: args.val_op_str("input"),
            count: args.bool("count"),
            threshold: args.val_uint("threshold"),
            max_items: args.val_uint("max_items"),
            sort_by: SortingOrder::None,
            separator: args.val_str("separator"),
        };
        config.parse_sort_by(args.val_str("sort_by"));
        return config;
    }

    fn parse_sort_by(&mut self, s: String) {
        self.sort_by = match s.as_ref() {
            "k" => SortingOrder::Key,
            "c" => SortingOrder::Count,
            _ => SortingOrder::None,
        };
    }
}
