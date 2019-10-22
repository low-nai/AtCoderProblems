extern crate openssl; // Just for musl-compiler
use openssl_probe; // Just for musl-compiler

use atcoder_problems_backend as backend;

use backend::sql::{
    AcceptedCountClient, LanguageCountClient, ProblemInfoUpdater, ProblemsSubmissionUpdater,
    RatedPointSumClient, StreakUpdater, SubmissionClient, SubmissionRequest,
};
use backend::utils::SplitToSegments;

use diesel::{Connection, PgConnection};
use log::{self, info};
use simple_logger;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    openssl_probe::init_ssl_cert_env_vars(); // Just for musl-compiler

    simple_logger::init_with_level(log::Level::Info)?;
    info!("Started!");

    info!("Connecting to SQL ...");
    let url = env::var("SQL_URL")?;
    let conn = PgConnection::establish(&url)?;

    info!("Loading users...");
    let users = conn.get_users()?;
    for segment in users.split_into_segments(7000).into_iter() {
        info!("Loading submissions of {} users ...", segment.len());
        let user_ids = segment.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let request = SubmissionRequest::UsersAccepted {
            user_ids: &user_ids,
        };
        let user_accepted_submissions = conn.get_submissions(request)?;

        info!("Executing update_rated_point_sum...");
        conn.update_rated_point_sum(&user_accepted_submissions)?;

        info!("Executing update_accepted_count...");
        conn.update_accepted_count(&user_accepted_submissions)?;

        info!("Executing update_language_count...");
        conn.update_language_count(&user_accepted_submissions)?;

        info!("Executing update_streak_count...");
        conn.update_streak_count(&user_accepted_submissions)?;
    }

    info!("Executing update_problem_solver_count...");
    conn.update_solver_count()?;

    info!("Executing update_problem_points...");
    conn.update_problem_points()?;

    info!("Executing update_submissions_of_problems...");
    conn.update_submissions_of_problems()?;

    info!("Finished");

    Ok(())
}
