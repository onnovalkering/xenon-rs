use crate::credentials::Credential;
use crate::storage::FileSystem;
use crate::xenon::{self as x, scheduler_service_client::SchedulerServiceClient};
use anyhow::Result;
use std::collections::HashMap;
use tonic::transport::Channel;

type Map<T> = std::collections::HashMap<String, T>;

///
///
///
pub struct Scheduler {
    pub adaptor: String,
    client: SchedulerServiceClient<Channel>,
    xenon_endpoint: String,
    pub(crate) scheduler: x::Scheduler,
    pub identifier: String,
}

impl Scheduler {
    ///
    ///
    ///
    pub async fn cancel_job(
        &mut self,
        job: Job,
    ) -> Result<JobStatus> {
        let request = x::JobRequest {
            job: Some(job.proto()),
            scheduler: Some(self.scheduler.clone()),
        };

        let response = self.client.cancel_job(request).await?;
        let response = response.into_inner();

        Ok(JobStatus::from(response))
    }

    ///
    ///
    ///
    pub async fn close(&mut self) -> Result<()> {
        if self.is_open().await? {
            self.client.close(self.scheduler.clone()).await?;
        }

        Ok(())
    }

    ///
    ///
    ///
    pub async fn create<S1, S2, S3>(
        adaptor: S1,
        location: S2,
        credential: Credential,
        xenon_endpoint: S3,
        properties: Option<HashMap<String, String>>,
    ) -> Result<Self>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        let adaptor = adaptor.into();
        let xenon_endpoint = xenon_endpoint.into();

        let credential = match credential {
            Credential::Password(password) => {
                x::create_scheduler_request::Credential::PasswordCredential(password.proto())
            }
            Credential::Certificate(certificate) => {
                x::create_scheduler_request::Credential::CertificateCredential(certificate.proto())
            }
        };

        // Construct create request message.
        let request = x::CreateSchedulerRequest {
            adaptor: adaptor.clone(),
            location: location.into(),
            properties: properties.unwrap_or_default(),
            credential: Some(credential),
        };

        let mut client = SchedulerServiceClient::connect(xenon_endpoint.clone()).await?;
        let scheduler = client.create(request).await?.into_inner();
        let identifier = scheduler.id.clone();

