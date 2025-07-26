// declare that an external module exist and it is publicly accessible, 
// without this the definition of the module will not be included for compilation
mod module_a; 

mod helpers {
    // TODO: Make this code compile, either by adding a `use` statement or by using
    //  the appropriate path to refer to the `Ticket` struct.
    use super::Ticket;
    // use crate::Ticket; // using from root directly.
    // use crate::module_a::module_b::Ticket; // using the implicit module_b within the external module_a
    // use crate::module_a::Empty; // using the Empty within the external module_a

    fn create_todo_ticket(title: String, description: String) -> Ticket {
        // With the use statement, private entity can be accessed because the helpers module is a submodule of the crate root, where Ticket is defined. Therefore, create_todo_ticket can access Ticket without any issues even though Ticket is private.
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
        Ticket::new(title, description, "To-Do".into())
    }
}

struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Ticket {
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
