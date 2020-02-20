use crate::credentials::{CertificateCredential, Credential, PasswordCredential};
use crate::storage::FileSystem;
use crate::xenon;
use crate::xenon_grpc::SchedulerServiceClient;
use futures::{future, Future, Sink, Stream};
use grpcio::{Channel, WriteFlags};
use std::collections::HashSet;
use protobuf::RepeatedField;

type FResult<T> = Result<T, failure::Error>;
type Map<T> = std::collections::HashMap<String, T>;

///
/// 
///
pub struct Scheduler {
    pub adaptor: String,
    client: SchedulerServiceClient,
    open: bool,
    pub(crate) scheduler: xenon::Scheduler,
    pub identifier: String,
}

impl Scheduler {
    ///
    /// 
    /// 
    pub fn cancel_job() -> FResult<()> {
        unimplemented!();
    }

    ///
    ///
    ///
    pub fn close(&mut self) -> FResult<()> {
        if self.open {
            self.client.close(&self.scheduler)?;
            self.open = false;
        }

        Ok(())
    }

    ///
    /// 
    /// 
    pub fn create(
        adaptor: String,
        channel: Channel,
        credential: Credential,
        location: String,
        properties: Map<String>,
    ) -> FResult<Scheduler> {
        let client = SchedulerServiceClient::new(channel);

        // Construct create request message.
        let mut request = xenon::CreateSchedulerRequest::new();
        request.set_adaptor(adaptor.clone());
        request.set_location(location.clone());
        request.set_properties(properties);
        match credential {
            Credential::Password(password) => request.set_password_credential(password.proto()),
            Credential::Certificate(certificate) => request.set_certificate_credential(certificate.proto()),
        }

        let scheduler = client.create(&request)?;
        let identifier = scheduler.id.clone();

        Ok(Scheduler {
            adaptor,
            scheduler,
            open: true,
            identifier,
            client
        })
    }

    ///
    /// 
    /// 
    pub fn get_default_queue_name(
        &self
    ) -> FResult<String> {
        let response = self.client.get_default_queue_name(&self.scheduler)?;

        Ok(response.name)
    }

    ///
    /// 
    /// 
    pub fn get_job_status(
        &self,
        job: Job, 
    ) -> FResult<Option<JobStatus>> {
        let mut request = xenon::JobRequest::new();
        request.set_job(job.proto());
        request.set_scheduler(self.scheduler.clone());

        let response = self.client.get_job_status(&request)?;

        Ok(JobStatus::from(response))
    }

    ///
    /// 
    /// 
    pub fn get_job_statuses(
        &self,
        jobs: Vec<Job>,
    ) -> FResult<Vec<Option<JobStatus>>> {
        let jobs = jobs.iter().map(|j| j.clone().proto()).collect();

        let mut request = xenon::GetJobStatusesRequest::new();
        request.set_jobs(RepeatedField::from_vec(jobs));
        request.set_scheduler(self.scheduler.clone());

        let response = self.client.get_job_statuses(&request)?;
        Ok(response.statuses.iter().map(|j| JobStatus::from(j.clone())).collect())
    }

    ///
    /// 
    /// 
    pub fn get_jobs(
        &self,
        queues: Option<Vec<String>>
    ) -> FResult<Vec<Job>> {
        let mut request = xenon::SchedulerAndQueues::new();
        request.set_scheduler(self.scheduler.clone());
        if let Some(queues) = queues {
            request.set_queues(RepeatedField::from_vec(queues));
        }

        let response = self.client.get_jobs(&request)?;
        Ok(response.jobs.iter().map(|j| Job::new(j.id.clone())).collect())
    }

    ///
    ///
    ///
    pub fn get_properties(
        &self
    ) -> FResult<Map<String>> {
        let response = self.client.get_properties(&self.scheduler)?;

        Ok(response.properties)
    }

    ///
    /// 
    /// 
    pub fn get_queue_names(
        &self
    ) -> FResult<Vec<String>> {
        let response = self.client.get_queue_names(&self.scheduler)?;

        Ok(response.name.into_vec())
    }

