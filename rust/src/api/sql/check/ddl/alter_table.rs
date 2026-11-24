use super::*;

pub(super) fn check(
    concept: SqlConcept,
    conditions: &mut [(bool, String)],
    diagnostics: &mut Diagnostics,
    expected_table: &Token2,
    expected_action: Option<&SQLAlterAction>,
    actual_table: &Token2,
    actual_action: Option<&SQLAlterAction>,
) {
    check_part(
        conditions,
        diagnostics,
        0,
        token_eq(expected_table, actual_table),
        "The ALTER TABLE target does not match",
        Some(actual_table),
    );
    let (Some(expected), Some(actual)) = (expected_action, actual_action)
    else {
        missing_clause(
            diagnostics,
            conditions,
            1,
            "The ALTER TABLE action is missing",
        );
        return;
    };
    match (concept, expected, actual) {
        (
            SqlConcept::AlterTable,
            SQLAlterAction::AddColumn(en, et),
            SQLAlterAction::AddColumn(an, at),
        ) => {
            check_part(
                conditions,
                diagnostics,
                1,
                true,
                "Add a column",
                Some(an),
            );
            check_part(
                conditions,
                diagnostics,
                2,
                token_eq(en, an),
                "The new column name does not match",
                Some(an),
            );
            check_part(
                conditions,
                diagnostics,
                3,
                token_eq(et, at),
                "The new column type does not match",
                Some(at),
            );
        }
        (
            SqlConcept::AlterTable,
            SQLAlterAction::DropColumn(en),
            SQLAlterAction::DropColumn(an),
        ) => {
            check_part(
                conditions,
                diagnostics,
                1,
                true,
                "Drop a column",
                Some(an),
            );
            check_part(
                conditions,
                diagnostics,
                2,
                token_eq(en, an),
                "The dropped column does not match",
                Some(an),
            );
        }
        (
            SqlConcept::AlterTable,
            SQLAlterAction::RenameTable(en),
            SQLAlterAction::RenameTable(an),
        ) => {
            check_part(
                conditions,
                diagnostics,
                1,
                true,
                "Rename the table",
                Some(an),
            );
            check_part(
                conditions,
                diagnostics,
                2,
                token_eq(en, an),
                "The new table name does not match",
                Some(an),
            );
        }
        (SqlConcept::AlterTable, _, _) => check_part(
            conditions,
            diagnostics,
            1,
            alter_action_eq(expected, actual),
            "The ALTER TABLE action does not match",
            alter_action_token(actual),
        ),
        (
            SqlConcept::AddColumn,
            SQLAlterAction::AddColumn(en, et),
            SQLAlterAction::AddColumn(an, at),
        ) => {
            check_part(
                conditions,
                diagnostics,
                1,
                token_eq(en, an),
                "The new column name does not match",
                Some(an),
            );
            check_part(
                conditions,
                diagnostics,
                2,
                token_eq(et, at),
                "The new column type does not match",
                Some(at),
            );
        }
        (
            SqlConcept::DropColumn,
            SQLAlterAction::DropColumn(en),
            SQLAlterAction::DropColumn(an),
        ) => {
            check_part(
                conditions,
                diagnostics,
                1,
                token_eq(en, an),
                "The dropped column does not match",
                Some(an),
            );
        }
        (
            SqlConcept::RenameTable,
            SQLAlterAction::RenameTable(en),
            SQLAlterAction::RenameTable(an),
        ) => {
            check_part(
                conditions,
                diagnostics,
                1,
                token_eq(en, an),
                "The new table name does not match",
                Some(an),
            );
        }
        _ => missing_clause(
            diagnostics,
            conditions,
            1,
            "The requested ALTER TABLE action is missing",
        ),
    }
}
