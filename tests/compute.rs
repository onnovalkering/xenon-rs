mod common;
use xenon_rs::compute::{QueueErrorType, QueueStatus};

type Map<T> = std::collections::HashMap<String, T>;

// #[test]
fn canceljob_existing_ok() {
    unimplemented!();
}

// #[test]
fn canceljob_nonexisting_err() {
    unimplemented!();
}

#[test]
fn create_default_ok() {
    let scheduler = common::create_slurm_scheduler();

    assert!(scheduler.is_ok());
}

#[test]
fn getdefaultqueuename_default_ok() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_default_queue_name();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "mypartition");
}

// #[test]
fn getjobstatus_existing_ok() {
    unimplemented!();
}

// #[test]
fn getjobstatus_nonexisting_err() {
    unimplemented!();
}

// #[test]
fn getjobstatuses_none_ok() {
    unimplemented!();
}

// #[test]
fn getjobstatuses_existing_ok() {
    unimplemented!();
}

// #[test]
fn getjobstatuses_nonexisting_err() {
    unimplemented!();
}

#[test]
fn getjobs_none_ok() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_jobs(None);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec!());
}

#[test]
fn getjobs_existing_ok() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_jobs(Some(vec![String::from("mypartition")]));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec!());
}

#[test]
fn getjobs_nonexisting_err() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_jobs(Some(vec![String::from(";)")]));

    assert!(result.is_err());
}

#[test]
fn getproperties_default_ok() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_properties();

    let mut expected = Map::<String>::new();
    expected.insert(
        String::from("xenon.adaptors.schedulers.ssh.strictHostKeyChecking"),
        String::from("false"),
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn getqueuenames_default_ok() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_queue_names();

    let expected = vec![String::from("mypartition"), String::from("otherpartition")];

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn getqueuestatus_existing_ok() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_queue_status(String::from("mypartition"));

    let expected = QueueStatus::new(String::from("mypartition"), String::new(), QueueErrorType::None);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn getqueuestatus_nonexisting_err() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_queue_status(String::from(";)"));

    assert!(result.is_err());
}

#[test]
fn getqueuestatuses_none_ok() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_queue_statuses(None);

    let expected = vec![
        QueueStatus::new(String::from("mypartition"), String::new(), QueueErrorType::None),
        QueueStatus::new(String::from("otherpartition"), String::new(), QueueErrorType::None),
    ];

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn getqueuestatuses_existing_ok() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_queue_statuses(Some(vec![String::from("mypartition")]));

    let expected = vec![QueueStatus::new(
        String::from("mypartition"),
        String::new(),
        QueueErrorType::None,
    )];

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn getqueuestatuses_nonexisting_ok() {
    let scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.get_queue_statuses(Some(vec![String::from(";)")]));

    let expected = vec![QueueStatus::new(
        String::from(";)"),
        String::from("slurm adaptor: Cannot get status of queue \";)\" from server, perhaps it does not exist?"),
        QueueErrorType::NotFound,
    )];

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}

#[test]
fn isopen_open_true() {
    let mut scheduler = common::create_slurm_scheduler().unwrap();

    let result = scheduler.is_open();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
fn isopen_closed_false() {
    let mut scheduler = common::create_slurm_scheduler().unwrap();
    scheduler.close().unwrap();

    let result = scheduler.is_open();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

// #[test]
fn submitbatchjob_valid_ok() {
    unimplemented!();
}

// #[test]
fn submitbatchjob_invalid_err() {
    unimplemented!();
}

// #[test]
fn waituntildone_existing_ok() {
    unimplemented!();
}

// #[test]
fn waituntildone_nonexisting_err() {
    unimplemented!();
}

// #[test]
fn waituntilrunning_existing_ok() {
    unimplemented!();
}

// #[test]
fn waituntilrunning_nonexisting_err() {
    unimplemented!();
}
