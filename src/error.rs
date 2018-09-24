quick_error! {
    #[derive(Debug)]
    pub enum StocklibError {
        ParseError { from() }
        NetworkError { from() }
    } 
}
