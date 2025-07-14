# WMS Planner

A flexible task planning module for warehouse management systems (WMS) that efficiently assigns tasks to workers using various planning algorithms.

## Features

- **Modular Architecture**: Extensible design supporting multiple planning strategies
- **Greedy Planning**: Simple and efficient task allocation to nearest available workers
- **Cost Estimation**: Pluggable cost estimators (distance-based, time-based)
- **Batch Planning**: Support for assigning multiple tasks per worker
- **Priority Handling**: Tasks are prioritized and assigned accordingly
- **Worker Load Management**: Takes into account current worker load and availability

## Core Components

### Traits

- **`TaskPlanner`**: Core interface for all planning algorithms
- **`CostEstimator`**: Interface for estimating task assignment costs
- **`BatchPlanner`**: Interface for batch planning operations

### Types

- **`Task`**: Represents a work task with location, priority, and optional duration
- **`Worker`**: Represents a worker/robot with location, availability, and load capacity
- **`Assignment`**: Represents the assignment of a task to a worker with estimated cost
- **`Location`**: 2D coordinate system for spatial calculations
- **`Priority`**: Task priority levels (Low, Medium, High, Critical)

## Usage

### Basic Example

```rust
use wms_planner::{GreedyPlanner, TaskPlanner, Task, Worker, Location, Priority};

// Create a planner
let planner = GreedyPlanner::new();

// Define tasks
let tasks = vec![
    Task::new(1, Location::new(0.0, 0.0), Priority::High),
    Task::new(2, Location::new(10.0, 10.0), Priority::Medium),
];

// Define workers
let workers = vec![
    Worker::new(1, Location::new(1.0, 1.0), true),
    Worker::new(2, Location::new(11.0, 11.0), true),
];

// Plan assignments
let assignments = planner.plan(&tasks, &workers);

for assignment in assignments {
    println!("Task {} assigned to Worker {} (cost: {:.2})", 
             assignment.task_id, assignment.worker_id, assignment.estimated_cost);
}
```

### Using Custom Cost Estimators

```rust
use wms_planner::{GreedyPlanner, TimeCostEstimator};

// Create a planner with time-based cost estimation
let time_estimator = TimeCostEstimator { travel_speed: 2.0 };
let planner = GreedyPlanner::with_cost_estimator(time_estimator);

// Use the planner as before...
```

### Batch Planning

```rust
use wms_planner::GreedyBatchPlanner;

let batch_planner = GreedyBatchPlanner::new();

// Allow up to 3 tasks per worker
let assignments = batch_planner.plan_batch(&tasks, &workers, 3);
```

### Advanced Worker Configuration

```rust
use wms_planner::{Worker, Location};

let worker = Worker::new(1, Location::new(0.0, 0.0), true)
    .with_load(0.3)        // 30% current load
    .with_max_tasks(5);    // Can handle up to 5 tasks

// Workers with higher load will have higher assignment costs
```

### Task Configuration

```rust
use wms_planner::{Task, Location, Priority};

let task = Task::new(1, Location::new(5.0, 5.0), Priority::Critical)
    .with_duration(45.0);  // Estimated 45 minutes to complete

// Tasks with higher priority will be assigned first
// Tasks with longer duration will affect time-based cost estimation
```

## Planning Algorithms

### Greedy Planner

The `GreedyPlanner` implements a simple greedy algorithm that:

1. Sorts tasks by priority (highest first)
2. For each task, finds the worker with the lowest assignment cost
3. Assigns each task to exactly one worker
4. Each worker can only be assigned one task

This algorithm is:
- **Fast**: O(n × m) where n = tasks, m = workers
- **Simple**: Easy to understand and debug
- **Effective**: Works well for many real-world scenarios

### Cost Estimation

#### Distance Cost Estimator (Default)

- Uses Euclidean distance as base cost
- Applies load penalty for busy workers
- Applies priority multiplier (lower cost for higher priority)

#### Time Cost Estimator

- Calculates travel time based on distance and speed
- Includes task execution time
- Factors in worker load and task priority

## Architecture

The crate is organized into modules:

```
wms-planner/
├── src/
│   ├── lib.rs              # Main module exports
│   ├── types.rs            # Core data types
│   └── planner/
│       ├── mod.rs          # Planner module exports
│       ├── traits.rs       # Core traits and interfaces
│       └── greedy.rs       # Greedy algorithm implementations
└── README.md
```

## Extensibility

The modular design makes it easy to add new planning algorithms:

1. Implement the `TaskPlanner` trait for basic planning
2. Implement the `BatchPlanner` trait for batch planning
3. Implement the `CostEstimator` trait for custom cost calculation

Example custom planner:

```rust
use wms_planner::{TaskPlanner, Task, Worker, Assignment};

struct CustomPlanner;

impl TaskPlanner for CustomPlanner {
    fn plan(&self, tasks: &[Task], workers: &[Worker]) -> Vec<Assignment> {
        // Your custom planning logic here
        todo!()
    }
}
```

## Testing

Run the test suite:

```bash
cargo test
```

The crate includes comprehensive tests covering:
- Basic assignment functionality
- Priority ordering
- Worker availability handling
- Cost estimation
- Batch planning
- Edge cases

## Future Enhancements

The architecture is designed to support:

- **Optimization Algorithms**: A*, genetic algorithms, linear programming
- **Multi-objective Optimization**: Balancing cost, time, and worker satisfaction
- **Dynamic Replanning**: Handling task updates and worker status changes
- **Constraint Handling**: Time windows, skill requirements, equipment needs
- **Performance Metrics**: Assignment quality measurement and reporting