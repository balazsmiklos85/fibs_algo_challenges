use std::{fmt, collections::VecDeque};

struct Assignment {
    team: usize,
    intern: usize,
    score: usize,
}

impl Assignment {
    fn conflicts(&self, best_assignment: &Assignment) -> bool {
        self.team == best_assignment.team || self.intern == best_assignment.intern
    }

    fn score(&self, team_preferences: &Vec<Vec<usize>>, intern_preferences: &Vec<Vec<usize>>) -> usize {
        let intern_score = intern_preferences[self.intern].iter()
            .position(|preferred_team| preferred_team == &self.team);
        let team_score = team_preferences[self.team].iter()
            .position(|preferred_intern| preferred_intern == &self.intern);
        match (intern_score, team_score) {
            (Some(intern_score), Some(team_score)) => intern_score + team_score,
            _ => usize::MAX,
        }
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.team, self.intern)
    }
}

struct Assigner {
    team_preferences: Vec<Vec<usize>>,
    intern_preferences: Vec<Vec<usize>>,
}

impl Assigner {
    fn solve_with_greedy(&self) {
        let mut possible_assignments = self.generate_possible_assignments_sorted_by_score();
        let mut result = vec![];
        while !possible_assignments.is_empty() {
            let best_assignment = possible_assignments.pop_front().unwrap();
            possible_assignments = possible_assignments
                .into_iter()
                .filter(|assignment| !assignment.conflicts(&best_assignment))
                .collect();
            result.push(best_assignment);
        }
        self.print_result(&result);
    }

    fn generate_possible_assignments_sorted_by_score(&self) -> VecDeque<Assignment> {
        let mut possible_assignments = VecDeque::new();
        for team_index in 0..self.intern_preferences.len() {
            for intern_index in 0..self.team_preferences.len() {
                let mut assignment = Assignment {
                    team: team_index,
                    intern: intern_index,
                    score: usize::MAX,
                };
                assignment.score = assignment.score(&self.team_preferences, &self.intern_preferences);
                possible_assignments.push_back(assignment);
            }
        }
        possible_assignments.make_contiguous()
                            .sort_by(|assignment1, assignment2| {
            assignment1.score.cmp(&assignment2.score)
        });
        possible_assignments
    }

    fn print_result(&self, assignments: &Vec<Assignment>) {
        let joined = assignments.into_iter()
            .map(|assignment| assignment.to_string())
            .collect::<Vec<String>>()
            .join(",\n  ");
        print!("[\n  {}\n]\n", joined)
    }
}

fn main() {
    let interns_input = vec![
        vec![0, 1, 2],
        vec![1, 0, 2],
        vec![1, 2, 0],
    ];
    let teams_input = vec![
        vec![2, 1, 0],
        vec![1, 2, 0],
        vec![0, 2, 1],
    ];

    let assigner = Assigner {
        team_preferences: teams_input,
        intern_preferences: interns_input,
    };
    assigner.solve_with_greedy();
}
