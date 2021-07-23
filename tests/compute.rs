use anyhow::Result;
use std::collections::HashMap;
use xenon::compute::{Job, JobDescription, JobErrorType, QueueErrorType, QueueStatus, Scheduler};
use xenon::credentials::Credential;

pub async fn create_slurm_scheduler() -> Result<Scheduler> {
    let credential = Credential::new_password(String::from("xenon"), String::from("javagat"));
    let mut properties = HashMap::new();
    properties.insert(
        String::from("xenon.adaptors.schedulers.ssh.strictHostKeyChecking"),
        String::from("false"),
    );

    let scheduler = Scheduler::create(
        String::from("slurm"),
        String::from("ssh://slurm:22"),
        credential,
        "http://localhost:50051",
        Some(properties),
    )
    .await?;

    Ok(scheduler)
}

#[tokio::test]
async fn canceljob_existing_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    let job_description = JobDescription {
        arguments: Some(vec![String::from("120")]),
        executable: Some(String::from("sleep")),
        ..Default::default()
    };

    let job = scheduler.submit_batch_job(job_description).await;
    assert!(job.is_ok());

    let job_status = scheduler.cancel_job(job.unwrap()).await;
    assert!(job_status.is_ok());
    let job_status = job_status.unwrap();

    assert!(job_status.done);
    assert_eq!(job_status.error_type, JobErrorType::Cancelled);

    Ok(())
}

#[tokio::test]
async fn canceljob_nonexisting_err() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    let job = Job::new(String::from("non-existing"));

    let result = scheduler.cancel_job(job).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn create_default_ok() -> Result<()> {
    let scheduler = create_slurm_scheduler().await;
    assert!(scheduler.is_ok());

    Ok(())
}

#[tokio::test]
async fn getdefaultqueuename_default_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_default_queue_name().await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "mypartition");

    Ok(())
}

#[tokio::test]
async fn getjobstatus_existing_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let job_description = JobDescription {
        arguments: Some(vec![String::from("3")]),
        executable: Some(String::from("sleep")),
        ..Default::default()
    };

    let job = scheduler.submit_batch_job(job_description).await;
    assert!(job.is_ok());
    let job = job.unwrap();

    let job_status = scheduler.get_job_status(job.clone()).await;
    assert!(job_status.is_ok());
    let job_status = job_status.unwrap();

    assert!(!job_status.done);
    assert_eq!(job_status.error_type, JobErrorType::None);

    scheduler.wait_until_done(job, None).await?;

    Ok(())
}

#[tokio::test]
async fn getjobstatus_nonexisting_err() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    let job = Job::new(String::from("non-existing"));

    let result = scheduler.get_job_status(job).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn getjobstatuses_none_empty() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_job_statuses(vec![]).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    let job_status = result.first();
    assert!(job_status.is_none());

    Ok(())
}

#[tokio::test]
async fn getjobstatuses_existing_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let job_description = JobDescription {
        arguments: Some(vec![String::from("3")]),
        executable: Some(String::from("sleep")),
        ..Default::default()
    };

    let job = scheduler.submit_batch_job(job_description).await;
    assert!(job.is_ok());
    let job = job.unwrap();

    let result = scheduler.get_job_statuses(vec![job.clone()]).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    let job_status = result.first();
    assert!(job_status.is_some());
    let job_status = job_status.unwrap();
    assert_eq!(job_status.error_type, JobErrorType::None);

    scheduler.wait_until_done(job, None).await?;

    Ok(())
}

#[tokio::test]
async fn getjobstatuses_nonexisting_err() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    let job = Job::new(String::from("non-existing"));

    let result = scheduler.get_job_statuses(vec![job]).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    let job_status = result.first();
    assert!(job_status.is_some());
    let job_status = job_status.unwrap();
    assert_eq!(job_status.error_type, JobErrorType::NotFound);

    Ok(())
}

#[tokio::test]
async fn getjobs_none_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_jobs(None).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn getjobs_existing_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_jobs(Some(vec![String::from("mypartition")])).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn getjobs_nonexisting_err() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_jobs(Some(vec![String::from(";)")])).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn getproperties_default_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_properties().await;

    let mut expected = HashMap::<String, String>::new();
    expected.insert(
        String::from("xenon.adaptors.schedulers.ssh.strictHostKeyChecking"),
        String::from("false"),
    );

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);

    Ok(())
}

