mod client;
mod models;

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use client::HevyClient;
use models::*;

// ─────────────────────────────────────────────────────
// Config helpers
// ─────────────────────────────────────────────────────

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("hevy-bridge")
        .join("config.json")
}

fn read_stored_api_key() -> Option<String> {
    let path = config_path();
    let data = std::fs::read_to_string(&path).ok()?;
    let v: serde_json::Value = serde_json::from_str(&data).ok()?;
    v.get("api_key")?.as_str().map(|s| s.to_string())
}

fn store_api_key(key: &str) -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .context("Failed to create config directory")?;
    }
    let data = serde_json::json!({ "api_key": key });
    std::fs::write(&path, serde_json::to_string_pretty(&data)?)
        .context("Failed to write config file")?;
    Ok(())
}

/// Resolve the API key from (in priority order):
///   1. --api-key flag
///   2. HEVY_API_KEY environment variable
///   3. Stored config file (~/.config/hevy-bridge/config.json)
fn resolve_api_key(cli_key: &Option<String>) -> Result<String> {
    if let Some(k) = cli_key {
        return Ok(k.clone());
    }
    if let Ok(k) = std::env::var("HEVY_API_KEY") {
        if !k.is_empty() {
            return Ok(k);
        }
    }
    if let Some(k) = read_stored_api_key() {
        return Ok(k);
    }
    anyhow::bail!(
        "No API key provided. Supply one via:\n  \
         1. --api-key <KEY>\n  \
         2. HEVY_API_KEY environment variable\n  \
         3. `hevy-bridge config set-key <KEY>` to persist it"
    )
}

// ─────────────────────────────────────────────────────
// CLI definition
// ─────────────────────────────────────────────────────

/// hevy-bridge — A CLI client for the Hevy workout tracking API.
///
/// Interact with your Hevy account from the command line: list workouts,
/// create routines, browse exercise templates, and more.
///
/// AUTHENTICATION:
///   All API commands require a Hevy API key (Hevy Pro required).
///   Obtain yours at https://hevy.com/settings?developer
///
///   Provide the key via one of (checked in this order):
///     1. --api-key <KEY>           (per-invocation flag)
///     2. HEVY_API_KEY env var      (session / CI)
///     3. `hevy-bridge config set-key <KEY>`  (persisted to disk)
///
/// OUTPUT:
///   All data commands output JSON to stdout so you can pipe them into
///   jq, scripts, or other tools.
///
/// EXAMPLES:
///   hevy-bridge config set-key YOUR_API_KEY
///   hevy-bridge user info
///   hevy-bridge workouts list --page 1 --page-size 5
///   hevy-bridge workouts get <WORKOUT_ID>
///   hevy-bridge workouts count
///   hevy-bridge exercises list --page-size 100
///   hevy-bridge routines list
///   hevy-bridge workouts create --json '{"workout":{...}}'
#[derive(Parser, Debug)]
#[command(
    name = "hevy-bridge",
    version,
    about = "CLI client for the Hevy workout tracking API (https://api.hevyapp.com/docs)",
    long_about = None,
    after_help = "\
AUTHENTICATION:
  All API commands require a Hevy API key (Hevy Pro).
  Get yours at: https://hevy.com/settings?developer

  The key is resolved in this order:
    1. --api-key <KEY>
    2. HEVY_API_KEY environment variable
    3. Persisted config via `hevy-bridge config set-key <KEY>`

OUTPUT:
  All data commands print JSON to stdout for easy piping to jq or scripts.

TIPS FOR AI AGENTS:
  • Use `hevy-bridge exercises list --page-size 100` to discover exercise
    template IDs needed when creating workouts or routines.
  • POST/PUT bodies are passed as raw JSON via --json flag.
  • Pagination: use --page and --page-size on list commands. Check the
    returned `page_count` to know when to stop.
  • Dates use ISO 8601 format: 2024-01-15T00:00:00Z"
)]
struct Cli {
    /// Hevy API key (overrides env var and stored config).
    #[arg(long, global = true, env = "HEVY_API_KEY", hide_env = true)]
    api_key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage API key configuration.
    ///
    /// Persist your API key to disk so you don't have to pass it every time.
    /// The key is stored at ~/.config/hevy-bridge/config.json
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Retrieve authenticated user information.
    ///
    /// Returns the user ID, display name, and public profile URL for the
    /// account that owns the API key.
    #[command(subcommand)]
    User(UserCommands),

