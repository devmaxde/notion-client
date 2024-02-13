use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Number;
use serde_with::skip_serializing_none;

use super::{emoji::Emoji, file::File, parent::Parent, rich_text::RichText, user::User};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Page {
    pub object: String,
    pub id: String,
    pub created_time: DateTime<Utc>,
    pub created_by: User,
    pub last_edited_time: DateTime<Utc>,
    pub last_edited_by: User,
    pub archived: bool,
    pub icon: Option<Icon>,
    pub cover: Option<File>,
    pub properties: HashMap<String, PageProperty>,
    pub parent: Parent,
    pub url: String,
    pub public_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Icon {
    File(File),
    Emoji(Emoji),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PageProperty {
    Checkbox {
        id: String,
        checkbox: bool,
    },
    CreatedBy {
        id: String,
        created_by: User,
    },
    CreatedTime {
        id: String,
        created_time: DateTime<Utc>,
    },
    Date {
        id: String,
        date: DatePropertyValue,
    },
    Email {
        id: String,
        email: String,
    },
    Files {
        id: String,
        files: Vec<FilePropertyValue>,
    },
    Formula {
        id: String,
        formula: FormulaPropertyValue,
    },
    LastEditedBy {
        id: String,
        last_edited_by: User,
    },
    LastEditedTime {
        id: String,
        last_edited_time: DateTime<Utc>,
    },
    MultiSelect {
        id: String,
        multi_select: Vec<SelectPropertyValue>,
    },
    Number {
        id: String,
        number: Number,
    },
    People {
        id: String,
        people: Vec<User>,
    },
    PhoneNumber {
        id: String,
        phone_number: String,
    },
    Relation {
        id: String,
        relation: Vec<RelationPropertyValue>,
        has_more: bool,
    },
    Rollup {
        id: String,
        rollup: RollupPropertyValue,
    },
    RichText {
        id: String,
        rich_text: Vec<RichText>,
    },
    Select {
        id: String,
        select: SelectPropertyValue,
    },
    Status {
        id: String,
        status: SelectPropertyValue,
    },
    Title {
        id: String,
        title: Vec<RichText>,
    },
    Url {
        id: String,
        url: String,
    },
    #[serde(rename = "unique_id")]
    UniqueID {
        id: String,
        unique_id: UniqueIDPropertyValue,
    },
    Verification {
        id: String,
        verification: VerificationPropertyValue,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Default,
    Gray,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct DatePropertyValue {
    pub start: Option<DateOrDateTime>,
    pub end: Option<DateOrDateTime>,
    pub time_zone: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum DateOrDateTime {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormulaPropertyValue {
    String { string: Option<String> },
    Number { number: Option<Number> },
    Boolean { boolean: bool },
    Date { date: Option<DatePropertyValue> },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct RelationPropertyValue {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RollupPropertyValue {
    Array {
        function: RollupFunction,
        array: Vec<RollupPropertyValue>,
    },
    Date {
        function: RollupFunction,
        date: DateTime<Utc>,
    },
    Incomplete {
        function: RollupFunction,
        incomplete: Option<String>,
    },
    Number {
        function: RollupFunction,
        number: Number,
    },
    Unsupported {
        function: RollupFunction,
        unsupported: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    Average,
    Checked,
    Count,
    CountPerGroup,
    CountValues,
    DateRange,
    EarliestDate,
    Empty,
    LatestDate,
    Max,
    Median,
    Min,
    NotEmpty,
    PercentChecked,
    PercentEmpty,
    PercentNotEmpty,
    PercentPerGroup,
    PercentUnchecked,
    Range,
    ShowOriginal,
    ShowUnique,
    Sum,
    Unchecked,
    Unique,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FilePropertyValue {
    pub name: String,
    #[serde(flatten)]
    pub file: File,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SelectPropertyValue {
    pub color: Color,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct UniqueIDPropertyValue {
    pub number: Number,
    pub prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct VerificationPropertyValue {
    pub state: VerificationState,
    pub verified_by: Option<User>,
    pub date: Option<DatePropertyValue>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VerificationState {
    Verified,
    Unverified,
}