#[tokio::test]
async fn getqueuenames_default_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_queue_names().await;

    let expected = vec![String::from("mypartition"), String::from("otherpartition")];

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);

    Ok(())
}

#[tokio::test]
async fn getqueuestatus_existing_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_queue_status(String::from("mypartition")).await;

    let expected = QueueStatus::new(String::from("mypartition"), String::new(), QueueErrorType::None);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);

    Ok(())
}

#[tokio::test]
async fn getqueuestatus_nonexisting_err() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_queue_status(String::from(";)")).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn getqueuestatuses_none_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_queue_statuses(None).await;

    let expected = vec![
        QueueStatus::new(String::from("mypartition"), String::new(), QueueErrorType::None),
        QueueStatus::new(String::from("otherpartition"), String::new(), QueueErrorType::None),
    ];

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);

    Ok(())
}

#[tokio::test]
async fn getqueuestatuses_existing_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler
        .get_queue_statuses(Some(vec![String::from("mypartition")]))
        .await;

    let expected = vec![QueueStatus::new(
        String::from("mypartition"),
        String::new(),
        QueueErrorType::None,
    )];

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);

    Ok(())
}

#[tokio::test]
async fn getqueuestatuses_nonexisting_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.get_queue_statuses(Some(vec![String::from(";)")])).await;

    let expected = vec![QueueStatus::new(
        String::from(";)"),
        String::from("slurm adaptor: Cannot get status of queue \";)\" from server, perhaps it does not exist?"),
        QueueErrorType::NotFound,
    )];

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);

    Ok(())
}

#[tokio::test]
async fn isopen_open_true() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;

    let result = scheduler.is_open().await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);

    Ok(())
}

#[tokio::test]
async fn isopen_closed_false() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    scheduler.close().await?;

    let result = scheduler.is_open().await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);

    Ok(())
}

#[tokio::test]
async fn submitbatchjob_valid_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    let mut environment = HashMap::new();
    environment.insert(String::from("NAME"), String::from("Xenon!"));

    let job_description = JobDescription {
        arguments: Some(vec![String::from("-c"), String::from("echo"), String::from("$NAME")]),
        executable: Some(String::from("bash")),
        environment: Some(environment),
        ..Default::default()
    };

    let result = scheduler.submit_batch_job(job_description).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn waituntildone_existing_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    let mut environment = HashMap::new();
    environment.insert(String::from("NAME"), String::from("Xenon!"));

    let job_description = JobDescription {
        arguments: Some(vec![String::from("-c"), String::from("echo"), String::from("$NAME")]),
        executable: Some(String::from("bash")),
        environment: Some(environment),
        ..Default::default()
    };

    let job = scheduler.submit_batch_job(job_description).await;
    assert!(job.is_ok());

    let job_status = scheduler.wait_until_done(job.unwrap(), None).await;
    assert!(job_status.is_ok());
    let job_status = job_status.unwrap();
    assert!(job_status.done);
    assert_eq!(job_status.error_type, JobErrorType::None);

    Ok(())
}

#[tokio::test]
async fn waituntildone_nonexisting_err() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    let job = Job::new(String::from("non-existing"));

    let job_status = scheduler.wait_until_done(job, None).await;

    assert!(job_status.is_err());

    Ok(())
}

#[tokio::test]
async fn waituntilrunning_existing_ok() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    let job_description = JobDescription {
        arguments: Some(vec![String::from("10")]),
        executable: Some(String::from("sleep")),
        ..Default::default()
    };

    let job = scheduler.submit_batch_job(job_description).await;
    assert!(job.is_ok());

    let job_status = scheduler.wait_until_running(job.unwrap(), None).await;
    assert!(job_status.is_ok());
    let job_status = job_status.unwrap();
    assert!(!job_status.done);
    assert_eq!(job_status.state, String::from("RUNNING"));

    Ok(())
}

#[tokio::test]
async fn waituntilrunning_nonexisting_err() -> Result<()> {
    let mut scheduler = create_slurm_scheduler().await?;
    let job = Job::new(String::from("non-existing"));

    let job_status = scheduler.wait_until_running(job, None).await;

    assert!(job_status.is_err());

    Ok(())
}