    /// List, view, create, update workouts and query workout events.
    ///
    /// Workouts are the core data type in Hevy. Each workout contains a
    /// title, timestamps, and a list of exercises with their sets.
    ///
    /// Set types: "normal", "warmup", "failure", "dropset"
    /// Weights are always in kilograms (weight_kg).
    #[command(subcommand)]
    Workouts(WorkoutCommands),

    /// List, view, create, update routines.
    ///
    /// Routines are workout templates. They contain exercises with
    /// target sets including optional rep ranges.
    #[command(subcommand)]
    Routines(RoutineCommands),

    /// List, view, and create exercise templates.
    ///
    /// Exercise templates define the exercises available in your account
    /// (both built-in and custom). You need exercise_template_id values
    /// when creating workouts or routines.
    ///
    /// Exercise types: weight_reps, reps_only, bodyweight_reps,
    ///   bodyweight_assisted_reps, duration, weight_duration,
    ///   distance_duration, short_distance_weight
    ///
    /// Muscle groups: abdominals, shoulders, biceps, triceps, forearms,
    ///   quadriceps, hamstrings, calves, glutes, abductors, adductors,
    ///   lats, upper_back, traps, lower_back, chest, cardio, neck,
    ///   full_body, other
    ///
    /// Equipment: none, barbell, dumbbell, kettlebell, machine, plate,
    ///   resistance_band, suspension, other
    #[command(subcommand)]
    Exercises(ExerciseCommands),

    /// List, view, and create routine folders.
    ///
    /// Routine folders organize your routines. New folders are created
    /// at index 0, shifting existing folder indexes up by 1.
    #[command(subcommand)]
    Folders(FolderCommands),

    /// View exercise history (set-level data across workouts).
    ///
    /// Returns every set ever logged for a given exercise template,
    /// including the parent workout context. Useful for tracking
    /// progression over time.
    #[command(subcommand)]
    History(HistoryCommands),

    /// Process a webhook workout payload and print a summary table.
    ///
    /// Accepts the JSON payload from a Hevy webhook (e.g. from a
    /// workout.completed event), fetches the full workout, and prints
    /// a human-readable table summarizing each exercise.
    ///
    /// Columns: Exercise, Sets, Best Weight (lbs), Reps @ Best, Result
    ///
    /// Result classification (based on reps at the heaviest set):
    ///   Struggled  — fewer than 8 reps
    ///   Succeeded  — 8 to 10 reps
    ///   Exceeded   — 11 or more reps
    ///
    /// Example:
    ///   hevy-bridge process-workout --json '{"workoutId":"ae4f95df-..."}'
    ProcessWorkout {
        /// Raw JSON webhook payload containing a "workoutId" field.
        #[arg(long)]
        json: String,
    },
}

// ── Config ────────────────────────────────────────────

#[derive(Subcommand, Debug)]
enum ConfigCommands {
    /// Save your API key to ~/.config/hevy-bridge/config.json
    ///
    /// Example: hevy-bridge config set-key abc123-def456-...
    SetKey {
        /// The Hevy API key to store.
        key: String,
    },

    /// Print the path to the config file.
    Path,
}

// ── User ──────────────────────────────────────────────

#[derive(Subcommand, Debug)]
enum UserCommands {
    /// Get the authenticated user's profile information.
    ///
    /// Returns JSON with: id, name, url
    ///
    /// Example: hevy-bridge user info
    Info,
}

// ── Workouts ──────────────────────────────────────────

