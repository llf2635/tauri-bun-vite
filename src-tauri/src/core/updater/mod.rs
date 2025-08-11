use tauri_plugin_updater::UpdaterExt;

// 检查更新，参考 https://v2.tauri.org.cn/plugin/updater/#checking-for-updates
#[cfg(desktop)]
pub(crate) async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        // 或者，我们也可以分别调用 update.download（） 和 update.install（）
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}

// 要将下载进度通知给前端，请考虑使用带有通道的命令。
#[cfg(desktop)]
mod app_updates {
    use serde::Serialize;
    use std::sync::Mutex;
    use tauri::{ipc::Channel, AppHandle, State};
    use tauri_plugin_updater::{Update, UpdaterExt};

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Updater(#[from] tauri_plugin_updater::Error),
        #[error("there is no pending update")]
        NoPendingUpdate,
    }

    impl Serialize for Error {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(self.to_string().as_str())
        }
    }

    type Result<T> = std::result::Result<T, Error>;

    #[derive(Clone, Serialize)]
    #[serde(tag = "event", content = "data")]
    pub enum DownloadEvent {
        #[serde(rename_all = "camelCase")]
        Started {
            content_length: Option<u64>,
        },
        #[serde(rename_all = "camelCase")]
        Progress {
            chunk_length: usize,
        },
        Finished,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UpdateMetadata {
        version: String,
        current_version: String,
    }

    #[tauri::command]
    pub async fn fetch_update(
        app: AppHandle,
        pending_update: State<'_, PendingUpdate>,
    ) -> Result<Option<UpdateMetadata>> {
        let channel = "stable";
        let url = url::Url::parse(&format!(
            "https://cdn.myupdater.com/{{{{target}}}}-{{{{arch}}}}/{{{{current_version}}}}?channel={channel}",
        )).expect("invalid URL");

        let update = app
            .updater_builder()
            .endpoints(vec![url])?
            .build()?
            .check()
            .await?;

        let update_metadata = update.as_ref().map(|update| UpdateMetadata {
            version: update.version.clone(),
            current_version: update.current_version.clone(),
        });

        *pending_update.0.lock().unwrap() = update;

        Ok(update_metadata)
    }

    #[tauri::command]
    pub async fn install_update(
        pending_update: State<'_, PendingUpdate>,
        on_event: Channel<DownloadEvent>,
    ) -> Result<()> {
        let Some(update) = pending_update.0.lock().unwrap().take() else {
            return Err(Error::NoPendingUpdate);
        };

        let mut started = false;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    if !started {
                        let _ = on_event.send(DownloadEvent::Started { content_length });
                        started = true;
                    }

                    let _ = on_event.send(DownloadEvent::Progress { chunk_length });
                },
                || {
                    let _ = on_event.send(DownloadEvent::Finished);
                },
            )
            .await?;

        Ok(())
    }

    struct PendingUpdate(Mutex<Option<Update>>);
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_process::init())
//         .setup(|app| {
//             #[cfg(desktop)]
//             {
//                 app.handle().plugin(tauri_plugin_updater::Builder::new().build());
//                 app.manage(app_updates::PendingUpdate(Mutex::new(None)));
//             }
//             Ok(())
//         })
//         .invoke_handler(tauri::generate_handler![
//             #[cfg(desktop)]
//             app_updates::fetch_update,
//             #[cfg(desktop)]
//             app_updates::install_update
//         ])
// }
