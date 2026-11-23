use rand::random_range;

use crate::api::diagnostics::Diagnostics;
use crate::api::generic_operator;
use crate::api::sql::check::sql_check;

pub mod check;

#[derive(Default, Debug, Clone, Copy)]
pub enum SqlConcept {
    #[default]
    Select = 0,
    Where = 1,
    OrderBy = 2,
    GroupBy = 3,
    Having = 4,
    Join = 5,
    Aggregate = 6,
    Insert = 7,
    Update = 8,
    Delete = 9,
    CreateTable = 10,
    AlterTable = 11,
    DropTable = 12,
    TruncateTable = 13,
    AddColumn = 14,
    DropColumn = 15,
    RenameTable = 16,
}

impl SqlConcept {
    #[flutter_rust_bridge::frb(sync)]
    pub fn to_string(&self) -> String {
        match self {
            Self::Select => "Select Statements",
            Self::Where => "Where Clauses",
            Self::OrderBy => "Order By Clauses",
            Self::GroupBy => "Group By Clauses",
            Self::Having => "Having Clauses",
            Self::Join => "Join Statements",
            Self::Aggregate => "Aggregate Functions",
            Self::Insert => "Insert Statements",
            Self::Update => "Update Statements",
            Self::Delete => "Delete Statements",
            Self::CreateTable => "Create Table",
            Self::AlterTable => "Alter Table",
            Self::DropTable => "Drop Table",
            Self::TruncateTable => "Truncate Table",
            Self::AddColumn => "Add Column",
            Self::DropColumn => "Drop Column",
            Self::RenameTable => "Rename Table",
        }
        .into()
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn from_string(s: String) -> Self {
        match s.as_str() {
            "Select Statements" => Self::Select,
            "Where Clauses" => Self::Where,
            "Order By Clauses" => Self::OrderBy,
            "Group By Clauses" => Self::GroupBy,
            "Having Clauses" => Self::Having,
            "Join Statements" => Self::Join,
            "Aggregate Functions" => Self::Aggregate,
            "Insert Statements" => Self::Insert,
            "Update Statements" => Self::Update,
            "Delete Statements" => Self::Delete,
            "Create Table" => Self::CreateTable,
            "Alter Table" => Self::AlterTable,
            "Drop Table" => Self::DropTable,
            "Truncate Table" => Self::TruncateTable,
            "Add Column" => Self::AddColumn,
            "Drop Column" => Self::DropColumn,
            "Rename Table" => Self::RenameTable,
            _ => Self::Select,
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn make_drill(&self) -> SqlDrill {
        match self {
            Self::Select => sql_select(),
            Self::Where => sql_where(),
            Self::OrderBy => sql_order_by(),
            Self::GroupBy => sql_group_by(),
            Self::Having => sql_having(),
            Self::Join => sql_join(),
            Self::Aggregate => sql_aggregate(),
            Self::Insert => sql_insert(),
            Self::Update => sql_update(),
            Self::Delete => sql_delete(),
            Self::CreateTable => sql_create_table(),
            Self::AlterTable => sql_alter_table(),
            Self::DropTable => sql_drop_table(),
            Self::TruncateTable => sql_truncate_table(),
            Self::AddColumn => sql_add_column(),
            Self::DropColumn => sql_drop_column(),
            Self::RenameTable => sql_rename_table(),
        }
    }
}

#[derive(Default, Debug, Clone)]
#[flutter_rust_bridge::frb(opaque)]
pub struct SqlDrill {
    pub concept: SqlConcept,
    pub conditions: Vec<(bool, String)>,
    pub answer: String,
    pub ident_level: usize,
}

impl SqlDrill {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new(
        concept: String,
        conditions: Vec<(bool, String)>,
        answer: String,
    ) -> Self {
        Self {
            concept: SqlConcept::from_string(concept),
            conditions,
            answer,
            ident_level: 0,
        }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn check(&mut self, user_code: String) -> Diagnostics {
        sql_check(self.concept, &mut self.conditions, &self.answer, &user_code)
    }

    #[inline]
    fn set_concept(&mut self, concept: SqlConcept) {
        self.concept = concept;
    }

    #[inline]
    fn new_condition(&mut self, condition: impl Into<String>) {
        self.conditions.push((false, condition.into()));
    }

    #[inline]
    fn indent(&mut self) {
        for _ in 0..self.ident_level {
            self.answer.push_str("    ");
        }
    }

    #[inline]
    fn increase_ident(&mut self) {
        self.ident_level += 1;
    }

    #[inline]
    fn decrease_ident(&mut self) {
        self.ident_level = self.ident_level.saturating_sub(1);
    }

    fn push_line(&mut self, line: &str) {
        self.indent();
        self.answer.push_str(line);
        self.answer.push('\n');
    }

    fn create_table(&mut self, table: &str, columns: &str) {
        self.push_line(&format!("CREATE TABLE {table} ("));
        self.increase_ident();
        self.push_line(columns);
        self.decrease_ident();
        self.push_line(");");

        self.new_condition(format!(
            "Create a table named {table} with the specified columns"
        ));
    }

    fn alter_table(&mut self, table: &str, action: &str) {
        self.push_line(&format!("ALTER TABLE {table} {action};"));

        self.new_condition(format!(
            "Alter the {table} table by performing: {action}"
        ));
    }

    fn drop_table(&mut self, table: &str) {
        self.push_line(&format!("DROP TABLE {table};"));

        self.new_condition(format!("Drop the table named {table}"));
    }

    fn truncate_table(&mut self, table: &str) {
        self.push_line(&format!("TRUNCATE TABLE {table};"));

        self.new_condition(format!(
            "Remove all records from {table} without dropping the table"
        ));
    }

    fn add_column(&mut self, table: &str, column: &str, data_type: &str) {
        self.push_line(&format!(
            "ALTER TABLE {table} ADD COLUMN {column} {data_type};"
        ));

        self.new_condition(format!(
            "Add a column named {column} of type {data_type} to {table}"
        ));
    }

    fn drop_column(&mut self, table: &str, column: &str) {
        self.push_line(&format!("ALTER TABLE {table} DROP COLUMN {column};"));

        self.new_condition(format!("Remove the {column} column from {table}"));
    }

    fn rename_table(&mut self, table: &str, new_name: &str) {
        self.push_line(&format!("ALTER TABLE {table} RENAME TO {new_name};"));

        self.new_condition(format!("Rename the table {table} to {new_name}"));
    }

    fn select(&mut self, columns: &str, table: &str) {
        self.push_line(&format!("SELECT {columns}"));
        self.push_line(&format!("FROM {table};"));

        self.new_condition(format!(
            "Select the columns {columns} from the {table} table"
        ));
    }

    fn where_clause(&mut self, column: &str, operator: &str, value: &str) {
        self.push_line(&format!("WHERE {column} {operator} {value};"));

        self.new_condition(format!(
            "Filter the results where {column} {operator} {value}"
        ));
    }

    fn order_by(&mut self, column: &str, direction: &str) {
        self.push_line(&format!("ORDER BY {column} {direction};"));

        self.new_condition(format!(
            "Sort the results by {column} in {direction} order"
        ));
    }

    fn group_by(&mut self, column: &str) {
        self.push_line(&format!("GROUP BY {column};"));

        self.new_condition(format!("Group the results by {column}"));
    }

    fn having(&mut self, expression: &str, operator: &str, value: &str) {
        self.push_line(&format!("HAVING {expression} {operator} {value};"));

        self.new_condition(format!(
            "Filter grouped results where {expression} {operator} {value}"
        ));
    }

    fn aggregate(
        &mut self,
        function: &str,
        column: &str,
        alias: &str,
        table: &str,
    ) {
        self.push_line(&format!("SELECT {function}({column}) AS {alias}"));
        self.push_line(&format!("FROM {table};"));

        self.new_condition(format!(
            "Use {function}() on {column} and name the result {alias}"
        ));
    }

    fn join(
        &mut self,
        table1: &str,
        table2: &str,
        column1: &str,
        column2: &str,
    ) {
        self.push_line(&format!("SELECT * FROM {table1}"));

        self.push_line(&format!("INNER JOIN {table2}"));

        self.push_line(&format!("ON {column1} = {column2};"));

        self.new_condition(format!(
            "Join {table1} with {table2} using {column1} = {column2}"
        ));
    }

    fn insert(&mut self, table: &str, columns: &str, values: &str) {
        self.push_line(&format!("INSERT INTO {table} ({columns})"));

        self.push_line(&format!("VALUES ({values});"));

        self.new_condition(format!(
            "Insert a new record into {table} with columns ({columns}) and values ({values})"
        ));
    }

    fn update(
        &mut self,
        table: &str,
        column: &str,
        value: &str,
        condition: &str,
    ) {
        self.push_line(&format!("UPDATE {table}"));

        self.push_line(&format!("SET {column} = {value}"));

        self.push_line(&format!("WHERE {condition};"));

        self.new_condition(format!(
            "Update {table} and set {column} to {value} where {condition}"
        ));
    }

    fn delete(&mut self, table: &str, condition: &str) {
        self.push_line(&format!("DELETE FROM {table}"));

        self.push_line(&format!("WHERE {condition};"));

        self.new_condition(format!(
            "Delete records from {table} where {condition}"
        ));
    }
}

fn sql_create_table() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::CreateTable);

    drill.create_table(
        "employees",
        "id INT PRIMARY KEY,\nname VARCHAR(100),\nsalary DECIMAL(10, 2)",
    );

    drill
}

fn sql_alter_table() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::AlterTable);

