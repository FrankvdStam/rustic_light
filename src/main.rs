#[macro_use]
extern crate windows_service;

use std::ffi::OsString;
use windows_service::{service_dispatcher, service_control_handler};

define_windows_service!(ffi_service_main, service_main);

mod color;
mod z390;
mod rtx2080;
mod animation;

use rtx2080::Rtx2080;
use crate::color::RgbDevice;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use windows_service::service_control_handler::ServiceControlHandlerResult;
use windows_service::service::{ServiceControl, ServiceStatus, ServiceType, ServiceState, ServiceControlAccept, ServiceExitCode};


fn main() -> Result<(), windows_service::Error> {
    // Register generated `ffi_service_main` with the system and start the service, blocking
    // this thread until the service is stopped.
    service_dispatcher::start("rustic light", ffi_service_main)?;
    Ok(())
}

fn service_main(arguments: Vec<OsString>)
{
    run_service(arguments).unwrap();
}

static mut RUNNING: bool = true;


fn run_service(_arguments: Vec<OsString>) -> Result<(), windows_service::Error> {

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop =>
            {
                unsafe { RUNNING = false; }
                // Handle stop event and return control back to the system.
                ServiceControlHandlerResult::NoError
            }
            // All services must accept Interrogate even if it's a no-op.
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler
    let status_handle = service_control_handler::register("rustic light", event_handler)?;

    let next_status = ServiceStatus {
        // Should match the one from system service registry
        service_type: ServiceType::OWN_PROCESS,
        // The new state
        current_state: ServiceState::Running,
        // Accept stop events when running
        controls_accepted: ServiceControlAccept::STOP,
        // Used to report an error when starting or stopping only, otherwise must be zero
        exit_code: ServiceExitCode::Win32(0),
        // Only used for pending states, otherwise must be zero
        checkpoint: 0,
        // Only used for pending states, otherwise must be zero
        wait_hint: Duration::default(),

        process_id: None
    };

    // Tell the system that the service is running now
    status_handle.set_service_status(next_status.clone())?;

    run_animation();

    let next_status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None
    };
    status_handle.set_service_status(next_status)?;

    #[allow(unreachable_code)]
    Ok(())
}


fn run_animation()
{
    let start = SystemTime::now();

    let mut rgb_devices: Vec<Box<dyn RgbDevice>> = z390::get_z390_rgb_devices();
    rgb_devices.push(Box::new(Rtx2080::new()));

    loop
    {
        let millis = SystemTime::now().duration_since(start).unwrap().as_millis();
        animation::color_spectrum(&mut rgb_devices, millis);
        for d in rgb_devices.iter_mut()
        {
            d.display();
        }

        sleep(Duration::from_millis(100));

        unsafe
        {
            if !RUNNING
            {
                return;
            }
        }
    }
}