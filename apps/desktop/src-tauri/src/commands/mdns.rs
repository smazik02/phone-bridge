use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use tokio_util::sync::CancellationToken;

pub struct ScanState(pub Mutex<Option<(CancellationToken, ServiceDaemon)>>);

#[tauri::command]
pub async fn start_mdns_scan(app: AppHandle, state: State<'_, ScanState>) -> Result<(), String> {
    let mut lock = state.0.lock().expect("Failed to lock state");

    if let Some((old_token, _)) = lock.take() {
        old_token.cancel();
    }

    let token = CancellationToken::new();
    let cloned_token = token.clone();

    let mdns = ServiceDaemon::new().map_err(|e| format!("Failed to create daemon: {:?}", e))?;

    let service_type = "_mdns-sd-my-test._udp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");

    *lock = Some((token, mdns));

    tauri::async_runtime::spawn(async move {
        log::info!("Starting mdns scan...");

        tokio::select! {
            _ = cloned_token.cancelled() => {
                log::info!("mdns scan stopped");
            }
            _ = async {
                while let Ok(event) = receiver.recv_async().await {
                    match event {
                        ServiceEvent::ServiceResolved(resolved) => {
                            let name = resolved.get_fullname();
                            log::info!("Resolved new service: {}", name);

                            if let Err(err) = app.emit("device-found", name) {
                                log::error!("Failed to emit device-found: {}", err);
                            }
                            break;
                        }
                        other_event => {
                            log::info!("Other event: {:?}", &other_event);
                        }
                    }
                }
                log::info!("mdns scan ended");
            } => {}
        }
    });

    Ok(())
}