        Ok(Scheduler {
            adaptor,
            scheduler,
            identifier,
            client,
            xenon_endpoint,
        })
    }

    ///
    ///
    ///
    pub async fn create_local<S1>(xenon_endpoint: S1) -> Result<Self>
    where
        S1: Into<String>,
    {
        let xenon_endpoint = xenon_endpoint.into();
        let mut client = SchedulerServiceClient::connect(xenon_endpoint.clone()).await?;

        let response = client.local_scheduler(x::Empty {}).await?;
        let response = response.into_inner();

        Self::restore(response.id, xenon_endpoint).await
    }

    ///
    ///
    ///
    pub async fn get_credential(&mut self) -> Result<Option<Credential>> {
        let response = self.client.get_credential(self.scheduler.clone()).await?;
        let response = response.into_inner();

        if let Some(credential) = response.credential {
            use x::get_credential_response::Credential::*;

            match credential {
                CertificateCredential(credential) => Ok(Some(Credential::new_certificate(
                    credential.certfile,
                    credential.username,
                    credential.passphrase,
                ))),
                PasswordCredential(credential) => {
                    Ok(Some(Credential::new_password(credential.username, credential.password)))
                }
                _ => unreachable!(),
            }
        } else {
            Ok(None)
        }
    }

    ///
    ///
    ///
    pub async fn get_default_queue_name(&mut self) -> Result<String> {
        let response = self.client.get_default_queue_name(self.scheduler.clone()).await?;
        let response = response.into_inner();

        Ok(response.name)
    }

    ///
    ///
    ///
    pub async fn get_default_runtime(&mut self) -> Result<u32> {
        let response = self.client.get_default_runtime(self.scheduler.clone()).await?;
        let response = response.into_inner();

        Ok(response.value)
    }

    ///
    ///
    ///
    pub async fn get_filesystem(&mut self) -> Result<FileSystem> {
        let response = self.client.get_file_system(self.scheduler.clone()).await?;
        let response = response.into_inner();

        FileSystem::restore(response.id, self.xenon_endpoint.clone()).await
    }

    ///
    ///
    ///
    pub async fn get_job_status(
        &mut self,
        job: Job,
    ) -> Result<JobStatus> {
        let request = x::JobRequest {
            job: Some(job.proto()),
            scheduler: Some(self.scheduler.clone()),
        };

        let response = self.client.get_job_status(request).await?;
        let response = response.into_inner();

        Ok(JobStatus::from(response))
    }

    ///
    ///
    ///
    pub async fn get_job_statuses(
        &mut self,
        jobs: Vec<Job>,
    ) -> Result<Vec<JobStatus>> {
        let jobs = jobs.iter().map(|j| j.clone().proto()).collect();

        let request = x::GetJobStatusesRequest {
            jobs,
            scheduler: Some(self.scheduler.clone()),
        };

        let response = self.client.get_job_statuses(request).await?;
        let response = response.into_inner();

        let statuses = response.statuses.iter().map(|j| JobStatus::from(j.clone())).collect();
        Ok(statuses)
    }

    ///
    ///
    ///
    pub async fn get_jobs(
        &mut self,
        queues: Option<Vec<String>>,
    ) -> Result<Vec<Job>> {
        let request = x::SchedulerAndQueues {
            queues: queues.unwrap_or_default(),
            scheduler: Some(self.scheduler.clone()),
        };

        let response = self.client.get_jobs(request).await?;
        let response = response.into_inner();

        let jobs = response.jobs.iter().map(|j| Job::new(j.id.clone())).collect();
        Ok(jobs)
    }

    ///
    ///
    ///
    pub async fn get_location(&mut self) -> Result<String> {
        let response = self.client.get_location(self.scheduler.clone()).await?;
        let response = response.into_inner();

        Ok(response.location)
    }

    ///
    ///
    ///
    pub async fn get_properties(&mut self) -> Result<Map<String>> {
        let response = self.client.get_properties(self.scheduler.clone()).await?;
        let response = response.into_inner();

        Ok(response.properties)
    }

    ///
    ///
    ///
    pub async fn get_queue_names(&mut self) -> Result<Vec<String>> {
        let response = self.client.get_queue_names(self.scheduler.clone()).await?;
        let response = response.into_inner();

        Ok(response.name)
    }

    ///
    ///
    ///
    pub async fn get_queue_status<S: Into<String>>(
        &mut self,
        queue: S,
    ) -> Result<QueueStatus> {
        let request = x::GetQueueStatusRequest {
            queue: queue.into(),
            scheduler: Some(self.scheduler.clone()),
        };

        let response = self.client.get_queue_status(request).await?;
        let response = response.into_inner();

        Ok(QueueStatus::from(response))
    }

    ///
    ///
    ///
    pub async fn get_queue_statuses(
        &mut self,
        queues: Option<Vec<String>>,
    ) -> Result<Vec<QueueStatus>> {
        let request = x::SchedulerAndQueues {
            queues: queues.unwrap_or_default(),
            scheduler: Some(self.scheduler.clone()),
        };

        let response = self.client.get_queue_statuses(request).await?;
        let response = response.into_inner();

        let statuses = response.statuses.iter().map(|s| QueueStatus::from(s.clone())).collect();
        Ok(statuses)
    }

    ///
    ///
    ///
    pub async fn list_schedulers<S1>(xenon_endpoint: S1) -> Result<Vec<String>>
    where
        S1: Into<String>,
    {
        let xenon_endpoint = xenon_endpoint.into();
        let mut client = SchedulerServiceClient::connect(xenon_endpoint.clone()).await?;

        let response = client.list_schedulers(x::Empty {}).await?;
        let response = response.into_inner();

        let identifiers = response.schedulers.into_iter().map(|f| f.id).collect();

        Ok(identifiers)
    }

    ///
    ///
    ///
    pub async fn is_open(&mut self) -> Result<bool> {
        let response = self.client.is_open(self.scheduler.clone()).await;
        let value = if let Ok(response) = response {
            let response = response.into_inner();
            response.value
        } else {
            false
        };

        Ok(value)
    }

    ///
    ///
    ///
    pub async fn restore<S1, S2>(
        identifier: S1,
        xenon_endpoint: S2,
    ) -> Result<Self>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let xenon_endpoint = xenon_endpoint.into();

        let mut client = SchedulerServiceClient::connect(xenon_endpoint.clone()).await?;
        let scheduler = x::Scheduler { id: identifier.into() };

        // Check if identifier corresponds to an existing and open scheduler.
        let exists_and_open = client.is_open(scheduler.clone()).await?.into_inner().value;
        if !exists_and_open {
            bail!("Identifier '{}' doesn't correspond to an existing and/or open scheduler.");
        }

        let adaptor = client.get_adaptor_name(scheduler.clone()).await?.into_inner().name;
        let identifier = scheduler.id.clone();

        Ok(Scheduler {
            adaptor,
            scheduler,
            identifier,
            client,
            xenon_endpoint,
        })
    }

    ///
    ///
    ///
    pub async fn submit_batch_job(
        &mut self,
        description: JobDescription,
    ) -> Result<Job> {
        let request = x::SubmitBatchJobRequest {
            description: Some(description.proto()),
            scheduler: Some(self.scheduler.clone()),
        };

        let response = self.client.submit_batch_job(request).await?;
        let response = response.into_inner();

        Ok(Job::new(response.id))
    }

    ///
    ///
    ///
    pub async fn wait_until_done(
        &mut self,
        job: Job,
        timeout: Option<u64>,
    ) -> Result<JobStatus> {
        let request = x::WaitRequest {
            job: Some(job.proto()),
            timeout: timeout.unwrap_or_default(),
            scheduler: Some(self.scheduler.clone()),
        };

        let response = self.client.wait_until_done(request).await?;
        let response = response.into_inner();

        Ok(JobStatus::from(response))
    }

    ///
    ///
    ///
    pub async fn wait_until_running(
        &mut self,
        job: Job,
        timeout: Option<u64>,
    ) -> Result<JobStatus> {
        let request = x::WaitRequest {
            job: Some(job.proto()),
            timeout: timeout.unwrap_or_default(),
            scheduler: Some(self.scheduler.clone()),
        };

        let response = self.client.wait_until_running(request).await?;
        let response = response.into_inner();

        Ok(JobStatus::from(response))
    }
}

