use std::path::{Path, PathBuf};

use tokio::process::Command;
use uuid::Uuid;

use crate::processor::helpers::copy_input_files;

pub async fn execute(
    dependencies_dir: &Path,
    data_dir: &Path,
    input_file_path: &Path,
    heavy_water_file_path: &Path,
    uuid: Option<(&Uuid, &Uuid)>, // 1st=input file uuid; 2nd=heavy water file uuid
) -> Result<PathBuf, String> {
    let (input_file_path, heavy_water_file_path, input_file_uuid, _heavy_water_file_uuid) = match uuid {
        Some((input_id, heavy_id)) => {
            (
                data_dir.join(format!("{input_id}.csv")),
                data_dir.join(format!("{heavy_id}.txt")),
                input_id.to_owned(),
                heavy_id.to_owned(),
            )
        },
        None => copy_input_files(data_dir, input_file_path, heavy_water_file_path).await?,
    };

    let mut command = Command::new(dependencies_dir.join("SRM_Rate.exe"));
    command
        .arg(heavy_water_file_path.to_str().unwrap())
        .arg(input_file_path.to_str().unwrap());
        // additional context here: https://stackoverflow.com/questions/60750113/how-do-i-hide-the-console-window-for-a-process-started-with-stdprocesscomman
        // CREATE_NO_WINDOW flag. See: https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags#CREATE_NO_WINDOW
        //.creation_flags(0x08000000);

    let output = command
        .output()
        .await
        .map_err(|err| format!("Command couldn't run: {err}"))?;

    if output.status.success() {
        Ok(data_dir.join(format!("{input_file_uuid}.RateConst.csv")))
    } else {
        // dump command output
        dbg!(output);
        Err("The command didn't complete successfully".to_string())
    }
}