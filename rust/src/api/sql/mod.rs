use rand::random_range;

use crate::api::diagnostics::Diagnostics;
use crate::api::generic_operator;
use crate::api::sql::check::sql_check;

pub mod check;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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

    fn create_table(&mut self, table: &str, columns: Vec<&str>) {
        self.push_line(&format!("CREATE TABLE {table} ("));
        self.increase_ident();
        for (index, column) in columns.iter().enumerate() {
            let suffix = if index + 1 == columns.len() { "" } else { "," };
            self.push_line(&format!("{column}{suffix}"));
        }
        self.decrease_ident();
        self.push_line(");");

        self.new_condition(format!("Create a table named {table}"));
        for column in columns {
            self.new_condition(format!("Add a column defined as {column}"));
        }
    }

    fn alter_table(&mut self, table: &str, action: &str) {
        self.push_line(&format!("ALTER TABLE {table} {action};"));

        self.new_condition(format!("Target the {table} table"));
        let parts = action.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            ["ADD", "COLUMN", column, data_type @ ..]
                if !data_type.is_empty() =>
            {
                self.new_condition("Add a column".to_string());
                self.new_condition(format!("Name the new column {column}"));
                self.new_condition(format!(
                    "Use {} as its type",
                    data_type.join(" ")
                ));
            }
            ["ALTER", "COLUMN", column, remainder @ ..]
                if !remainder.is_empty() =>
            {
                self.new_condition(format!("Alter the {column} column"));
                self.new_condition(format!("Apply {}", remainder.join(" ")));
            }
            ["DROP", "COLUMN", column] => {
                self.new_condition("Drop a column".to_string());
                self.new_condition(format!("Remove the {column} column"));
            }
            ["RENAME", "TO", new_name] => {
                self.new_condition("Rename the table".to_string());
                self.new_condition(format!(
                    "Use {new_name} as the new table name"
                ));
            }
            _ => self.new_condition(format!(
                "Apply the ALTER TABLE action: {action}"
            )),
        }
    }

    fn drop_table(&mut self, table: &str) {
        self.push_line(&format!("DROP TABLE {table};"));

        self.new_condition("Use the DROP TABLE statement".to_string());
        self.new_condition(format!("Drop the table named {table}"));
    }

    fn truncate_table(&mut self, table: &str) {
        self.push_line(&format!("TRUNCATE TABLE {table};"));

        self.new_condition(
            "Use TRUNCATE TABLE to keep the table structure".to_string(),
        );
        self.new_condition(format!("Remove all records from {table}"));
    }

    fn add_column(&mut self, table: &str, column: &str, data_type: &str) {
        self.push_line(&format!(
            "ALTER TABLE {table} ADD COLUMN {column} {data_type};"
        ));

        self.new_condition(format!("Target the {table} table"));
        self.new_condition(format!("Add a column named {column}"));
        self.new_condition(format!("Give the column the type {data_type}"));
    }

    fn drop_column(&mut self, table: &str, column: &str) {
        self.push_line(&format!("ALTER TABLE {table} DROP COLUMN {column};"));

        self.new_condition(format!("Target the {table} table"));
        self.new_condition(format!("Remove the {column} column"));
    }

    fn rename_table(&mut self, table: &str, new_name: &str) {
        self.push_line(&format!("ALTER TABLE {table} RENAME TO {new_name};"));

        self.new_condition(format!("Rename the table {table}"));
        self.new_condition(format!("Use {new_name} as the new table name"));
    }

    fn select(&mut self, columns: Vec<&str>, table: &str) {
        self.push_line(&format!("SELECT {}", columns.join(", ")));
        self.push_line(&format!("FROM {table};"));

        for column in columns {
            self.new_condition(format!("Select {column}"));
        }
        self.new_condition(format!("Select them from the {table} table"));
    }

    fn where_clause(&mut self, predicates: Vec<(String, String, String)>) {
        let clauses = predicates
            .iter()
            .map(|(column, operator, value)| {
                format!("{column} {operator} {value}")
            })
            .collect::<Vec<_>>();
        self.push_line(&format!("WHERE {};", clauses.join(" AND ")));
        self.add_predicate_conditions("Filter using", &predicates);
    }

    fn order_by(&mut self, columns: Vec<(String, String)>) {
        let terms = columns
            .iter()
            .map(|(column, direction)| format!("{column} {direction}"))
            .collect::<Vec<_>>();
        self.push_line(&format!("ORDER BY {};", terms.join(", ")));
        for (column, direction) in columns {
            self.new_condition(format!(
                "Sort the results by the {column} column"
            ));
            self.new_condition(format!("Use {direction} order for {column}"));
        }
    }

    fn group_by(&mut self, columns: Vec<&str>) {
        self.push_line(&format!("GROUP BY {};", columns.join(", ")));
        for column in columns {
            self.new_condition(format!("Group the results by {column}"));
        }
    }

    fn having(&mut self, predicates: Vec<(String, String, String)>) {
        let clauses = predicates
            .iter()
            .map(|(expression, operator, value)| {
                format!("{expression} {operator} {value}")
            })
            .collect::<Vec<_>>();
        self.push_line(&format!("HAVING {};", clauses.join(" AND ")));
        self.add_predicate_conditions(
            "Filter the grouped results using",
            &predicates,
        );
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

        self.new_condition(format!("Use the {function}() aggregate function"));
        self.new_condition(format!("Apply it to {column}"));
        self.new_condition(format!("Name the result {alias}"));
        self.new_condition(format!("Read the result from {table}"));
    }

    fn join(
        &mut self,
        table1: &str,
        table2: &str,
        conditions: Vec<(&str, &str, &str)>,
    ) {
        self.push_line(&format!("SELECT * FROM {table1}"));

        self.push_line(&format!("INNER JOIN {table2}"));

        let clauses = conditions
            .iter()
            .map(|(left, operator, right)| format!("{left} {operator} {right}"))
            .collect::<Vec<_>>();
        self.push_line(&format!("ON {};", clauses.join(" AND ")));

        self.new_condition("Select all columns".to_string());
        self.new_condition(format!("Read the rows from {table1}"));
        self.new_condition(format!("Join the {table2} table with INNER JOIN"));
        for (left, operator, right) in conditions {
            self.new_condition(format!("Match rows using {left}"));
            self.new_condition(format!(
                "Use {operator} to compare the join columns"
            ));
            self.new_condition(format!("Compare it with {right}"));
        }
    }

    fn insert(&mut self, table: &str, columns: Vec<&str>, values: Vec<String>) {
        self.push_line(&format!(
            "INSERT INTO {table} ({})",
            columns.join(", ")
        ));

        self.push_line(&format!("VALUES ({});", values.join(", ")));

        self.new_condition(format!("Insert a new record into {table}"));
        for column in &columns {
            self.new_condition(format!(
                "Provide a value for the {column} column"
            ));
        }
        for (column, value) in columns.iter().zip(values.iter()) {
            self.new_condition(format!("Set {column} to {value}"));
        }
    }

    fn update(
        &mut self,
        table: &str,
        assignments: Vec<(String, String)>,
        predicates: Vec<(String, String, String)>,
    ) {
        self.push_line(&format!("UPDATE {table}"));

        let sets = assignments
            .iter()
            .map(|(column, value)| format!("{column} = {value}"))
            .collect::<Vec<_>>();
        self.push_line(&format!("SET {}", sets.join(", ")));
        let clauses = predicates
            .iter()
            .map(|(column, operator, value)| {
                format!("{column} {operator} {value}")
            })
            .collect::<Vec<_>>();
        self.push_line(&format!("WHERE {};", clauses.join(" AND ")));

        self.new_condition(format!("Update the {table} table"));
        for (column, value) in assignments {
            self.new_condition(format!("Set the {column} column"));
            self.new_condition(format!("Assign it the value {value}"));
        }
        self.add_predicate_conditions("Filter rows using", &predicates);
    }

    fn delete(
        &mut self,
        table: &str,
        predicates: Vec<(String, String, String)>,
    ) {
        self.push_line(&format!("DELETE FROM {table}"));

        let clauses = predicates
            .iter()
            .map(|(column, operator, value)| {
                format!("{column} {operator} {value}")
            })
            .collect::<Vec<_>>();
        self.push_line(&format!("WHERE {};", clauses.join(" AND ")));

        self.new_condition(format!("Delete records from {table}"));
        self.add_predicate_conditions("Restrict deletion using", &predicates);
    }

    fn add_predicate_conditions(
        &mut self,
        instruction: &str,
        predicates: &[(String, String, String)],
    ) {
        for (column, operator, value) in predicates {
            self.new_condition(format!("{instruction} the {column} column"));
            self.new_condition(format!(
                "Use {operator} as the comparison operator"
            ));
            self.new_condition(format!("Compare it with {value}"));
        }
    }
}

