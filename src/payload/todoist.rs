use serde::{Deserialize, Deserializer};
use serde::de::Unexpected;

struct TodoistPayload {
    user_id: i64,
    event_name: String,
    event_data: Box<dyn TodoistEvent>,
    initator: TodoistCollaborator
}

/// for all events, see https://developer.todoist.com/sync/v8/#configuration
trait TodoistEvent {
    fn event_name(&self) -> &'static str;
}

// #[todoist_event(event_name = "item:added")]
struct ItemAdded(TodoistItem);

// #[todoist_event("item:added")]
struct ItemRemoved(TodoistItem);

// #[todoist_event("item:added")]
struct ItemDeleted(TodoistItem);

// #[todoist_event("item:added")]
struct ItemCompleted(TodoistItem);

// #[todoist_event("item:added")]
struct ItemUncompleted(TodoistItem);

// #[todoist_event("item:added")]
struct NoteAdded(TodoistNote);

// #[todoist_event("item:added")]
struct NoteUpdated(TodoistNote);

// #[todoist_event("item:added")]
struct NoteDeleted(TodoistNote);

// #[todoist_event("item:added")]
struct ProjectAdded(TodoistProject);

// #[todoist_event("item:added")]
struct ProjectUpdated(TodoistProject);

// #[todoist_event("item:added")]
struct ProjectDeleted(TodoistProject);

// #[todoist_event("item:added")]
struct ProjectArchived(TodoistProject);

// #[todoist_event("item:added")]
struct ProjectUnarchived(TodoistProject);

// #[todoist_event("item:added")]
struct SectionAdded(TodoistSession);

// #[todoist_event("item:added")]
struct SectionUpdated(TodoistSession);

// #[todoist_event("item:added")]
struct SectionDeleted(TodoistSession);

// #[todoist_event("item:added")]
struct SectionArchived(TodoistSession);

// #[todoist_event("item:added")]
struct SectionUnarchived(TodoistSession);

// #[todoist_event("item:added")]
struct LabelAdded(TodoistSession);

// #[todoist_event("item:added")]
struct LabelDeleted(TodoistSession);

// #[todoist_event("item:added")]
struct LabelUpdated(TodoistSession);

// #[todoist_event("item:added")]
struct FilterAdded(TodoistFilter);

// #[todoist_event("item:added")]
struct FilterDeleted(TodoistFilter);

// #[todoist_event("item:added")]
struct FilterUpdated(TodoistFilter);

// #[todoist_event("item:added")]
struct ReminderFired(TodoistReminder);

/// please see https://developer.todoist.com/sync/v8/#items
#[derive(Deserialize)]
struct TodoistItem {
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

#[derive(Deserialize)]
struct TodoistCollaborator {
    // TODO: fill fields
}

#[derive(Deserialize)]
struct TodoistNote {
    // TODO: fill fields
}

#[derive(Deserialize)]
struct TodoistProject {
    // TODO: fill fields
}

#[derive(Deserialize)]
struct TodoistSession {
    // TODO: fill fields
}

#[derive(Deserialize)]
struct TodoistFilter {
    // TODO: fill fields
}

#[derive(Deserialize)]
struct TodoistReminder {
    // TODO: fill fields
}

#[derive(Deserialize)]
struct TaskID(i64);

#[derive(Deserialize)]
struct LegacyTaskID(i64);

#[derive(Deserialize)]
struct UserID(i64);

#[derive(Deserialize)]
struct ProjectID(i64);

#[derive(Deserialize)]
struct LegacyProjectID(i64);

#[derive(Deserialize)]
struct Due(i64); // TODO: this seems invalid

#[derive(Deserialize)]
struct SectionID(i64);

#[derive(Deserialize)]
struct SyncID(i64); // TODO: this seems invalid

#[derive(Deserialize)]
struct TodoistDate;