///
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct Job {
    pub id: String,
}

impl Job {
    ///
    ///
    ///
    pub(crate) fn from(job: Option<x::Job>) -> Option<Job> {
        if let Some(job) = job {
            Some(Job::new(job.id))
        } else {
            None
        }
    }

    ///
    ///
    ///
    pub fn new<S: Into<String>>(id: S) -> Job {
        Job { id: id.into() }
    }

    ///
    ///
    ///
    pub(crate) fn proto(self) -> x::Job {
        x::Job { id: self.id }
    }
}

///
///
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct JobDescription {
    pub name: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub executable: Option<String>,
    pub working_directory: Option<String>,
    pub environment: Option<Map<String>>,
    pub queue: Option<String>,
    pub max_runtime: Option<u32>,
    pub stderr: Option<String>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub max_memory: Option<u32>,
    pub scheduler_arguments: Option<Vec<String>>,
    pub tasks: Option<u32>,
    pub cores_per_tasks: Option<u32>,
    pub tasks_per_node: Option<u32>,
    pub start_per_task: Option<bool>,
    pub start_time: Option<String>,
    pub temp_space: Option<u32>,
}

impl JobDescription {
    ///
    ///
    ///
    pub(crate) fn proto(self) -> x::JobDescription {
        x::JobDescription {
            name: self.name.unwrap_or_default(),
            arguments: self.arguments.unwrap_or_default(),
            executable: self.executable.unwrap_or_default(),
            working_directory: self.working_directory.unwrap_or_default(),
            environment: self.environment.unwrap_or_default(),
            queue_name: self.queue.unwrap_or_default(),
            max_runtime: self.max_runtime.unwrap_or_default(),
            stderr: self.stderr.unwrap_or_default(),
            stdin: self.stdin.unwrap_or_default(),
            stdout: self.stdout.unwrap_or_default(),
            max_memory: self.max_memory.unwrap_or_default(),
            scheduler_arguments: self.scheduler_arguments.unwrap_or_default(),
            tasks: self.tasks.unwrap_or_default(),
            cores_per_task: self.cores_per_tasks.unwrap_or_default(),
            tasks_per_node: self.tasks_per_node.unwrap_or_default(),
            start_per_task: self.start_per_task.unwrap_or_default(),
            start_time: self.start_time.unwrap_or_default(),
            temp_space: self.temp_space.unwrap_or_default(),
        }
    }
}