    ///
    /// 
    /// 
    pub fn get_queue_status(
        &self,
        queue: Option<String>,
    ) -> FResult<()> {
        unimplemented!();
    }

    ///
    /// 
    /// 
    pub fn get_queue_statuses(
        &self,
        queues: Option<Vec<String>>,
    ) -> FResult<()> {
        unimplemented!();
    }

    ///
    /// 
    /// 
    pub fn is_open(
        &self
    ) -> FResult<bool> {
        let response = self.client.is_open(&self.scheduler)?;

        Ok(response.value)
    }

    ///
    /// 
    /// 
    pub fn submit_batch_job(
        &self,
        description: JobDescription
    ) -> FResult<()> {
        unimplemented!();
    }

    ///
    /// 
    /// 
    pub fn wait_until_done(
        &self,
        job: Job,
        timeout: u64,
    ) -> FResult<()> {
        unimplemented!();
    }

    ///
    /// 
    /// 
    pub fn wait_until_running(
        &self,
        job: Job,
        timeout: u64,
    ) -> FResult<()> {
        unimplemented!();
    }
}

impl Drop for Scheduler {
    ///
    ///
    ///
    fn drop(&mut self) {
        self.close().unwrap();
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
    pub(crate) fn from(job: protobuf::SingularPtrField<xenon::Job>) -> Option<Job> {
        if let Some(job) = job.into_option() {
            Some(Job::new(job.id))
        } else {
            None
        }
    }

    ///
    ///
    ///
    pub fn new(id: String) -> Job {
        Job { id }
    }

    ///
    ///
    ///
    pub(crate) fn proto(self) -> xenon::Job {
        let mut job = xenon::Job::new();
        job.set_id(self.id);

        job
    }
}

///
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct JobDescription {
    pub arguments: Vec<String>,
    pub executable: String,
    pub working_directory: String,
    pub environment: Map<String>,
    pub queue: String,
    pub max_runtime: u32,
    pub stderr: String,
    pub stdin: String,
    pub stdout: String,
    pub max_memory: u32,
    pub scheduler_arguments: Vec<String>,
    pub tasks: u32,
    pub cores_per_tasak: u32,
    pub tasks_per_node: u32,
    pub start_per_task: bool,
    pub start_time: String,
    pub temp_space: u32,
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
    pub job: Job,
    pub name: String,
    pub running: bool,
    pub state: String,
}

impl JobStatus {
    ///
    /// 
    /// 
    pub(crate) fn from(status: xenon::JobStatus) -> Option<JobStatus> {
        if let Some(job) = Job::from(status.job) {
            let error_type = JobErrorType::from(&status.error_type);

            Some(JobStatus::new(
                job,
                status.done,
                status.exit_code,
                status.error_message,
                error_type,
                status.name,
                status.running,
                status.state,
            ))
        } else {
            None
        }
    }

    ///
    ///
    ///
    pub fn new(
        job: Job,
        done: bool,
        exit_code: i32,
        error_message: String,
        error_type: JobErrorType,
        name: String,
        running: bool,
        state: String,
    ) -> JobStatus {
        JobStatus { job, done, exit_code, error_message, error_type, name, running, state }
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
    pub(crate) fn from(error_type: &xenon::JobStatus_ErrorType) -> JobErrorType {
        use xenon::JobStatus_ErrorType::*;
        use JobErrorType::*;

        match error_type {
            NONE => None,
            NOT_FOUND => NotFound,
            CANCELLED => Cancelled,
            NOT_CONNECTED => NotConnected,
            XENON => Xenon,
            IO => InputOutput,
            OTHER => Other,
        }
    }

    ///
    ///
    ///
    pub fn proto(&self) -> xenon::JobStatus_ErrorType {
        use xenon::JobStatus_ErrorType::*;
        use JobErrorType::*;

        match self {
            None => NONE,
            NotFound => NOT_FOUND,
            Cancelled => CANCELLED,
            NotConnected => NOT_CONNECTED,
            Xenon => XENON,
            InputOutput => IO,
            Other => OTHER,
        }
    }
}
