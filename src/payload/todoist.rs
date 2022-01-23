use serde::{Deserialize, Deserializer};
use serde::de::Unexpected;

#[derive(Deserialize)]
pub struct TodoistPayload {
    user_id: i64,
    #[serde(flatten)]
    event: TodoistEvent,
    #[serde(rename = "version_number")]
    version: String,
    initator: TodoistCollaborator
}

/// for all events, see https://developer.todoist.com/sync/v8/#configuration
#[derive(Deserialize)]
#[serde(tag = "event_name")]
enum TodoistEvent {
    // TODO: replace those boilerplate with proc-macro
    #[serde(rename = "item:added")]
    ItemAdded(
        #[serde(rename = "event_data")]
        TodoistItem
    ),
    ItemRemoved(TodoistItem),
    ItemDeleted(TodoistItem),
    ItemCompleted(TodoistItem),
    ItemUncompleted(TodoistItem),
    NoteAdded(TodoistNote),
    NoteUpdated(TodoistNote),
    NoteDeleted(TodoistNote),
    ProjectAdded(TodoistProject),
    ProjectUpdated(TodoistProject),
    ProjectDeleted(TodoistProject),
    ProjectArchived(TodoistProject),
    ProjectUnarchived(TodoistProject),
    SectionAdded(TodoistSession),
    SectionUpdated(TodoistSession),
    SectionDeleted(TodoistSession),
    SectionArchived(TodoistSession),
    SectionUnarchived(TodoistSession),
    LabelAdded(TodoistSession),
    LabelDeleted(TodoistSession),
    LabelUpdated(TodoistSession),
    FilterAdded(TodoistFilter),
    FilterDeleted(TodoistFilter),
    FilterUpdated(TodoistFilter),
    ReminderFired(TodoistReminder),
}

// #[todoist_event(event_name = "item:added")]
struct ItemAdded(TodoistItem);
struct ItemRemoved(TodoistItem);
struct ItemDeleted(TodoistItem);
struct ItemCompleted(TodoistItem);
struct ItemUncompleted(TodoistItem);
struct NoteAdded(TodoistNote);
struct NoteUpdated(TodoistNote);
struct NoteDeleted(TodoistNote);
struct ProjectAdded(TodoistProject);
struct ProjectUpdated(TodoistProject);
struct ProjectDeleted(TodoistProject);
struct ProjectArchived(TodoistProject);
struct ProjectUnarchived(TodoistProject);
struct SectionAdded(TodoistSession);
struct SectionUpdated(TodoistSession);
struct SectionDeleted(TodoistSession);
struct SectionArchived(TodoistSession);
struct SectionUnarchived(TodoistSession);
struct LabelAdded(TodoistSession);
struct LabelDeleted(TodoistSession);
struct LabelUpdated(TodoistSession);
struct FilterAdded(TodoistFilter);
struct FilterDeleted(TodoistFilter);
struct FilterUpdated(TodoistFilter);
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

/// please see https://developer.todoist.com/sync/v8/#collaborators
#[derive(Deserialize)]
struct TodoistCollaborator {
    id: UserID,
    email: String,
    full_name: String,
    timezone: String,
    image_id: ImageID
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
struct ImageID(i64);

#[derive(Deserialize)]
struct TodoistDate;