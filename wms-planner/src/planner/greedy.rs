use crate::planner::traits::{CostEstimator, TaskPlanner, DistanceCostEstimator};
use crate::types::{Assignment, Task, Worker};
use std::collections::HashSet;

/// Greedy task planner that assigns each task to the nearest available worker
#[derive(Debug)]
pub struct GreedyPlanner<C = DistanceCostEstimator> 
where 
    C: CostEstimator,
{
    cost_estimator: C,
}

impl Default for GreedyPlanner<DistanceCostEstimator> {
    fn default() -> Self {
        Self::new()
    }
}

impl GreedyPlanner<DistanceCostEstimator> {
    /// Create a new greedy planner with default distance-based cost estimation
    pub fn new() -> Self {
        Self {
            cost_estimator: DistanceCostEstimator,
        }
    }
}

impl<C> GreedyPlanner<C> 
where 
    C: CostEstimator,
{
    /// Create a new greedy planner with a custom cost estimator
    pub fn with_cost_estimator(cost_estimator: C) -> Self {
        Self { cost_estimator }
    }
}

impl<C> TaskPlanner for GreedyPlanner<C> 
where 
    C: CostEstimator,
{
    fn plan(&self, tasks: &[Task], workers: &[Worker]) -> Vec<Assignment> {
        let mut assignments = Vec::new();
        let mut assigned_workers = HashSet::new();
        let mut assigned_tasks = HashSet::new();

        // Sort tasks by priority (highest first) to ensure critical tasks get assigned first
        let mut sorted_tasks: Vec<_> = tasks.iter().enumerate().collect();
        sorted_tasks.sort_by(|a, b| {
            b.1.priority.to_numeric().cmp(&a.1.priority.to_numeric())
        });

        for (_, task) in sorted_tasks {
            // Skip if task is already assigned
            if assigned_tasks.contains(&task.id) {
                continue;
            }

            let mut best_assignment: Option<Assignment> = None;
            let mut best_cost = f64::INFINITY;

            // Find the best available worker for this task
            for worker in workers {
                // Skip if worker is already assigned or not available
                if assigned_workers.contains(&worker.id) || !worker.can_accept_task() {
                    continue;
                }

                let cost = self.cost_estimator.estimate(task, worker);
                
                if cost < best_cost {
                    best_cost = cost;
                    best_assignment = Some(Assignment::new(task.id, worker.id, cost));
                }
            }

            // Make the assignment if we found a suitable worker
            if let Some(assignment) = best_assignment {
                assigned_workers.insert(assignment.worker_id);
                assigned_tasks.insert(assignment.task_id);
                assignments.push(assignment);
            }
        }

        assignments
    }
}

/// Greedy planner that supports batch assignments (multiple tasks per worker)
#[derive(Debug)]
pub struct GreedyBatchPlanner<C = DistanceCostEstimator> 
where 
    C: CostEstimator,
{
    cost_estimator: C,
}

impl Default for GreedyBatchPlanner<DistanceCostEstimator> {
    fn default() -> Self {
        Self::new()
    }
}

impl GreedyBatchPlanner<DistanceCostEstimator> {
    pub fn new() -> Self {
        Self {
            cost_estimator: DistanceCostEstimator,
        }
    }
}

