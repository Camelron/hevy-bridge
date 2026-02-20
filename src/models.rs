use serde::{Deserialize, Serialize};

// ──────────────────────────────────────────────
// Sets
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
    #[serde(rename = "type")]
    pub set_type: Option<String>,
    pub weight_kg: Option<f64>,
    pub reps: Option<f64>,
    pub distance_meters: Option<f64>,
    pub duration_seconds: Option<f64>,
    pub rpe: Option<f64>,
    pub custom_metric: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSet {
    #[serde(rename = "type")]
    pub set_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reps: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metric: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpe: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
    #[serde(rename = "type")]
    pub set_type: Option<String>,
    pub weight_kg: Option<f64>,
    pub reps: Option<f64>,
    pub rep_range: Option<RepRange>,
    pub distance_meters: Option<f64>,
    pub duration_seconds: Option<f64>,
    pub rpe: Option<f64>,
    pub custom_metric: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineSet {
    #[serde(rename = "type")]
    pub set_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_kg: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reps: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_meters: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metric: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_range: Option<RepRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepRange {
    pub start: Option<f64>,
    pub end: Option<f64>,
}

// ──────────────────────────────────────────────
// Exercises
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub index: Option<f64>,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub exercise_template_id: Option<String>,
    pub supersets_id: Option<f64>,
    pub sets: Vec<Set>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostExercise {
    pub exercise_template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superset_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub sets: Vec<PostSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineExercise {
    pub index: Option<f64>,
    pub title: Option<String>,
    pub rest_seconds: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub exercise_template_id: Option<String>,
    pub supersets_id: Option<f64>,
    pub sets: Vec<RoutineSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineExercise {
    pub exercise_template_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superset_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub sets: Vec<PostRoutineSet>,
}

// ──────────────────────────────────────────────
// Workouts
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workout {
    pub id: Option<String>,
    pub title: Option<String>,
    pub routine_id: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWorkoutInner {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    pub exercises: Vec<PostExercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWorkoutBody {
    pub workout: PostWorkoutInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutsPage {
    pub page: i64,
    pub page_count: i64,
    pub workouts: Vec<Workout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutCountResponse {
    pub workout_count: i64,
}

// ──────────────────────────────────────────────
// Workout Events
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WorkoutEvent {
    #[serde(rename = "updated")]
    Updated { workout: Workout },
    #[serde(rename = "deleted")]
    Deleted {
        id: String,
        deleted_at: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedWorkoutEvents {
    pub page: i64,
    pub page_count: i64,
    pub events: Vec<WorkoutEvent>,
}

// ──────────────────────────────────────────────
// Routines
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Routine {
    pub id: Option<String>,
    pub title: Option<String>,
    pub folder_id: Option<f64>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
    pub exercises: Vec<RoutineExercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineInner {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub exercises: Vec<PostRoutineExercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineBody {
    pub routine: PostRoutineInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutRoutineInner {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub exercises: Vec<PostRoutineExercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutRoutineBody {
    pub routine: PutRoutineInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutinesPage {
    pub page: i64,
    pub page_count: i64,
    pub routines: Vec<Routine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleRoutineResponse {
    pub routine: Routine,
}

// ──────────────────────────────────────────────
// Exercise Templates
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseTemplate {
    pub id: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub exercise_type: Option<String>,
    pub primary_muscle_group: Option<String>,
    pub secondary_muscle_groups: Option<Vec<String>>,
    pub is_custom: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseTemplatesPage {
    pub page: i64,
    pub page_count: i64,
    pub exercise_templates: Vec<ExerciseTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExerciseInner {
    pub title: String,
    pub exercise_type: String,
    pub equipment_category: String,
    pub muscle_group: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_muscles: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExerciseBody {
    pub exercise: CreateExerciseInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExerciseResponse {
    pub id: Option<serde_json::Value>,
}

// ──────────────────────────────────────────────
// Routine Folders
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineFolder {
    pub id: Option<f64>,
    pub index: Option<f64>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutineFoldersPage {
    pub page: i64,
    pub page_count: i64,
    pub routine_folders: Vec<RoutineFolder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineFolderInner {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRoutineFolderBody {
    pub routine_folder: PostRoutineFolderInner,
}

// ──────────────────────────────────────────────
// Exercise History
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseHistoryEntry {
    pub workout_id: Option<String>,
    pub workout_title: Option<String>,
    pub workout_start_time: Option<String>,
    pub workout_end_time: Option<String>,
    pub exercise_template_id: Option<String>,
    pub weight_kg: Option<f64>,
    pub reps: Option<i64>,
    pub distance_meters: Option<i64>,
    pub duration_seconds: Option<i64>,
    pub rpe: Option<f64>,
    pub custom_metric: Option<f64>,
    pub set_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseHistoryResponse {
    pub exercise_history: Vec<ExerciseHistoryEntry>,
}

// ──────────────────────────────────────────────
// User
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub data: UserInfo,
}
