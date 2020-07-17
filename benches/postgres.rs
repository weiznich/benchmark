pub struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
    created_at: chrono::NaiveDateTime,
}

impl User {
    fn from_row(row: &postgres::Row) -> Self {
        User {
            id: row.get("id"),
            name: row.get("name"),
            hair_color: row.get("hair_color"),
            created_at: row.get("created_at"),
        }
    }
}

impl crate::Client for postgres::Client {
    type Entity = User;
    type Error = postgres::Error;

    fn create(dsn: &str) -> Result<Self, Self::Error> {
        postgres::Client::connect(dsn, postgres::NoTls)
    }

    fn exec(&mut self, query: &str) -> Result<(), Self::Error> {
        self.execute(query, &[]).map(|_| ())
    }

    fn tear_down(&mut self) -> Result<(), Self::Error> {
        self.execute("DROP TABLE users;", &[]).map(|_| ())
    }

    fn insert_x(&mut self, x: usize) -> Result<(), Self::Error> {
        self.execute(
            "INSERT INTO users (name, hair_color) VALUES ($1, $2)",
            &[&format!("User {}", x), &format!("hair color {}", x)],
        )
        .map(|_| ())
    }

    fn fetch_all(&mut self) -> Result<Vec<Self::Entity>, Self::Error> {
        let results = self
            .query("SELECT id, name, hair_color, created_at FROM users", &[])?
            .iter()
            .map(User::from_row)
            .collect::<Vec<_>>();

        Ok(results)
    }

    fn fetch_first(&mut self) -> Result<Self::Entity, Self::Error> {
        let result = self
            .query("SELECT id, name, hair_color, created_at FROM users", &[])?
            .iter()
            .map(User::from_row)
            .next()
            .unwrap();

        Ok(result)
    }

    fn fetch_last(&mut self) -> Result<Self::Entity, Self::Error> {
        let result = self
            .query("SELECT id, name, hair_color, created_at FROM users", &[])?
            .iter()
            .map(User::from_row)
            .nth(9_999)
            .unwrap();

        Ok(result)
    }

    fn insert(&mut self, n: usize) -> Result<(), Self::Error> {
        if n == 0 {
            return Ok(());
        }

        let mut query = String::from("INSERT INTO users (name, hair_color) VALUES");
        let mut params = Vec::with_capacity(2 * n);
        for x in 0..n {
            query += &format!(
                "{} (${}, ${})",
                if x == 0 { "" } else { "," },
                2 * x + 1,
                2 * x + 2,
            );
            params.push(format!("User {}", x));
            params.push(format!("hair color {}", x));
        }
        let params = params.iter().map(|p| p as _).collect::<Vec<_>>();
        self.execute(&query as &str, &params)?;
        Ok(())
    }
}

crate::bench! {postgres::Client}