///
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct JobStatus {
    pub done: bool,
    pub exit_code: i32,
    pub error_message: String,
    pub error_type: JobErrorType,
    pub job: Option<Job>,
    pub name: String,
    pub running: bool,
    pub state: String,
}

impl JobStatus {
    ///
    ///
    ///
    pub(crate) fn from(status: x::JobStatus) -> JobStatus {
        let error_type = x::job_status::ErrorType::from_i32(status.error_type);
        let error_type = JobErrorType::from(error_type.unwrap_or_default());

        JobStatus {
            done: status.done,
            exit_code: status.exit_code,
            error_message: status.error_message,
            error_type,
            job: Job::from(status.job),
            name: status.name,
            running: status.running,
            state: status.state,
        }
    }
}

///
///
///
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum JobErrorType {
    None = 0,
    NotFound = 1,
    Cancelled = 2,
    NotConnected = 3,
    Xenon = 4,
    InputOutput = 5,
    Other = 6,
}

impl JobErrorType {
    ///
    ///
    ///
    pub(crate) fn from(error_type: x::job_status::ErrorType) -> JobErrorType {
        use x::job_status::ErrorType::*;

        match error_type {
            None => JobErrorType::None,
            NotFound => JobErrorType::NotFound,
            Cancelled => JobErrorType::Cancelled,
            NotConnected => JobErrorType::NotConnected,
            Xenon => JobErrorType::Xenon,
            Io => JobErrorType::InputOutput,
            Other => JobErrorType::Other,
        }
    }
}

///
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct QueueStatus {
    name: String,
    error_message: String,
    error_type: QueueErrorType,
}

impl QueueStatus {
    ///
    ///
    ///
    pub(crate) fn from(status: x::QueueStatus) -> QueueStatus {
        let error_type = x::queue_status::ErrorType::from_i32(status.error_type);
        let error_type = QueueErrorType::from(error_type.unwrap_or_default());

        QueueStatus::new(status.name, status.error_message, error_type)
    }

    ///
    ///
    ///
    pub fn new(
        name: String,
        error_message: String,
        error_type: QueueErrorType,
    ) -> QueueStatus {
        QueueStatus {
            name,
            error_message,
            error_type,
        }
    }
}

///
///
///
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum QueueErrorType {
    None = 0,
    NotFound = 1,
    NotConnected = 2,
    Xenon = 3,
    InputOutput = 4,
    Other = 5,
}

impl QueueErrorType {
    ///
    ///
    ///
    pub(crate) fn from(error_type: x::queue_status::ErrorType) -> QueueErrorType {
        use x::queue_status::ErrorType::*;

        match error_type {
            None => QueueErrorType::None,
            NotFound => QueueErrorType::NotFound,
            NotConnected => QueueErrorType::NotConnected,
            Xenon => QueueErrorType::Xenon,
            Io => QueueErrorType::InputOutput,
            Other => QueueErrorType::Other,
        }
    }
}