#[derive(Subcommand, Debug)]
enum WorkoutCommands {
    /// List workouts (paginated).
    ///
    /// Returns a JSON object with: page, page_count, workouts[]
    /// Each workout includes: id, title, description, start_time, end_time,
    /// created_at, updated_at, routine_id, and exercises[].
    ///
    /// Example: hevy-bridge workouts list --page 1 --page-size 5
    List {
        /// Page number (1-based).
        #[arg(long, default_value_t = 1)]
        page: u32,

        /// Items per page (max 10).
        #[arg(long, default_value_t = 5)]
        page_size: u32,
    },

    /// Get a single workout by its ID.
    ///
    /// Returns the full workout JSON including all exercises and sets.
    ///
    /// Example: hevy-bridge workouts get b459cba5-cd6d-463c-abd6-54f8eafcadcb
    Get {
        /// The workout ID (UUID).
        id: String,
    },

    /// Get the total number of workouts on the account.
    ///
    /// Returns JSON: { "workout_count": <number> }
    ///
    /// Example: hevy-bridge workouts count
    Count,

    /// List workout events (updates and deletes) since a given date.
    ///
    /// Useful for syncing a local cache. Events are ordered newest to oldest.
    /// Returns: page, page_count, events[] (each tagged "updated" or "deleted").
    ///
    /// Example: hevy-bridge workouts events --since 2024-01-01T00:00:00Z
    Events {
        /// Page number (1-based).
        #[arg(long, default_value_t = 1)]
        page: u32,

        /// Items per page (max 10).
        #[arg(long, default_value_t = 5)]
        page_size: u32,

        /// ISO 8601 date to filter events from (e.g. 2024-01-01T00:00:00Z).
        #[arg(long)]
        since: Option<String>,
    },

    /// Create a new workout.
    ///
    /// Accepts a JSON body describing the workout. The JSON must match the
    /// PostWorkoutsRequestBody schema:
    ///
    ///   {
    ///     "workout": {
    ///       "title": "Leg Day 🔥",
    ///       "description": "Optional description",
    ///       "start_time": "2024-08-14T12:00:00Z",
    ///       "end_time": "2024-08-14T12:30:00Z",
    ///       "is_private": false,
    ///       "exercises": [
    ///         {
    ///           "exercise_template_id": "D04AC939",
    ///           "superset_id": null,
    ///           "notes": "Felt good",
    ///           "sets": [
    ///             {
    ///               "type": "normal",
    ///               "weight_kg": 100,
    ///               "reps": 10,
    ///               "rpe": 8.5
    ///             }
    ///           ]
    ///         }
    ///       ]
    ///     }
    ///   }
    ///
    /// Set types: "normal", "warmup", "failure", "dropset"
    /// RPE values: 6, 7, 7.5, 8, 8.5, 9, 9.5, 10
    ///
    /// Example: hevy-bridge workouts create --json '{"workout":{...}}'
    Create {
        /// Raw JSON body (PostWorkoutsRequestBody).
        #[arg(long)]
        json: String,
    },

    /// Update an existing workout.
    ///
    /// Takes the workout ID and a JSON body with the same schema as create.
    ///
    /// Example: hevy-bridge workouts update <ID> --json '{"workout":{...}}'
    Update {
        /// The workout ID to update (UUID).
        id: String,

        /// Raw JSON body (PostWorkoutsRequestBody).
        #[arg(long)]
        json: String,
    },
}

// ── Routines ──────────────────────────────────────────

#[derive(Subcommand, Debug)]
enum RoutineCommands {
    /// List routines (paginated).
    ///
    /// Returns: page, page_count, routines[]
    /// Each routine includes exercises with target sets and optional rep_range.
    ///
    /// Example: hevy-bridge routines list --page 1 --page-size 5
    List {
        /// Page number (1-based).
        #[arg(long, default_value_t = 1)]
        page: u32,

        /// Items per page (max 10).
        #[arg(long, default_value_t = 5)]
        page_size: u32,
    },

    /// Get a single routine by its ID.
    ///
    /// Example: hevy-bridge routines get <ROUTINE_ID>
    Get {
        /// The routine ID.
        id: String,
    },