fn sql_create_table() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::CreateTable);

    drill.create_table("employees", vec![
        "id INT PRIMARY KEY",
        "name VARCHAR(100)",
        "salary DECIMAL(10, 2)",
    ]);

    drill
}

fn sql_alter_table() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::AlterTable);

    let actions = [
        "ADD COLUMN email VARCHAR(255)",
        "ADD COLUMN age INT",
        "DROP COLUMN email",
        "RENAME TO staff",
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

    let columns: [&[&str]; 4] =
        [&["name", "age"], &["name", "grade"], &["*"], &["id", "name"]];
    let column = columns[random_range(0..columns.len())];

    drill.select(column.to_vec(), "students");

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
    drill.new_condition("Select all columns".to_string());
    drill.new_condition("Read the rows from students".to_string());

    drill.where_clause(vec![(column.into(), operator.into(), value)]);

    drill
}

fn sql_order_by() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::OrderBy);

    let column = ["name", "age", "grade"][random_range(0..3)];
    let direction = random_direction();

    drill.push_line("SELECT *");
    drill.push_line("FROM students");
    drill.new_condition("Select all columns".to_string());
    drill.new_condition("Read the rows from students".to_string());
    drill.order_by(vec![(column.into(), direction.into())]);

    drill
}

fn sql_group_by() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::GroupBy);

    drill.push_line("SELECT course_id, COUNT(*)");
    drill.push_line("FROM enrollments");
    drill.new_condition("Select the course_id column");
    drill.new_condition("Count the enrollments using COUNT(*)");
    drill.new_condition("Read the rows from enrollments");
    drill.group_by(vec!["course_id"]);

    drill
}

