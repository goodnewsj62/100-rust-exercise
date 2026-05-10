// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let v = ticket.clone();
    (ticket, v.summary())
}

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}

impl Clone for Ticket {
    fn clone(&self) -> Self {
        Ticket {
            title: String::from(&self.title),
            description: (&self.description).into(),
            status: (&self.status).into(),
        }
    }
}
