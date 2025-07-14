pub mod planner;
pub mod types;

// Re-export commonly used items
pub use planner::traits::TaskPlanner;
pub use planner::greedy::GreedyPlanner;
pub use types::{Task, Worker, Assignment, Location, Priority, TaskId, WorkerId};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_integration_test() {
        let planner = GreedyPlanner::new();
        let tasks = vec![
            Task::new(1, Location::new(0.0, 0.0), Priority::High),
            Task::new(2, Location::new(10.0, 10.0), Priority::Medium),
        ];
        let workers = vec![
            Worker::new(1, Location::new(1.0, 1.0), true),
            Worker::new(2, Location::new(5.0, 5.0), true),
        ];

        let assignments = planner.plan(&tasks, &workers);
        assert_eq!(assignments.len(), 2);
    }
}