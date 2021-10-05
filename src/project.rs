// Based on https://gist.github.com/diwic/5c20a283ca3a03752e1a27b0f3ebfa30
// See https://old.reddit.com/r/rust/comments/4xneq5/the_calendar_example_challenge_ii_why_eddyb_all/

use anyhow::Context as AnyhowContext;
use anyhow::{anyhow, Result};
use std::fmt;

const COL_WIDTH: usize = 21;

use chrono::{Datelike, Duration, Local, Month, NaiveDate, NaiveDateTime, TimeZone};

use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Block, Widget},
};

use crate::table::TableState;
use itertools::Itertools;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::process::{Command, Output};
use task_hookrs::project::Project;
use uuid::Uuid;

pub struct Projects {
    pub(crate) list: Vec<Project>,
    pub table_state: TableState,
    pub details: HashMap<Uuid, String>,
    pub marked: HashSet<Uuid>,
    pub current_selection: usize,
    pub current_selection_uuid: Option<Uuid>,
    pub current_selection_id: Option<u64>,
    pub report_table: ProjectReportTable,
    pub report_show_info: bool,
    pub report_height: u16,
    pub details_scroll: u16,
}

impl Projects {
    pub(crate) fn new() -> Result<Self> {
        let virtual_tags = vec!["PROJECT", "TASKS"];
        let mut projects_report_table = ProjectReportTable {
            columns: vec![],
            rows: vec![],
            virtual_tags: virtual_tags.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
            description_width: 100,
        };
        projects_report_table.load_data()?;
        Ok(Self {
            list: projects_report_table
                .rows
                .iter()
                .map(|x| x.project.clone())
                .collect_vec(),
            table_state: Default::default(),
            details: Default::default(),
            marked: Default::default(),
            current_selection: 0,
            current_selection_uuid: None,
            current_selection_id: None,
            report_table: projects_report_table,
            report_show_info: false,
            report_height: 0,
            details_scroll: 0,
        })
    }
}

pub struct ProjectReportTable {
    pub columns: Vec<String>,
    pub rows: Vec<ProjectDetails>,
    pub virtual_tags: Vec<String>,
    pub description_width: usize,
}
pub struct ProjectDetails {
    project: Project,
    tasks: usize,
}
impl ProjectReportTable {
    pub fn load_data(&mut self) -> Result<()> {
        self.columns = vec![format!("{}", "Project"), format!("{}", "Tasks")];

        let output = Command::new("task")
            .arg("projects")
            .output()
            .context("Unable to ryn `task projects`")
            .unwrap();
        let data = String::from_utf8_lossy(&output.stdout);
        for line in data.split('\n').into_iter().skip(3) {
            if !line.is_empty() {
                let pair: Vec<String> = line
                    .split(' ')
                    .map(|x| x.trim())
                    .map(|x| x.trim_start())
                    .filter(|x| !x.is_empty())
                    .map(|x| x.to_string())
                    .collect();
                println!("{:?}", &line);
                self.rows.push(ProjectDetails {
                    project: (&pair[0]).parse()?,
                    tasks: (&pair[1]).parse()?,
                });
            } else {
                break;
            }
        }

        Ok(())
    }
}
