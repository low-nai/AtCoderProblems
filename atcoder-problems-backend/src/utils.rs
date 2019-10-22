use crate::sql::models::Submission;
use std::cmp;

pub trait SplitToSegments<T> {
    fn split_into_segments(&self, size: usize) -> Vec<&[T]>;
}

impl<T> SplitToSegments<T> for [T] {
    fn split_into_segments(&self, size: usize) -> Vec<&[T]> {
        let mut result = vec![];
        let mut cur = self;
        while !cur.is_empty() {
            let (left, right) = cur.split_at(cmp::min(size, cur.len()));
            result.push(left);
            cur = right;
        }
        result
    }
}

pub trait GetUserId {
    fn get_user_id(&self) -> &str;
}

impl GetUserId for Submission {
    fn get_user_id(&self) -> &str {
        self.user_id.as_str()
    }
}

pub trait GetProblemId {
    fn get_problem_id(&self) -> &str;
}

impl GetProblemId for Submission {
    fn get_problem_id(&self) -> &str {
        self.problem_id.as_str()
    }
}

pub trait GetEpochSecond {
    fn get_epoch_second(&self) -> i64;
}

impl GetEpochSecond for Submission {
    fn get_epoch_second(&self) -> i64 {
        self.epoch_second
    }
}

pub trait GetPoint {
    fn get_point(&self) -> f64;
}

impl GetPoint for Submission {
    fn get_point(&self) -> f64 {
        self.point
    }
}

pub trait GetContestId {
    fn get_contest_id(&self) -> &str;
}

impl GetContestId for Submission {
    fn get_contest_id(&self) -> &str {
        self.contest_id.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_to_segments() {
        let values = (0..25000usize).collect::<Vec<usize>>();
        let segments = values.split_into_segments(10000);
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].len(), 10000);
        assert_eq!(segments[1].len(), 10000);
        assert_eq!(segments[2].len(), 5000);
    }
}