    let actions = [
        "ADD COLUMN email VARCHAR(255)",
        "ADD COLUMN age INT",
        "ALTER COLUMN salary SET DEFAULT 0",
    ];

    let action = actions[random_range(0..actions.len())];

    drill.alter_table("employees", action);

    drill
}

fn sql_drop_table() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::DropTable);

    drill.drop_table("temporary_employees");

    drill
}

fn sql_truncate_table() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::TruncateTable);

    drill.truncate_table("logs");

    drill
}

fn sql_select() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Select);

    let columns = ["name, age", "name, grade", "*", "id, name"];
    let column = columns[random_range(0..columns.len())];

    drill.select(column, "students");

    drill
}

fn sql_add_column() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::AddColumn);

    let columns = [
        ("email", "VARCHAR(255)"),
        ("age", "INT"),
        ("is_active", "BOOLEAN"),
        ("salary", "DECIMAL(10, 2)"),
    ];

    let (column, data_type) = columns[random_range(0..columns.len())];

    drill.add_column("employees", column, data_type);

    drill
}

fn sql_drop_column() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::DropColumn);

    let columns = ["email", "age", "salary", "is_active"];
    let column = columns[random_range(0..columns.len())];

    drill.drop_column("employees", column);

    drill
}

fn sql_rename_table() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::RenameTable);
    drill.rename_table("old_employees", "employees");
    drill
}

