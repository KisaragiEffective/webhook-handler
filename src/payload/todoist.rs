use serde::{Deserialize, Deserializer};
use serde::de::Unexpected;

#[derive(Deserialize)]
pub struct TodoistPayload {
    pub user_id: i64,
    #[serde(flatten)]
    pub event: TodoistEvent,
    #[serde(rename = "version_number")]
    pub version: String,
    pub initiator: TodoistCollaborator
}

/// for all events, see https://developer.todoist.com/sync/v8/#configuration
#[derive(Deserialize)]
#[serde(tag = "event_name")]
pub enum TodoistEvent {
    // TODO: replace those boilerplate with proc-macro
    #[serde(rename = "item:added")]
    ItemAdded(
        #[serde(rename = "event_data")]
        TodoistItem
    ),
    #[serde(rename = "item:removed")]
    ItemRemoved(
        #[serde(rename = "event_data")]
        TodoistItem
    ),
    #[serde(rename = "item:deleted")]
    ItemDeleted(
        #[serde(rename = "event_data")]
        TodoistItem
    ),
    #[serde(rename = "item:completed")]
    ItemCompleted(
        #[serde(rename = "event_data")]
        TodoistItem
    ),
    #[serde(rename = "item:uncompleted")]
    ItemUncompleted(
        #[serde(rename = "event_data")]
        TodoistItem
    ),
    #[serde(rename = "note:added")]
    NoteAdded(
        #[serde(rename = "event_data")]
        TodoistNote
    ),
    #[serde(rename = "note:updated")]
    NoteUpdated(
        #[serde(rename = "event_data")]
        TodoistNote
    ),
    #[serde(rename = "note:deleted")]
    NoteDeleted(
        #[serde(rename = "event_data")]
        TodoistNote
    ),
    #[serde(rename = "project:added")]
    ProjectAdded(
        #[serde(rename = "event_data")]
        TodoistProject
    ),
    #[serde(rename = "project:updated")]
    ProjectUpdated(
        #[serde(rename = "event_data")]
        TodoistProject
    ),
    #[serde(rename = "project:deleted")]
    ProjectDeleted(
        #[serde(rename = "event_data")]
        TodoistProject
    ),
    #[serde(rename = "project:archived")]
    ProjectArchived(
        #[serde(rename = "event_data")]
        TodoistProject
    ),
    #[serde(rename = "project:unarchived")]
    ProjectUnarchived(
        #[serde(rename = "event_data")]
        TodoistProject
    ),
    #[serde(rename = "section:added")]
    SectionAdded(
        #[serde(rename = "event_data")]
        TodoistSession
    ),
    #[serde(rename = "section:updated")]
    SectionUpdated(
        #[serde(rename = "event_data")]
        TodoistSession
    ),
    #[serde(rename = "section:deleted")]
    SectionDeleted(
        #[serde(rename = "event_data")]
        TodoistSession
    ),
    #[serde(rename = "section:archived")]
    SectionArchived(
        #[serde(rename = "event_data")]
        TodoistSession
    ),
    #[serde(rename = "section:unarchived")]
    SectionUnarchived(
        #[serde(rename = "event_data")]
        TodoistSession
    ),
    #[serde(rename = "label:added")]
    LabelAdded(
        #[serde(rename = "event_data")]
        TodoistSession
    ),
    #[serde(rename = "label:deleted")]
    LabelDeleted(
        #[serde(rename = "event_data")]
        TodoistSession
    ),
    #[serde(rename = "label:updated")]
    LabelUpdated(
        #[serde(rename = "event_data")]
        TodoistSession
    ),
    #[serde(rename = "filter:added")]
    FilterAdded(
        #[serde(rename = "event_data")]
        TodoistFilter
    ),
    #[serde(rename = "filter:deleted")]
    FilterDeleted(
        #[serde(rename = "event_data")]
        TodoistFilter
    ),
    #[serde(rename = "filter:updated")]
    FilterUpdated(
        #[serde(rename = "event_data")]
        TodoistFilter
    ),
    #[serde(rename = "reminder:fired")]
    ReminderFired(
        #[serde(rename = "event_data")]
        TodoistReminder
    ),


}

/// please see https://developer.todoist.com/sync/v8/#items
#[derive(Deserialize)]
pub struct TodoistItem {
    id: TaskID,
    legacy_id: Option<LegacyTaskID>,
    user_id: UserID,
    project_id: ProjectID,
    legacy_project_id: Option<LegacyProjectID>,
    content: String,
    description: String,
    due: Due,
    priority: TodoistPriority,
    parent_id: Option<TaskID>,
    legacy_parent_id: Option<LegacyTaskID>,
    child_order: u32,
    section_id: SectionID,
    day_order: u32,
    #[serde(deserialize_with = "deserialize_one_zero_bool")]
    collapsed: bool,
    labels: Vec<TaskID>,
    /// The UserID who created the task. This value is set to null on tasks created before 2019/10/31.
    added_by_uid: UserID,
    assigned_by_uid: UserID,
    responsible_uid: Option<UserID>,
    #[serde(rename = "checked", deserialize_with = "deserialize_one_zero_bool")]
    completed: bool,
    #[serde(deserialize_with = "deserialize_one_zero_bool")]
    in_history: bool,
    #[serde(deserialize_with = "deserialize_one_zero_bool")]
    is_deleted: bool,
    sync_id: Option<SyncID>,
    date_completed: TodoistDate,
    date_added: TodoistDate,
}

fn deserialize_one_zero_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    match u8::deserialize(deserializer) {
        Ok(a) => {
            match a {
                0 => Ok(true),
                1 => Ok(false),
                _ => Err(serde::de::Error::invalid_value(Unexpected::Unsigned(u64::from(a)),&"expected 0 or 1"))
            }
        },
        Err(b) => {
            Err(serde::de::Error::custom(b))
        }
    }
}

#[derive(Deserialize)]
enum TodoistPriority {
    // raw: 4
    P1,
    // raw: 3
    P2,
    // raw: 2
    P3,
    // raw: 1
    P4,
}

/// please see https://developer.todoist.com/sync/v8/#collaborators
#[derive(Deserialize)]
pub struct TodoistCollaborator {
    id: UserID,
    email: String,
    full_name: String,
    timezone: String,
    image_id: ImageID
}

#[derive(Deserialize)]
pub struct TodoistNote {
    // TODO: fill fields
}

#[derive(Deserialize)]
pub struct TodoistProject {
    // TODO: fill fields
}

#[derive(Deserialize)]
pub struct TodoistSession {
    // TODO: fill fields
}

#[derive(Deserialize)]
pub struct TodoistFilter {
    // TODO: fill fields
}

#[derive(Deserialize)]
pub struct TodoistReminder {
    // TODO: fill fields
}

#[derive(Deserialize)]
pub struct TaskID(i64);

#[derive(Deserialize)]
pub struct LegacyTaskID(i64);

#[derive(Deserialize)]
pub struct UserID(i64);

#[derive(Deserialize)]
pub struct ProjectID(i64);

#[derive(Deserialize)]
pub struct LegacyProjectID(i64);

#[derive(Deserialize)]
pub struct Due(i64); // TODO: this seems invalid

#[derive(Deserialize)]
pub struct SectionID(i64);

#[derive(Deserialize)]
pub struct SyncID(i64); // TODO: this seems invalid

#[derive(Deserialize)]
pub struct ImageID(i64);

#[derive(Deserialize)]
pub struct TodoistDate;