    /// Create a new routine.
    ///
    /// JSON schema (PostRoutinesRequestBody):
    ///
    ///   {
    ///     "routine": {
    ///       "title": "Push Day",
    ///       "folder_id": null,
    ///       "notes": "Focus on form",
    ///       "exercises": [
    ///         {
    ///           "exercise_template_id": "D04AC939",
    ///           "superset_id": null,
    ///           "rest_seconds": 90,
    ///           "notes": "Slow and controlled",
    ///           "sets": [
    ///             {
    ///               "type": "normal",
    ///               "weight_kg": 80,
    ///               "reps": 10,
    ///               "rep_range": { "start": 8, "end": 12 }
    ///             }
    ///           ]
    ///         }
    ///       ]
    ///     }
    ///   }
    ///
    /// Example: hevy-bridge routines create --json '{"routine":{...}}'
    Create {
        /// Raw JSON body (PostRoutinesRequestBody).
        #[arg(long)]
        json: String,
    },

    /// Update an existing routine.
    ///
    /// JSON schema (PutRoutinesRequestBody) — same as create but without folder_id:
    ///
    ///   {
    ///     "routine": {
    ///       "title": "Updated Push Day",
    ///       "notes": "...",
    ///       "exercises": [...]
    ///     }
    ///   }
    ///
    /// Example: hevy-bridge routines update <ID> --json '{"routine":{...}}'
    Update {
        /// The routine ID to update.
        id: String,

        /// Raw JSON body (PutRoutinesRequestBody).
        #[arg(long)]
        json: String,
    },
}

// ── Exercises ─────────────────────────────────────────

#[derive(Subcommand, Debug)]
enum ExerciseCommands {
    /// List exercise templates (paginated).
    ///
    /// Returns: page, page_count, exercise_templates[]
    /// Each template: id, title, type, primary_muscle_group,
    /// secondary_muscle_groups, is_custom.
    ///
    /// TIP: Use --page-size 100 (max) to fetch many at once.
    ///
    /// Example: hevy-bridge exercises list --page-size 100
    List {
        /// Page number (1-based).
        #[arg(long, default_value_t = 1)]
        page: u32,

        /// Items per page (max 100).
        #[arg(long, default_value_t = 5)]
        page_size: u32,
    },

    /// Get a single exercise template by ID.
    ///
    /// Example: hevy-bridge exercises get D04AC939
    Get {
        /// The exercise template ID.
        id: String,
    },

    /// Create a custom exercise template.
    ///
    /// JSON schema (CreateCustomExerciseRequestBody):
    ///
    ///   {
    ///     "exercise": {
    ///       "title": "My Custom Press",
    ///       "exercise_type": "weight_reps",
    ///       "equipment_category": "barbell",
    ///       "muscle_group": "chest",
    ///       "other_muscles": ["triceps", "shoulders"]
    ///     }
    ///   }
    ///
    /// exercise_type values:
    ///   weight_reps, reps_only, bodyweight_reps,
    ///   bodyweight_assisted_reps, duration, weight_duration,
    ///   distance_duration, short_distance_weight
    ///
    /// equipment_category values:
    ///   none, barbell, dumbbell, kettlebell, machine, plate,
    ///   resistance_band, suspension, other
    ///
    /// muscle_group values:
    ///   abdominals, shoulders, biceps, triceps, forearms,
    ///   quadriceps, hamstrings, calves, glutes, abductors,
    ///   adductors, lats, upper_back, traps, lower_back,
    ///   chest, cardio, neck, full_body, other
    ///
    /// Example: hevy-bridge exercises create --json '{"exercise":{...}}'
    Create {
        /// Raw JSON body (CreateCustomExerciseRequestBody).
        #[arg(long)]
        json: String,
    },
}

// ── Folders ───────────────────────────────────────────

