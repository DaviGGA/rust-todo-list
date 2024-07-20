pub struct Task {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

pub trait  Display {
    fn display(&self) -> ();
}

impl Display for Task {
    fn display(&self) -> () {
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        println!("Completed: {}", if self.completed {"✓"} else {"X"})
    }
}

impl Task {
    pub fn new (title: String, description: String) -> Task {
        Task { title, description, completed: false }
    }

    pub fn complete_task(&mut self) -> () {
        self.completed = true;
    }
}

