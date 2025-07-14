use serde::{Deserialize, Serialize};

pub type TaskId = u32;
pub type WorkerId = u32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub x: f64,
    pub y: f64,
}

impl Location {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculate Euclidean distance to another location
    pub fn distance_to(&self, other: &Location) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    /// Convert priority to numeric value for comparison
    pub fn to_numeric(&self) -> u8 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Critical => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub location: Location,
    pub priority: Priority,
    pub estimated_duration: Option<f64>, // in minutes
}

impl Task {
    pub fn new(id: TaskId, location: Location, priority: Priority) -> Self {
        Self {
            id,
            location,
            priority,
            estimated_duration: None,
        }
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.estimated_duration = Some(duration);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Worker {
    pub id: WorkerId,
    pub location: Location,
    pub is_available: bool,
    pub current_load: f64, // 0.0 to 1.0, where 1.0 is fully loaded
    pub max_tasks: usize,  // Maximum number of tasks this worker can handle
}

impl Worker {
    pub fn new(id: WorkerId, location: Location, is_available: bool) -> Self {
        Self {
            id,
            location,
            is_available,
            current_load: 0.0,
            max_tasks: 1,
        }
    }

    pub fn with_load(mut self, load: f64) -> Self {
        self.current_load = load.clamp(0.0, 1.0);
        self
    }

    pub fn with_max_tasks(mut self, max_tasks: usize) -> Self {
        self.max_tasks = max_tasks;
        self
    }

    pub fn can_accept_task(&self) -> bool {
        self.is_available && self.current_load < 1.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub task_id: TaskId,
    pub worker_id: WorkerId,
    pub estimated_cost: f64,
}

impl Assignment {
    pub fn new(task_id: TaskId, worker_id: WorkerId, estimated_cost: f64) -> Self {
        Self {
            task_id,
            worker_id,
            estimated_cost,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_distance() {
        let loc1 = Location::new(0.0, 0.0);
        let loc2 = Location::new(3.0, 4.0);
        assert_eq!(loc1.distance_to(&loc2), 5.0);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::High.to_numeric() > Priority::Medium.to_numeric());
        assert!(Priority::Critical.to_numeric() > Priority::High.to_numeric());
    }

    #[test]
    fn test_worker_availability() {
        let worker = Worker::new(1, Location::new(0.0, 0.0), true);
        assert!(worker.can_accept_task());

        let busy_worker = Worker::new(2, Location::new(0.0, 0.0), true).with_load(1.0);
        assert!(!busy_worker.can_accept_task());

        let unavailable_worker = Worker::new(3, Location::new(0.0, 0.0), false);
        assert!(!unavailable_worker.can_accept_task());
    }
}