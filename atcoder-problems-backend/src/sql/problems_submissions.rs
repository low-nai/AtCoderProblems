use diesel;
use diesel::connection::SimpleConnection;
use diesel::{PgConnection, QueryResult};
use log::info;

fn get_query(table: &str, column: &str) -> String {
    format!(
        r"
                INSERT INTO {table}
                (submission_id, problem_id, contest_id)
                SELECT id, problem_id, contest_id FROM submissions
                WHERE id IN
                (
                    SELECT MIN(id) FROM submissions
                    WHERE result='AC'
                    AND (problem_id, {column}) IN
                    (
                        SELECT problem_id, MIN({column})
                        FROM submissions 
                        WHERE result='AC'
                        GROUP BY problem_id
                    )
                    GROUP BY problem_id
                )
                ON CONFLICT (problem_id)
                DO UPDATE SET
                            contest_id=EXCLUDED.contest_id,
                            problem_id=EXCLUDED.problem_id,
                            submission_id=EXCLUDED.submission_id;",
        table = table,
        column = column
    )
}

pub trait ProblemsSubmissionUpdater {
    fn update_submissions_of_problems(&self) -> QueryResult<()>;
}

impl ProblemsSubmissionUpdater for PgConnection {
    fn update_submissions_of_problems(&self) -> QueryResult<()> {
        info!("Updating shortest...");
        self.batch_execute(&get_query("shortest", "length"))?;

        info!("Updating fastest...");
        self.batch_execute(&get_query("fastest", "execution_time"))?;

        info!("Updating first...");
        self.batch_execute(&get_query("first", "id"))
    }
}