#[derive(Subcommand, Debug)]
enum FolderCommands {
    /// List routine folders (paginated).
    ///
    /// Returns: page, page_count, routine_folders[]
    /// Each folder: id, index, title, updated_at, created_at.
    ///
    /// Example: hevy-bridge folders list
    List {
        /// Page number (1-based).
        #[arg(long, default_value_t = 1)]
        page: u32,

        /// Items per page (max 10).
        #[arg(long, default_value_t = 5)]
        page_size: u32,
    },

    /// Get a single routine folder by ID.
    ///
    /// Example: hevy-bridge folders get 42
    Get {
        /// The folder ID.
        id: String,
    },

    /// Create a new routine folder.
    ///
    /// The folder is created at index 0; existing folders shift up.
    ///
    /// JSON schema:
    ///   { "routine_folder": { "title": "Push Pull 🏋️‍♂️" } }
    ///
    /// Example: hevy-bridge folders create --json '{"routine_folder":{"title":"My Folder"}}'
    Create {
        /// Raw JSON body (PostRoutineFolderRequestBody).
        #[arg(long)]
        json: String,
    },
}

// ── History ───────────────────────────────────────────

#[derive(Subcommand, Debug)]
enum HistoryCommands {
    /// Get set-level history for a specific exercise template.
    ///
    /// Returns every set ever recorded for the given exercise, each with
    /// workout context (workout_id, title, timestamps) and set data
    /// (weight_kg, reps, rpe, distance_meters, duration_seconds, set_type).
    ///
    /// Optionally filter by date range (ISO 8601).
    ///
    /// Example:
    ///   hevy-bridge history get D04AC939
    ///   hevy-bridge history get D04AC939 --start 2024-01-01T00:00:00Z --end 2024-12-31T23:59:59Z
    Get {
        /// The exercise template ID.
        exercise_template_id: String,

        /// Optional start date filter (ISO 8601).
        #[arg(long)]
        start: Option<String>,

        /// Optional end date filter (ISO 8601).
        #[arg(long)]
        end: Option<String>,
    },
}