impl<C> GreedyBatchPlanner<C> 
where 
    C: CostEstimator,
{
    pub fn with_cost_estimator(cost_estimator: C) -> Self {
        Self { cost_estimator }
    }

    /// Plan assignments allowing multiple tasks per worker
    pub fn plan_batch(&self, tasks: &[Task], workers: &[Worker], max_tasks_per_worker: usize) -> Vec<Assignment> {
        let mut assignments = Vec::new();
        let mut worker_task_counts: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
        let mut assigned_tasks = HashSet::new();

        // Sort tasks by priority (highest first)
        let mut sorted_tasks: Vec<_> = tasks.iter().collect();
        sorted_tasks.sort_by(|a, b| {
            b.priority.to_numeric().cmp(&a.priority.to_numeric())
        });

        for task in sorted_tasks {
            // Skip if task is already assigned
            if assigned_tasks.contains(&task.id) {
                continue;
            }

            let mut best_assignment: Option<Assignment> = None;
            let mut best_cost = f64::INFINITY;

            // Find the best available worker for this task
            for worker in workers {
                if !worker.can_accept_task() {
                    continue;
                }

                let current_task_count = worker_task_counts.get(&worker.id).unwrap_or(&0);
                if *current_task_count >= max_tasks_per_worker {
                    continue;
                }

                let cost = self.cost_estimator.estimate(task, worker);
                
                if cost < best_cost {
                    best_cost = cost;
                    best_assignment = Some(Assignment::new(task.id, worker.id, cost));
                }
            }

            // Make the assignment if we found a suitable worker
            if let Some(assignment) = best_assignment {
                *worker_task_counts.entry(assignment.worker_id).or_insert(0) += 1;
                assigned_tasks.insert(assignment.task_id);
                assignments.push(assignment);
            }
        }

        assignments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Location, Priority};

    #[test]
    fn test_greedy_planner_basic_assignment() {
        let planner = GreedyPlanner::new();
        
        let tasks = vec![
            Task::new(1, Location::new(0.0, 0.0), Priority::High),
            Task::new(2, Location::new(10.0, 10.0), Priority::Medium),
        ];
        
        let workers = vec![
            Worker::new(1, Location::new(1.0, 1.0), true),
            Worker::new(2, Location::new(11.0, 11.0), true),
        ];

        let assignments = planner.plan(&tasks, &workers);
        
        assert_eq!(assignments.len(), 2);
        
        // Task 1 should be assigned to worker 1 (closer)
        let task1_assignment = assignments.iter().find(|a| a.task_id == 1).unwrap();
        assert_eq!(task1_assignment.worker_id, 1);
        
        // Task 2 should be assigned to worker 2 (closer)
        let task2_assignment = assignments.iter().find(|a| a.task_id == 2).unwrap();
        assert_eq!(task2_assignment.worker_id, 2);
    }

    #[test]
    fn test_greedy_planner_priority_ordering() {
        let planner = GreedyPlanner::new();
        
        let tasks = vec![
            Task::new(1, Location::new(0.0, 0.0), Priority::Low),
            Task::new(2, Location::new(0.0, 0.0), Priority::Critical),
        ];
        
        // Only one worker available
        let workers = vec![
            Worker::new(1, Location::new(1.0, 1.0), true),
        ];

        let assignments = planner.plan(&tasks, &workers);
        
        // Only one assignment should be made
        assert_eq!(assignments.len(), 1);
        
        // The critical priority task should be assigned
        assert_eq!(assignments[0].task_id, 2);
        assert_eq!(assignments[0].worker_id, 1);
    }

    #[test]
    fn test_greedy_planner_no_available_workers() {
        let planner = GreedyPlanner::new();
        
        let tasks = vec![
            Task::new(1, Location::new(0.0, 0.0), Priority::High),
        ];
        
        let workers = vec![
            Worker::new(1, Location::new(1.0, 1.0), false), // Not available
        ];

        let assignments = planner.plan(&tasks, &workers);
        
        // No assignments should be made
        assert_eq!(assignments.len(), 0);
    }

    #[test]
    fn test_greedy_batch_planner() {
        let planner = GreedyBatchPlanner::new();
        
        let tasks = vec![
            Task::new(1, Location::new(0.0, 0.0), Priority::High),
            Task::new(2, Location::new(1.0, 1.0), Priority::Medium),
            Task::new(3, Location::new(2.0, 2.0), Priority::Low),
        ];
        
        let workers = vec![
            Worker::new(1, Location::new(0.5, 0.5), true),
        ];

        let assignments = planner.plan_batch(&tasks, &workers, 2);
        
        // Two assignments should be made (max 2 tasks per worker)
        assert_eq!(assignments.len(), 2);
        
        // Both should be assigned to worker 1
        assert!(assignments.iter().all(|a| a.worker_id == 1));
        
        // Higher priority tasks should be assigned first
        assert!(assignments.iter().any(|a| a.task_id == 1)); // High priority
        assert!(assignments.iter().any(|a| a.task_id == 2)); // Medium priority
    }

    #[test]
    fn test_worker_load_affects_cost() {
        let planner = GreedyPlanner::new();
        
        let tasks = vec![
            Task::new(1, Location::new(0.0, 0.0), Priority::Medium),
        ];
        
        let workers = vec![
            Worker::new(1, Location::new(1.0, 1.0), true).with_load(0.0), // No load
            Worker::new(2, Location::new(1.0, 1.0), true).with_load(0.8), // High load
        ];

        let assignments = planner.plan(&tasks, &workers);
        
        assert_eq!(assignments.len(), 1);
        // Should prefer worker with lower load
        assert_eq!(assignments[0].worker_id, 1);
    }
}