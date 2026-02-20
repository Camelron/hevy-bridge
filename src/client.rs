use anyhow::{Context, Result};
use reqwest::Client;

use crate::models::*;

const BASE_URL: &str = "https://api.hevyapp.com/v1";

/// HTTP client wrapper for the Hevy API.
///
/// All endpoints require an API key passed via the `api-key` header.
/// Obtain your key at <https://hevy.com/settings?developer> (Hevy Pro required).
pub struct HevyClient {
    client: Client,
    api_key: String,
}

impl HevyClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    // ── Workouts ───────────────────────────────────────

    /// GET /v1/workouts — paginated list of workouts.
    pub async fn list_workouts(&self, page: u32, page_size: u32) -> Result<WorkoutsPage> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/workouts"))
            .header("api-key", &self.api_key)
            .query(&[("page", page), ("pageSize", page_size)])
            .send()
            .await
            .context("Failed to send request to GET /workouts")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /workouts returned {status}: {body}");
        }

        resp.json::<WorkoutsPage>()
            .await
            .context("Failed to parse workouts response")
    }

    /// GET /v1/workouts/{id} — single workout by ID.
    pub async fn get_workout(&self, workout_id: &str) -> Result<Workout> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/workouts/{workout_id}"))
            .header("api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send request to GET /workouts/{id}")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /workouts/{workout_id} returned {status}: {body}");
        }

        resp.json::<Workout>()
            .await
            .context("Failed to parse workout response")
    }

    /// POST /v1/workouts — create a new workout.
    pub async fn create_workout(&self, body: &PostWorkoutBody) -> Result<Workout> {
        let resp = self
            .client
            .post(format!("{BASE_URL}/workouts"))
            .header("api-key", &self.api_key)
            .json(body)
            .send()
            .await
            .context("Failed to send request to POST /workouts")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("POST /workouts returned {status}: {body}");
        }

        resp.json::<Workout>()
            .await
            .context("Failed to parse created workout response")
    }

    /// PUT /v1/workouts/{id} — update an existing workout.
    pub async fn update_workout(
        &self,
        workout_id: &str,
        body: &PostWorkoutBody,
    ) -> Result<Workout> {
        let resp = self
            .client
            .put(format!("{BASE_URL}/workouts/{workout_id}"))
            .header("api-key", &self.api_key)
            .json(body)
            .send()
            .await
            .context("Failed to send request to PUT /workouts/{id}")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("PUT /workouts/{workout_id} returned {status}: {body}");
        }

        resp.json::<Workout>()
            .await
            .context("Failed to parse updated workout response")
    }

    /// GET /v1/workouts/count — total workout count.
    pub async fn workout_count(&self) -> Result<WorkoutCountResponse> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/workouts/count"))
            .header("api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send request to GET /workouts/count")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /workouts/count returned {status}: {body}");
        }

        resp.json::<WorkoutCountResponse>()
            .await
            .context("Failed to parse workout count response")
    }

    /// GET /v1/workouts/events — paginated workout events (updates/deletes).
    pub async fn workout_events(
        &self,
        page: u32,
        page_size: u32,
        since: Option<&str>,
    ) -> Result<PaginatedWorkoutEvents> {
        let mut req = self
            .client
            .get(format!("{BASE_URL}/workouts/events"))
            .header("api-key", &self.api_key)
            .query(&[("page", page), ("pageSize", page_size)]);

        if let Some(since) = since {
            req = req.query(&[("since", since)]);
        }

        let resp = req
            .send()
            .await
            .context("Failed to send request to GET /workouts/events")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /workouts/events returned {status}: {body}");
        }

        resp.json::<PaginatedWorkoutEvents>()
            .await
            .context("Failed to parse workout events response")
    }

    // ── Routines ──────────────────────────────────────

    /// GET /v1/routines — paginated list of routines.
    pub async fn list_routines(&self, page: u32, page_size: u32) -> Result<RoutinesPage> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/routines"))
            .header("api-key", &self.api_key)
            .query(&[("page", page), ("pageSize", page_size)])
            .send()
            .await
            .context("Failed to send request to GET /routines")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /routines returned {status}: {body}");
        }

        resp.json::<RoutinesPage>()
            .await
            .context("Failed to parse routines response")
    }

    /// GET /v1/routines/{id} — single routine by ID.
    pub async fn get_routine(&self, routine_id: &str) -> Result<SingleRoutineResponse> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/routines/{routine_id}"))
            .header("api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send request to GET /routines/{id}")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /routines/{routine_id} returned {status}: {body}");
        }

        resp.json::<SingleRoutineResponse>()
            .await
            .context("Failed to parse routine response")
    }

    /// POST /v1/routines — create a new routine.
    pub async fn create_routine(&self, body: &PostRoutineBody) -> Result<Routine> {
        let resp = self
            .client
            .post(format!("{BASE_URL}/routines"))
            .header("api-key", &self.api_key)
            .json(body)
            .send()
            .await
            .context("Failed to send request to POST /routines")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("POST /routines returned {status}: {body}");
        }

        resp.json::<Routine>()
            .await
            .context("Failed to parse created routine response")
    }

    /// PUT /v1/routines/{id} — update an existing routine.
    pub async fn update_routine(
        &self,
        routine_id: &str,
        body: &PutRoutineBody,
    ) -> Result<Routine> {
        let resp = self
            .client
            .put(format!("{BASE_URL}/routines/{routine_id}"))
            .header("api-key", &self.api_key)
            .json(body)
            .send()
            .await
            .context("Failed to send request to PUT /routines/{id}")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("PUT /routines/{routine_id} returned {status}: {body}");
        }

        resp.json::<Routine>()
            .await
            .context("Failed to parse updated routine response")
    }

    // ── Exercise Templates ────────────────────────────

    /// GET /v1/exercise_templates — paginated list.
    pub async fn list_exercise_templates(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<ExerciseTemplatesPage> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/exercise_templates"))
            .header("api-key", &self.api_key)
            .query(&[("page", page), ("pageSize", page_size)])
            .send()
            .await
            .context("Failed to send request to GET /exercise_templates")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /exercise_templates returned {status}: {body}");
        }

        resp.json::<ExerciseTemplatesPage>()
            .await
            .context("Failed to parse exercise templates response")
    }

    /// GET /v1/exercise_templates/{id} — single template by ID.
    pub async fn get_exercise_template(&self, template_id: &str) -> Result<ExerciseTemplate> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/exercise_templates/{template_id}"))
            .header("api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send request to GET /exercise_templates/{id}")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /exercise_templates/{template_id} returned {status}: {body}");
        }

        resp.json::<ExerciseTemplate>()
            .await
            .context("Failed to parse exercise template response")
    }

    /// POST /v1/exercise_templates — create a custom exercise template.
    pub async fn create_exercise_template(
        &self,
        body: &CreateExerciseBody,
    ) -> Result<CreateExerciseResponse> {
        let resp = self
            .client
            .post(format!("{BASE_URL}/exercise_templates"))
            .header("api-key", &self.api_key)
            .json(body)
            .send()
            .await
            .context("Failed to send request to POST /exercise_templates")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("POST /exercise_templates returned {status}: {body}");
        }

        resp.json::<CreateExerciseResponse>()
            .await
            .context("Failed to parse create exercise template response")
    }

    // ── Routine Folders ───────────────────────────────

    /// GET /v1/routine_folders — paginated list.
    pub async fn list_routine_folders(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<RoutineFoldersPage> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/routine_folders"))
            .header("api-key", &self.api_key)
            .query(&[("page", page), ("pageSize", page_size)])
            .send()
            .await
            .context("Failed to send request to GET /routine_folders")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /routine_folders returned {status}: {body}");
        }

        resp.json::<RoutineFoldersPage>()
            .await
            .context("Failed to parse routine folders response")
    }

    /// GET /v1/routine_folders/{id} — single folder by ID.
    pub async fn get_routine_folder(&self, folder_id: &str) -> Result<RoutineFolder> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/routine_folders/{folder_id}"))
            .header("api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send request to GET /routine_folders/{id}")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /routine_folders/{folder_id} returned {status}: {body}");
        }

        resp.json::<RoutineFolder>()
            .await
            .context("Failed to parse routine folder response")
    }

    /// POST /v1/routine_folders — create a new routine folder.
    pub async fn create_routine_folder(
        &self,
        body: &PostRoutineFolderBody,
    ) -> Result<RoutineFolder> {
        let resp = self
            .client
            .post(format!("{BASE_URL}/routine_folders"))
            .header("api-key", &self.api_key)
            .json(body)
            .send()
            .await
            .context("Failed to send request to POST /routine_folders")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("POST /routine_folders returned {status}: {body}");
        }

        resp.json::<RoutineFolder>()
            .await
            .context("Failed to parse created routine folder response")
    }

    // ── Exercise History ──────────────────────────────

    /// GET /v1/exercise_history/{exerciseTemplateId} — history for a specific exercise.
    pub async fn exercise_history(
        &self,
        template_id: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<ExerciseHistoryResponse> {
        let mut req = self
            .client
            .get(format!("{BASE_URL}/exercise_history/{template_id}"))
            .header("api-key", &self.api_key);

        if let Some(s) = start_date {
            req = req.query(&[("start_date", s)]);
        }
        if let Some(e) = end_date {
            req = req.query(&[("end_date", e)]);
        }

        let resp = req
            .send()
            .await
            .context("Failed to send request to GET /exercise_history/{id}")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /exercise_history/{template_id} returned {status}: {body}");
        }

        resp.json::<ExerciseHistoryResponse>()
            .await
            .context("Failed to parse exercise history response")
    }

    // ── User ──────────────────────────────────────────

    /// GET /v1/user/info — authenticated user info.
    pub async fn user_info(&self) -> Result<UserInfoResponse> {
        let resp = self
            .client
            .get(format!("{BASE_URL}/user/info"))
            .header("api-key", &self.api_key)
            .send()
            .await
            .context("Failed to send request to GET /user/info")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GET /user/info returned {status}: {body}");
        }

        resp.json::<UserInfoResponse>()
            .await
            .context("Failed to parse user info response")
    }
}