fn sql_where() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Where);

    let column = ["age", "grade", "id"][random_range(0..3)];
    let operator = generic_operator();
    let value = random_range(1..100).to_string();

    drill.push_line("SELECT *");
    drill.push_line("FROM students;");

    // Move the semicolon to the end of the WHERE clause.
    drill.answer = drill.answer.replace("students;", "students");

    drill.where_clause(column, operator, &value);

    drill
}

fn sql_order_by() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::OrderBy);

    let column = ["name", "age", "grade"][random_range(0..3)];
    let direction = random_direction();

    drill.push_line("SELECT *");
    drill.push_line("FROM students");
    drill.order_by(column, direction);

    drill
}

fn sql_group_by() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::GroupBy);

    drill.push_line("SELECT course_id, COUNT(*)");
    drill.push_line("FROM enrollments");
    drill.group_by("course_id");

    drill.new_condition("Count the enrollments for each course using GROUP BY");

    drill
}

fn sql_having() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Having);

    let count = random_range(2..10);

    drill.push_line("SELECT course_id, COUNT(*) AS total");
    drill.push_line("FROM enrollments");
    drill.push_line("GROUP BY course_id");
    drill.having("COUNT(*)", ">=", &count.to_string());

    drill
}

fn sql_join() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Join);

    drill.join(
        "students",
        "enrollments",
        "students.id",
        "enrollments.student_id",
    );

    drill
}

fn sql_aggregate() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Aggregate);

    let function = random_aggregate();

    let (column, alias, table) = match function {
        "COUNT" => ("*", "total_students", "students"),
        "SUM" => ("grade", "total_grade", "students"),
        "AVG" => ("grade", "average_grade", "students"),
        "MIN" => ("grade", "lowest_grade", "students"),
        _ => ("grade", "highest_grade", "students"),
    };

    drill.aggregate(function, column, alias, table);

    drill
}

fn sql_insert() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Insert);

    let name = ["Alice", "Bob", "Charlie"][random_range(0..3)];
    let age = random_range(18..30);

    drill.insert("students", "name, age", &format!("'{name}', {age}"));

    drill
}

fn sql_update() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Update);

    let age = random_range(18..30);
    let id = random_range(1..20);

    drill.update("students", "age", &age.to_string(), &format!("id = {id}"));

    drill
}

fn sql_delete() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Delete);

    let id = random_range(1..20);
    drill.delete("students", &format!("id = {id}"));
    drill
}

fn random_direction() -> &'static str {
    ["ASC", "DESC"][random_range(0..2)]
}

fn random_aggregate() -> &'static str {
    ["COUNT", "SUM", "AVG", "MIN", "MAX"][random_range(0..5)]
}
