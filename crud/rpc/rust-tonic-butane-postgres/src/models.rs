#[model]
#[derive(Debug, Default)]
pub struct Artist {
    #[auto]
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Artist {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct NewArtist {
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct UpdatedArtist {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
}
