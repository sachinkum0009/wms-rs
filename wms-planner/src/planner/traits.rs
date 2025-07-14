use crate::types::{Assignment, Task, Worker};

/// Core trait for task planning algorithms
pub trait TaskPlanner {
    /// Plan task assignments given a set of tasks and workers
    /// 
    /// # Arguments
    /// * `tasks` - List of tasks to be assigned
    /// * `workers` - List of available workers
    /// 
    /// # Returns
    /// Vector of assignments mapping tasks to workers
    fn plan(&self, tasks: &[Task], workers: &[Worker]) -> Vec<Assignment>;
}

/// Trait for estimating the cost of assigning a task to a worker
pub trait CostEstimator {
    /// Estimate the cost of assigning a specific task to a specific worker
    /// 
    /// # Arguments
    /// * `task` - The task to be assigned
    /// * `worker` - The worker who would handle the task
    /// 
    /// # Returns
    /// Estimated cost (lower is better)
    fn estimate(&self, task: &Task, worker: &Worker) -> f64;
}

/// Trait for batch planning (multiple tasks per worker)
pub trait BatchPlanner {
    /// Plan assignments where workers can handle multiple tasks
    /// 
    /// # Arguments
    /// * `tasks` - List of tasks to be assigned
    /// * `workers` - List of available workers
    /// * `max_tasks_per_worker` - Maximum number of tasks per worker
    /// 
    /// # Returns
    /// Vector of assignments potentially with multiple tasks per worker
    fn plan_batch(
        &self,
        tasks: &[Task],
        workers: &[Worker],
        max_tasks_per_worker: usize,
    ) -> Vec<Assignment>;
}

/// Basic distance-based cost estimator
#[derive(Debug, Default)]
pub struct DistanceCostEstimator;

impl CostEstimator for DistanceCostEstimator {
    fn estimate(&self, task: &Task, worker: &Worker) -> f64 {
        // Simple Euclidean distance as base cost
        let distance = worker.location.distance_to(&task.location);
        
        // Factor in worker load (higher load = higher cost)
        let load_penalty = worker.current_load * 10.0;
        
        // Factor in task priority (higher priority = lower cost multiplier)
        let priority_multiplier = match task.priority {
            crate::types::Priority::Critical => 0.5,
            crate::types::Priority::High => 0.7,
            crate::types::Priority::Medium => 1.0,
            crate::types::Priority::Low => 1.5,
        };
        
        (distance + load_penalty) * priority_multiplier
    }
}

/// Time-based cost estimator that includes travel time and task duration
#[derive(Debug)]
pub struct TimeCostEstimator {
    pub travel_speed: f64, // units per minute
}

impl Default for TimeCostEstimator {
    fn default() -> Self {
        Self {
            travel_speed: 1.0, // 1 unit per minute
        }
    }
}

impl CostEstimator for TimeCostEstimator {
    fn estimate(&self, task: &Task, worker: &Worker) -> f64 {
        // Travel time based on distance and speed
        let distance = worker.location.distance_to(&task.location);
        let travel_time = distance / self.travel_speed;
        
        // Task execution time
        let execution_time = task.estimated_duration.unwrap_or(30.0); // default 30 minutes
        
        // Total time cost
        let total_time = travel_time + execution_time;
        
        // Factor in worker load and task priority similar to distance estimator
        let load_penalty = worker.current_load * total_time * 0.5;
        let priority_multiplier = match task.priority {
            crate::types::Priority::Critical => 0.5,
            crate::types::Priority::High => 0.7,
            crate::types::Priority::Medium => 1.0,
            crate::types::Priority::Low => 1.5,
        };
        
        (total_time + load_penalty) * priority_multiplier
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Location, Priority};

    #[test]
    fn test_distance_cost_estimator() {
        let estimator = DistanceCostEstimator;
        let task = Task::new(1, Location::new(0.0, 0.0), Priority::High);
        let worker = Worker::new(1, Location::new(3.0, 4.0), true);
        
        let cost = estimator.estimate(&task, &worker);
        // Should be distance (5.0) * priority multiplier (0.7) = 3.5
        assert!((cost - 3.5).abs() < 0.01);
    }

    #[test]
    fn test_time_cost_estimator() {
        let estimator = TimeCostEstimator::default();
        let task = Task::new(1, Location::new(0.0, 0.0), Priority::Medium)
            .with_duration(20.0);
        let worker = Worker::new(1, Location::new(3.0, 4.0), true);
        
        let cost = estimator.estimate(&task, &worker);
        // Travel time: 5.0 / 1.0 = 5.0
        // Execution time: 20.0
        // Total: 25.0 * priority multiplier (1.0) = 25.0
        assert!((cost - 25.0).abs() < 0.01);
    }
}