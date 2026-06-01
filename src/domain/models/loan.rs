// CREATE TABLE Loan (
//     loan_id INTEGER PRIMARY KEY,
//     member_id INTEGER NOT NULL,
//     book_id INTEGER NOT NULL,
//     loan_date TEXT NOT NULL,
//     due_date TEXT NOT NULL,
//     return_date TEXT,
//     FOREIGN KEY (member_id) REFERENCES Member(member_id),
//     FOREIGN KEY (book_id) REFERENCES Book(book_id)
// );

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct Loan {
    pub loan_id: i64,
    pub member_id: i64,
    pub book_id: i64,
    pub loan_date: String,
    pub due_date: String,
    pub return_date: Option<String>,
}

#[derive(Debug, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct LoanWithBook {
    pub loan_id: i64,
    pub member_id: i64,
    pub book_id: i64,
    pub book_name: String,
    pub loan_date: String,
    pub due_date: String,
    pub return_date: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateLoan {
    pub member_id: i64,
    pub book_id: i64,
    pub loan_date: String,
    pub due_date: String,
    pub return_date: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateLoan {
    pub member_id: Option<i64>,
    pub book_id: Option<i64>,
    pub loan_date: Option<String>,
    pub due_date: Option<String>,
    pub return_date: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct LoanQuery {
    pub member_id: Option<i64>,
    pub book_id: Option<i64>,
    pub loan_date: Option<String>,
    pub due_date: Option<String>,
    pub return_date: Option<String>,
}