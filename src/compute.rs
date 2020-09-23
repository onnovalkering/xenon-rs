use anyhow::Result;
use crate::credentials::Credential;
use crate::xenon;
use crate::xenon_grpc::SchedulerServiceClient;
use grpcio::Channel;
use protobuf::RepeatedField;

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
    pub fn cancel_job(
        &self,
        job: Job,
    ) -> Result<JobStatus> {
        let mut request = xenon::JobRequest::new();
        request.set_job(job.proto());
        request.set_scheduler(self.scheduler.clone());

        let response = self.client.cancel_job(&request)?;

        Ok(JobStatus::from(response))
    }

    ///
    ///
    ///
    pub fn close(&mut self) -> Result<()> {
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
    ) -> Result<Scheduler> {
        let client = SchedulerServiceClient::new(channel);

        // Construct create request message.
        let mut request = xenon::CreateSchedulerRequest::new();
        request.set_adaptor(adaptor.clone());
        request.set_location(location);
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
            client,
        })
    }

    ///
    ///
    ///
    pub fn get_default_queue_name(&self) -> Result<String> {
        let response = self.client.get_default_queue_name(&self.scheduler)?;

        Ok(response.name)
    }

    ///
    ///
    ///
    pub fn get_job_status(
        &self,
        job: Job,
    ) -> Result<JobStatus> {
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
    ) -> Result<Vec<JobStatus>> {
        let jobs = jobs.iter().map(|j| j.clone().proto()).collect();

        let mut request = xenon::GetJobStatusesRequest::new();
        request.set_jobs(RepeatedField::from_vec(jobs));
        request.set_scheduler(self.scheduler.clone());

        let response = self.client.get_job_statuses(&request)?;
        let statuses = response.statuses.iter().map(|j| JobStatus::from(j.clone())).collect();

        Ok(statuses)
    }

    ///
    ///
    ///
    pub fn get_jobs(
        &self,
        queues: Option<Vec<String>>,
    ) -> Result<Vec<Job>> {
        let mut request = xenon::SchedulerAndQueues::new();
        request.set_scheduler(self.scheduler.clone());
        if let Some(queues) = queues {
            request.set_queues(RepeatedField::from_vec(queues));
        }

        let response = self.client.get_jobs(&request)?;
        let jobs = response.jobs.iter().map(|j| Job::new(j.id.clone())).collect();

        Ok(jobs)
    }

    ///
    ///
    ///
    pub fn get_properties(&self) -> Result<Map<String>> {
        let response = self.client.get_properties(&self.scheduler)?;

        Ok(response.properties)
    }

    ///
    ///
    ///
    pub fn get_queue_names(&self) -> Result<Vec<String>> {
        let response = self.client.get_queue_names(&self.scheduler)?;

        Ok(response.name.into_vec())
    }

    ///
    ///
    ///
    pub fn get_queue_status(
        &self,
        queue: String,
    ) -> Result<QueueStatus> {
        let mut request = xenon::GetQueueStatusRequest::new();
        request.set_scheduler(self.scheduler.clone());
        request.set_queue(queue);

        let response = self.client.get_queue_status(&request)?;
        Ok(QueueStatus::from(response))
    }

    ///
    ///
    ///
    pub fn get_queue_statuses(
        &self,
        queues: Option<Vec<String>>,
    ) -> Result<Vec<QueueStatus>> {
        let mut request = xenon::SchedulerAndQueues::new();
        request.set_scheduler(self.scheduler.clone());
        if let Some(queues) = queues {
            request.set_queues(RepeatedField::from_vec(queues));
        }

        let response = self.client.get_queue_statuses(&request)?;
        let statuses = response.statuses.iter().map(|s| QueueStatus::from(s.clone())).collect();

        Ok(statuses)
    }

    ///
    ///
    ///
    pub fn is_open(&mut self) -> Result<bool> {
        if self.open {
            let response = self.client.is_open(&self.scheduler)?;
            self.open = response.value
        }

        Ok(self.open)
    }

    ///
    ///
    ///
    pub fn submit_batch_job(
        &self,
        description: JobDescription,
    ) -> Result<Job> {
        let mut request = xenon::SubmitBatchJobRequest::new();
        request.set_description(description.proto());
        request.set_scheduler(self.scheduler.clone());

        let response = self.client.submit_batch_job(&request)?;

        Ok(Job::new(response.id))
    }

    ///
    ///
    ///
    pub fn wait_until_done(
        &self,
        job: Job,
        timeout: Option<u64>,
    ) -> Result<JobStatus> {
        let mut request = xenon::WaitRequest::new();
        request.set_job(job.proto());
        request.set_scheduler(self.scheduler.clone());
        if let Some(timeout) = timeout {
            request.set_timeout(timeout);
        }

        let response = self.client.wait_until_done(&request)?;

        Ok(JobStatus::from(response))
    }

    ///
    ///
    ///
    pub fn wait_until_running(
        &self,
        job: Job,
        timeout: Option<u64>,
    ) -> Result<JobStatus> {
        let mut request = xenon::WaitRequest::new();
        request.set_job(job.proto());
        request.set_scheduler(self.scheduler.clone());
        if let Some(timeout) = timeout {
            request.set_timeout(timeout);
        }

        let response = self.client.wait_until_running(&request)?;

        Ok(JobStatus::from(response))
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
    pub(crate) fn proto(self) -> xenon::JobDescription {
        let mut description = xenon::JobDescription::new();
        if let Some(arguments) = self.arguments {
            description.set_arguments(RepeatedField::from_vec(arguments));
        }
        if let Some(executable) = self.executable {
            description.set_executable(executable);
        }
        if let Some(working_directory) = self.working_directory {
            description.set_working_directory(working_directory);
        }
        if let Some(environment) = self.environment {
            description.set_environment(environment);
        }
        if let Some(queue) = self.queue {
            description.set_queue_name(queue);
        }
        if let Some(max_runtime) = self.max_runtime {
            description.set_max_runtime(max_runtime);
        }
        if let Some(stderr) = self.stderr {
            description.set_stderr(stderr);
        }
        if let Some(stdin) = self.stdin {
            description.set_stdin(stdin);
        }
        if let Some(stdout) = self.stdout {
            description.set_stdout(stdout);
        }
        if let Some(max_memory) = self.max_memory {
            description.set_max_memory(max_memory);
        }
        if let Some(scheduler_arguments) = self.scheduler_arguments {
            description.set_scheduler_arguments(RepeatedField::from_vec(scheduler_arguments));
        }
        if let Some(tasks) = self.tasks {
            description.set_tasks(tasks);
        }
        if let Some(cores_per_tasks) = self.cores_per_tasks {
            description.set_cores_per_task(cores_per_tasks);
        }
        if let Some(tasks_per_node) = self.tasks_per_node {
            description.set_tasks_per_node(tasks_per_node);
        }
        if let Some(start_per_task) = self.start_per_task {
            description.set_start_per_task(start_per_task);
        }
        if let Some(start_time) = self.start_time {
            description.set_start_time(start_time);
        }
        if let Some(temp_space) = self.temp_space {
            description.set_temp_space(temp_space);
        }

        description
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
    pub(crate) fn from(status: xenon::JobStatus) -> JobStatus {
        let error_type = JobErrorType::from(status.error_type);

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
    pub(crate) fn from(error_type: xenon::JobStatus_ErrorType) -> JobErrorType {
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
    pub(crate) fn from(status: xenon::QueueStatus) -> QueueStatus {
        let error_type = QueueErrorType::from(status.error_type);

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
    pub(crate) fn from(error_type: xenon::QueueStatus_ErrorType) -> QueueErrorType {
        use xenon::QueueStatus_ErrorType::*;
        use QueueErrorType::*;

        match error_type {
            NONE => None,
            NOT_FOUND => NotFound,
            NOT_CONNECTED => NotConnected,
            XENON => Xenon,
            IO => InputOutput,
            OTHER => Other,
        }
    }
}