// ─────────────────────────────────────────────────────
// Entrypoint
// ─────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // ── Config ─────────────────────────
        Commands::Config(cmd) => match cmd {
            ConfigCommands::SetKey { key } => {
                store_api_key(&key)?;
                eprintln!("✓ API key saved to {}", config_path().display());
            }
            ConfigCommands::Path => {
                println!("{}", config_path().display());
            }
        },

        // ── User ───────────────────────────
        Commands::User(cmd) => {
            let api_key = resolve_api_key(&cli.api_key)?;
            let client = HevyClient::new(api_key);
            match cmd {
                UserCommands::Info => {
                    let info = client.user_info().await?;
                    println!("{}", serde_json::to_string_pretty(&info)?);
                }
            }
        }

        // ── Workouts ───────────────────────
        Commands::Workouts(cmd) => {
            let api_key = resolve_api_key(&cli.api_key)?;
            let client = HevyClient::new(api_key);
            match cmd {
                WorkoutCommands::List { page, page_size } => {
                    let data = client.list_workouts(page, page_size).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                WorkoutCommands::Get { id } => {
                    let data = client.get_workout(&id).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                WorkoutCommands::Count => {
                    let data = client.workout_count().await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                WorkoutCommands::Events {
                    page,
                    page_size,
                    since,
                } => {
                    let data = client
                        .workout_events(page, page_size, since.as_deref())
                        .await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                WorkoutCommands::Create { json } => {
                    let body: PostWorkoutBody = serde_json::from_str(&json)
                        .context("Invalid JSON for workout body. See `hevy-bridge workouts create --help` for the expected schema.")?;
                    let data = client.create_workout(&body).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                WorkoutCommands::Update { id, json } => {
                    let body: PostWorkoutBody = serde_json::from_str(&json)
                        .context("Invalid JSON for workout body. See `hevy-bridge workouts update --help` for the expected schema.")?;
                    let data = client.update_workout(&id, &body).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
            }
        }

        // ── Routines ──────────────────────
        Commands::Routines(cmd) => {
            let api_key = resolve_api_key(&cli.api_key)?;
            let client = HevyClient::new(api_key);
            match cmd {
                RoutineCommands::List { page, page_size } => {
                    let data = client.list_routines(page, page_size).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                RoutineCommands::Get { id } => {
                    let data = client.get_routine(&id).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                RoutineCommands::Create { json } => {
                    let body: PostRoutineBody = serde_json::from_str(&json)
                        .context("Invalid JSON for routine body. See `hevy-bridge routines create --help` for the expected schema.")?;
                    let data = client.create_routine(&body).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                RoutineCommands::Update { id, json } => {
                    let body: PutRoutineBody = serde_json::from_str(&json)
                        .context("Invalid JSON for routine body. See `hevy-bridge routines update --help` for the expected schema.")?;
                    let data = client.update_routine(&id, &body).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
            }
        }

        // ── Exercises ─────────────────────
        Commands::Exercises(cmd) => {
            let api_key = resolve_api_key(&cli.api_key)?;
            let client = HevyClient::new(api_key);
            match cmd {
                ExerciseCommands::List { page, page_size } => {
                    let data = client.list_exercise_templates(page, page_size).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                ExerciseCommands::Get { id } => {
                    let data = client.get_exercise_template(&id).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                ExerciseCommands::Create { json } => {
                    let body: CreateExerciseBody = serde_json::from_str(&json)
                        .context("Invalid JSON for exercise body. See `hevy-bridge exercises create --help` for the expected schema.")?;
                    let data = client.create_exercise_template(&body).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
            }
        }

        // ── Folders ───────────────────────
        Commands::Folders(cmd) => {
            let api_key = resolve_api_key(&cli.api_key)?;
            let client = HevyClient::new(api_key);
            match cmd {
                FolderCommands::List { page, page_size } => {
                    let data = client.list_routine_folders(page, page_size).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                FolderCommands::Get { id } => {
                    let data = client.get_routine_folder(&id).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
                FolderCommands::Create { json } => {
                    let body: PostRoutineFolderBody = serde_json::from_str(&json)
                        .context("Invalid JSON for folder body. See `hevy-bridge folders create --help` for the expected schema.")?;
                    let data = client.create_routine_folder(&body).await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
            }
        }

        // ── History ───────────────────────
        Commands::History(cmd) => {
            let api_key = resolve_api_key(&cli.api_key)?;
            let client = HevyClient::new(api_key);
            match cmd {
                HistoryCommands::Get {
                    exercise_template_id,
                    start,
                    end,
                } => {
                    let data = client
                        .exercise_history(
                            &exercise_template_id,
                            start.as_deref(),
                            end.as_deref(),
                        )
                        .await?;
                    println!("{}", serde_json::to_string_pretty(&data)?);
                }
            }
        }

        // ── Process Workout ───────────────
        Commands::ProcessWorkout { json } => {
            let payload: WebhookPayload = serde_json::from_str(&json)
                .context("Invalid webhook JSON. Expected: {\"workoutId\":\"<UUID>\"}")?;

            let api_key = resolve_api_key(&cli.api_key)?;
            let client = HevyClient::new(api_key);
            let workout = client.get_workout(&payload.workout_id).await?;

            // If the workout is based on a routine, fetch it for per-set targets
            let routine = if let Some(ref routine_id) = workout.routine_id {
                client.get_routine(routine_id).await.ok().map(|r| r.routine)
            } else {
                None
            };

            // Build a per-set lookup: (exercise_template_id, set_index) -> (lo, hi)
            let mut set_targets: std::collections::HashMap<(String, usize), (i64, i64)> =
                std::collections::HashMap::new();
            if let Some(ref r) = routine {
                for ex in &r.exercises {
                    if let Some(ref tmpl_id) = ex.exercise_template_id {
                        for (i, s) in ex.sets.iter().enumerate() {
                            let (lo, hi) = if let Some(ref range) = s.rep_range {
                                let lo = range.start.map(|v| v as i64).unwrap_or(8);
                                let hi = range.end.map(|v| v as i64).unwrap_or(lo);
                                (lo, hi)
                            } else {
                                let r = s.reps.map(|v| v as i64).unwrap_or(10);
                                (r.saturating_sub(1), r + 1)
                            };
                            set_targets.insert((tmpl_id.clone(), i), (lo, hi));
                        }
                    }
                }
            }

            let title = workout.title.as_deref().unwrap_or("Untitled Workout");
            println!();
            println!("  {title}");
            println!("  {}", "─".repeat(title.len()));
            if let Some(ref routine_id) = workout.routine_id {
                println!("  Routine ID: {routine_id}");
            }
            println!();

            // ── Routine table (printed first when available) ──
            if let Some(ref routine) = routine {
                let routine_title = routine.title.as_deref().unwrap_or("Untitled Routine");

                println!("  Routine: {routine_title}");
                println!("  {}", "─".repeat(routine_title.len() + 10));
                println!();

                println!(
                    "  {:<35} {:>5} {:>18} {:>12} {:>12}   {}",
                    "Exercise", "Sets", "Target Wt (lbs)", "Target Reps", "Rest (s)", "Notes"
                );
                println!("  {}", "─".repeat(120));

                for exercise in &routine.exercises {
                    let ex_title = exercise
                        .title
                        .as_deref()
                        .unwrap_or("Unknown Exercise");
                    let notes = exercise.notes.as_deref().unwrap_or("");
                    let num_sets = exercise.sets.len();

                    let rest = exercise
                        .rest_seconds
                        .as_ref()
                        .and_then(|v| v.as_f64())
                        .map(|v| format!("{}", v as i64))
                        .unwrap_or_else(|| "—".to_string());

                    // Show the heaviest target weight and its rep range
                    let (best_kg, reps_display) = exercise
                        .sets
                        .iter()
                        .map(|s| {
                            let w = s.weight_kg.unwrap_or(0.0);
                            let rep_str = if let Some(ref range) = s.rep_range {
                                let lo = range.start.map(|v| v as i64);
                                let hi = range.end.map(|v| v as i64);
                                match (lo, hi) {
                                    (Some(l), Some(h)) => format!("{l}-{h}"),
                                    (Some(l), None) => format!("{l}+"),
                                    _ => s.reps.map(|r| format!("{}", r as i64)).unwrap_or_else(|| "—".to_string()),
                                }
                            } else {
                                s.reps.map(|r| format!("{}", r as i64)).unwrap_or_else(|| "—".to_string())
                            };
                            (w, rep_str)
                        })
                        .fold((0.0_f64, "—".to_string()), |(bw, br), (w, r)| {
                            if w > bw { (w, r) } else { (bw, br) }
                        });

                    let best_lbs = best_kg * 2.20462;
                    let weight_str = if best_kg > 0.0 {
                        format!("{best_lbs:.1}")
                    } else {
                        "—".to_string()
                    };

                    println!(
                        "  {:<35} {:>5} {:>18} {:>12} {:>12}   {}",
                        truncate_str(ex_title, 35),
                        num_sets,
                        weight_str,
                        reps_display,
                        rest,
                        notes
                    );

                    // Indented per-set detail rows
                    for (i, s) in exercise.sets.iter().enumerate() {
                        let set_num = i + 1;
                        let set_label = format!(
                            "  Set {set_num}{}",
                            s.set_type
                                .as_ref()
                                .map(|t| format!(" ({t})"))
                                .unwrap_or_default()
                        );
                        let w_lbs = s.weight_kg.unwrap_or(0.0) * 2.20462;
                        let rep_str = if let Some(ref range) = s.rep_range {
                            let lo = range.start.map(|v| v as i64);
                            let hi = range.end.map(|v| v as i64);
                            match (lo, hi) {
                                (Some(l), Some(h)) => format!("{l}-{h}"),
                                (Some(l), None) => format!("{l}+"),
                                _ => s.reps.map(|r| format!("{}", r as i64)).unwrap_or_else(|| "—".to_string()),
                            }
                        } else {
                            s.reps.map(|r| format!("{}", r as i64)).unwrap_or_else(|| "—".to_string())
                        };
                        let w_str = if s.weight_kg.unwrap_or(0.0) > 0.0 {
                            format!("{w_lbs:.1}")
                        } else {
                            "—".to_string()
                        };
                        println!(
                            "  {:<35} {:>5} {:>18} {:>12} {:>12}   {}",
                            set_label,
                            "",
                            w_str,
                            rep_str,
                            "",
                            ""
                        );
                    }
                }

                println!();
            }

            // ── Workout results table ──
            println!(
                "  {:<35} {:>5} {:>18} {:>13} {:>12}   {}",
                "Exercise", "Sets", "Weight (lbs)", "Reps", "Result", "Notes"
            );
            println!("  {}", "─".repeat(120));

            for exercise in &workout.exercises {
                let ex_title = exercise
                    .title
                    .as_deref()
                    .unwrap_or("Unknown Exercise");
                let notes = exercise.notes.as_deref().unwrap_or("");
                let num_sets = exercise.sets.len();

                // Compute an overall result: worst individual set classification wins
                let mut has_struggled = false;
                let mut all_exceeded = true;
                for (i, s) in exercise.sets.iter().enumerate() {
                    let reps = s.reps.map(|v| v as i64).unwrap_or(0);
                    let (lo, hi) = exercise
                        .exercise_template_id
                        .as_ref()
                        .and_then(|id| set_targets.get(&(id.clone(), i)))
                        .copied()
                        .unwrap_or((8, 10));
                    if reps < lo {
                        has_struggled = true;
                        all_exceeded = false;
                    } else if reps <= hi {
                        all_exceeded = false;
                    }
                }
                let overall = if has_struggled {
                    "\x1b[33mStruggled\x1b[0m"
                } else if all_exceeded {
                    "\x1b[36mExceeded\x1b[0m"
                } else {
                    "\x1b[32mSucceeded\x1b[0m"
                };

                // Exercise summary row (no weight/reps — those are on the set rows)
                println!(
                    "  {:<35} {:>5} {:>18} {:>13} {:>21}   {}",
                    truncate_str(ex_title, 35),
                    num_sets,
                    "",
                    "",
                    overall,
                    notes
                );

                // Indented per-set detail rows with individual results
                for (i, s) in exercise.sets.iter().enumerate() {
                    let set_num = i + 1;
                    let set_label = format!(
                        "  Set {set_num}{}",
                        s.set_type
                            .as_ref()
                            .map(|t| format!(" ({t})"))
                            .unwrap_or_default()
                    );
                    let w_lbs = s.weight_kg.unwrap_or(0.0) * 2.20462;
                    let reps = s.reps.map(|v| v as i64);

                    let (lo, hi) = exercise
                        .exercise_template_id
                        .as_ref()
                        .and_then(|id| set_targets.get(&(id.clone(), i)))
                        .copied()
                        .unwrap_or((8, 10));

                    let r = reps.unwrap_or(0);
                    let result = if r < lo {
                        "\x1b[33mStruggled\x1b[0m"
                    } else if r <= hi {
                        "\x1b[32mSucceeded\x1b[0m"
                    } else {
                        "\x1b[36mExceeded\x1b[0m"
                    };

                    let rpe_str = s
                        .rpe
                        .map(|v| format!("RPE {v}"))
                        .unwrap_or_default();

                    println!(
                        "  {:<35} {:>5} {:>18.1} {:>13} {:>21}   {}",
                        set_label,
                        "",
                        w_lbs,
                        reps.map(|v| v.to_string()).unwrap_or_else(|| "—".to_string()),
                        result,
                        rpe_str
                    );
                }
            }

            println!();
        }
    }

    Ok(())
}

/// Truncate a string to `max` characters, appending "…" if shortened.
fn truncate_str(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max - 1).collect();
        format!("{truncated}…")
    }
}
