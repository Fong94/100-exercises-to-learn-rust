pub(crate) mod module_b {
    pub(crate) struct Ticket {
        title: String,
        description: String,
        status: String,
    }
    
    impl Ticket {
        pub(crate) fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }
    
            Ticket {
                title,
                description,
                status,
            }
        }
    }

}

pub struct Empty {}
