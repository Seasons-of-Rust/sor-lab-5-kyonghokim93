use personnel::AstronautJob;
use personnel::Candidate;
use std::cmp::Reverse;

fn get_job_code(job: &AstronautJob) -> i32 {
    match job {
        AstronautJob::Biogeochemist => 251,
        AstronautJob::Biologist => 257,
        AstronautJob::Engineer => 263,
        AstronautJob::Geologist => 269,
        AstronautJob::Mechanic => 271,
        AstronautJob::Medic => 277,
        AstronautJob::RoverOp => 281,
        AstronautJob::Scientist => 283,
    }
}

fn calculate_job_score(_candidate: &Candidate) -> i32 {
    let job_score = get_job_code(&_candidate.primary_job)
        * match &_candidate.secondary_job {
            None => get_job_code(&_candidate.primary_job),
            Some(v) => get_job_code(v),
        };
    job_score % 576
}

fn calculate_candidate_score(_candidate: &Candidate) -> i32 {
    let candidate_score: i32 =
        (_candidate.health as i32 + calculate_job_score(_candidate)) * _candidate.age as i32;
    candidate_score % 3928
}

fn main() {
    let mut _candidates: Vec<Candidate> = Candidate::load_candidate_file();
    _candidates.sort_by_key(|b| Reverse(calculate_candidate_score(b)));
}

#[test]
fn test_job_score() {
    let a = Candidate {
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: None,
        age: 25,
        health: 72,
    };

    assert_eq!(calculate_job_score(&a), 217);
}
