use std::io::Cursor;
use std::path::Path;
use futures::future::join_all;
use reqwest::Client;
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri::Manager;
use tokio::fs;
use tokio::fs::create_dir;
use crate::aggregator::aggregate;
use crate::analyzer::analyze_all;

use crate::grouper::{group_by_na_columns, group_by_peptides};
use crate::parser::{EngineType, InputFile, parse};
use crate::processor::process_file;
use crate::serializer::{serialize, serialize_calculations};
use tokio::task::{JoinHandle, JoinSet};

pub enum ProgressUpdate {
    Set {
        iterations: usize,
        total_iterations: Option<usize>,
    },
    Increment {
        iterations: usize,
    },
}

pub type ProgressCallback = Box<dyn Fn(ProgressUpdate) + Send + Sync>;

#[derive(Clone, serde::Serialize)]
struct ProgressSetPayload {
    uuid: String,
    iterations: usize,
    total_iterations: Option<usize>,
}

#[derive(Clone, serde::Serialize)]
struct ProgressIncrementPayload {
    uuid: String,
    iterations: usize,
}

#[derive(Clone, serde::Serialize)]
struct ErrorPayload {
    uuid: String,
    message: String,
}

#[tauri::command]
pub async fn process_data(
    app: tauri::AppHandle,
    engine_type: EngineType,
    should_remove_na_calculations: bool,
    tolerance_multiplier: f64,
    input_files: Vec<InputFile>,
) -> Result<(), String> {
    // TODO: https://tauri.app/v1/guides/features/events/
    dbg!("Run");
    dbg!(&input_files);

    let deps_path = match engine_type {
        EngineType::Single =>
            app.path_resolver()
                .resolve_resource("assets").unwrap().join("single-timepoint-engine"),
        EngineType::Multi => app.path_resolver()
            .resolve_resource("assets").unwrap().join("multi-timepoint-engine"),
    };

    let window = app.get_window("main").unwrap();
    let mut tasks: Vec<JoinHandle<anyhow::Result<()>>> = vec![];

    for input_file in input_files {
        dbg!("hellno");
        let deps_path = deps_path.clone();
        let window = window.clone();
        let input_uuid = input_file.uuid.clone();

        let task = tokio::spawn(async move {
            let window2 = window.clone();
            let progress_callback: ProgressCallback = Box::new(move |update| {
                match update {
                    ProgressUpdate::Set { iterations, total_iterations } => {
                        let payload = ProgressSetPayload {
                            uuid: input_uuid.clone(),
                            iterations,
                            total_iterations,
                        };

                        window2.emit("progress-set", payload).unwrap();
                    }
                    ProgressUpdate::Increment { iterations } => {
                        let payload = ProgressIncrementPayload {
                            uuid: input_uuid.clone(),
                            iterations,
                        };

                        window2.emit("progress-update", payload).unwrap();
                    }
                }
            });

            let input_uuid = input_file.uuid.clone();

            match process_file(
                &deps_path,
                should_remove_na_calculations,
                tolerance_multiplier,
                input_file,
                progress_callback,
            ).await {
                Ok(()) => Ok(()),
                Err(err) => {
                    dbg!(&err);
                    let payload = ErrorPayload {
                        uuid: input_uuid,
                        message: err.to_string(),
                    };
                    window.emit("process-error", payload).unwrap();
                    Err(err)
                }
            }
        });

        dbg!("hello yes");

        tasks.push(task);
    }

    join_all(tasks).await;

    Ok(())
}