fn sql_having() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Having);

    let count = random_range(2..10);

    drill.push_line("SELECT course_id, COUNT(*) AS total");
    drill.push_line("FROM enrollments");
    drill.push_line("GROUP BY course_id");
    drill.new_condition("Select the course_id column");
    drill.new_condition("Count the rows using COUNT(*)");
    drill.new_condition("Name the count total");
    drill.new_condition("Read the rows from enrollments");
    drill.new_condition("Group the results by course_id");
    drill.having(vec![("COUNT(*)".into(), ">=".into(), count.to_string())]);

    drill
}

fn sql_join() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Join);

    drill.join("students", "enrollments", vec![(
        "students.id".into(),
        "=".into(),
        "enrollments.student_id".into(),
    )]);

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

    drill.insert("students", vec!["name", "age"], vec![
        format!("'{name}'"),
        age.to_string(),
    ]);

    drill
}

fn sql_update() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Update);

    let age = random_range(18..30);
    let id = random_range(1..20);

    drill.update("students", vec![("age".into(), age.to_string())], vec![(
        "id".into(),
        "=".into(),
        id.to_string(),
    )]);

    drill
}

fn sql_delete() -> SqlDrill {
    let mut drill = SqlDrill::default();
    drill.set_concept(SqlConcept::Delete);

    let id = random_range(1..20);
    drill.delete("students", vec![("id".into(), "=".into(), id.to_string())]);
    drill
}

fn random_direction() -> &'static str {
    ["ASC", "DESC"][random_range(0..2)]
}

fn random_aggregate() -> &'static str {
    ["COUNT", "SUM", "AVG", "MIN", "MAX"][random_range(0..5)]
}
