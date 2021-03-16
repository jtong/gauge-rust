use crate::scanner;

use tonic::{transport::Server, Request, Response, Status};


use hello_world1::runner_server::{Runner, RunnerServer};
use hello_world1::{StepValidateResponse,
                   StepPositionsResponse,
                   StepNamesResponse,
                   StepNameResponse,
                   ExecutionStatusResponse,
                   ImplementationFileListResponse,
                   ImplementationFileGlobPatternResponse,
                   RefactorResponse,
                   Empty,
                   FileDiff,
                   RefactorRequest,
                   KillProcessRequest,
                   ExecutionStartingRequest,
                   ExecutionEndingRequest,
                   ExecuteStepRequest,
                   ScenarioExecutionStartingRequest,
                   ScenarioExecutionEndingRequest,
                   ScenarioDataStoreInitRequest,
                   SuiteDataStoreInitRequest,
                   StepNamesRequest,
                   StepNameRequest,
                   StepValidateRequest,
                   StepPositionsRequest,
                   StepExecutionStartingRequest,
                   StepExecutionEndingRequest,
                   SpecDataStoreInitRequest,
                   SpecExecutionStartingRequest,
                   SpecExecutionEndingRequest,
                   StubImplementationCodeRequest,
                   CacheFileRequest};
use futures::executor::block_on;

use tokio::runtime::Runtime;


pub mod hello_world1 {
    tonic::include_proto!("gauge.messages"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyRunner {}

#[tonic::async_trait]
impl Runner for MyRunner {
    async fn validate_step(&self, request: Request<StepValidateRequest>) -> Result<Response<StepValidateResponse>, Status> {
        Ok(Response::new(StepValidateResponse{
            error_message: "".to_string(),
            error_type: 0,
            is_valid: true,
            suggestion: "".to_string()
        }))
    }


    // SuiteDataStoreInit is a RPC to initialize the suite level data store.
    //
    // Accepts a Empty message and returns a ExecutionStatusResponse message
    async fn initialize_suite_data_store(&self, request: Request<SuiteDataStoreInitRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }

    // ExecutionStarting is a RPC to tell runner to execute Suite level hooks.
    //
    // Accepts a ExecutionStartingRequest message and returns a ExecutionStatusResponse message
    async fn start_execution(&self, request: Request<ExecutionStartingRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // SpecDataStoreInit is a RPC to initialize the spec level data store.
    //
    // Accepts a Empty message and returns a ExecutionStatusResponse message
    async fn initialize_spec_data_store(&self, request: Request<SpecDataStoreInitRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // SpecExecutionStarting is a RPC to tell runner to execute spec level hooks.
    //
    // Accepts a SpecExecutionStartingRequest message and returns a ExecutionStatusResponse message
    async fn start_spec_execution(&self, request: Request<SpecExecutionStartingRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // ScenarioDataStoreInit is a RPC to initialize the scenario level data store.
    //
    // Accepts a Empty message and returns a ExecutionStatusResponse message
    async fn initialize_scenario_data_store(&self, request: Request<ScenarioDataStoreInitRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // ScenarioExecutionStarting is a RPC to tell runner to execute scenario level hooks.
    //
    // Accepts a ScenarioExecutionStartingRequest message and returns a ExecutionStatusResponse message
    async fn start_scenario_execution(&self, request: Request<ScenarioExecutionStartingRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // StepExecutionStarting is a RPC to tell runner to execute step level hooks.
    //
    // Accepts a StepExecutionStartingRequest message and returns a ExecutionStatusResponse message
    async fn start_step_execution(&self, request: Request<StepExecutionStartingRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // ExecuteStep is a RPC to tell runner to execute a step .
    //
    // Accepts a ExecuteStepRequest message and returns a ExecutionStatusResponse message
    async fn execute_step(&self, request: Request<ExecuteStepRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // StepExecutionEnding is a RPC to tell runner to execute step level hooks.
    //
    // Accepts a StepExecutionEndingRequest message and returns a ExecutionStatusResponse message
    async fn finish_step_execution(&self, request: Request<StepExecutionEndingRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // ScenarioExecutionEnding is a RPC to tell runner to execute Scenario level hooks.
    //
    // Accepts a ScenarioExecutionEndingRequest message and returns a ExecutionStatusResponse message
    async fn finish_scenario_execution(&self, request: Request<ScenarioExecutionEndingRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // SpecExecutionEnding is a RPC to tell runner to execute spec level hooks.
    //
    // Accepts a SpecExecutionEndingRequest message and returns a ExecutionStatusResponse message
    async fn finish_spec_execution(&self, request: Request<SpecExecutionEndingRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // ExecutionEnding is a RPC to tell runner to execute suite level hooks.
    //
    // Accepts a ExecutionEndingRequest message and returns a ExecutionStatusResponse message
    async fn finish_execution(&self, request: Request<ExecutionEndingRequest>) -> Result<Response<ExecutionStatusResponse>, Status> {
        Ok(Response::new(ExecutionStatusResponse{
            execution_result: None
        }))
    }


    // CacheFile is a RPC to tell runner to load/reload/unload a implementation file.
    //
    // Accepts a CacheFileRequest message and returns a Empty message
    async fn cache_file(&self, request: Request<CacheFileRequest>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty{}))
    }


    // GetStepName is a RPC to get information about the given step.
    //
    // Accepts a StepNameRequest message and returns a StepNameResponse message.
    async fn get_step_name(&self, request: Request<StepNameRequest>) -> Result<Response<StepNameResponse>, Status> {
        Ok(Response::new(StepNameResponse{
            file_name: "".to_string(),
            has_alias: false,
            is_external: true,
            is_step_present: true,
            span: None,
            step_name: (&[]).to_vec()

        }))
    }


    // GetGlobPatterns is a RPC to get the file path pattern which needs to be cached.
    //
    // Accepts a Empty message and returns a ImplementationFileGlobPatternResponse message.
    async fn get_glob_patterns(&self, request: Request<Empty>) -> Result<Response<ImplementationFileGlobPatternResponse>, Status> {
        Ok(Response::new(ImplementationFileGlobPatternResponse{
            glob_patterns: (&[]).to_vec()
        }))
    }


    // GetStepNames is a RPC to get all the available steps from the runner.
    //
    // Accepts a StepNamesRequest message and returns a StepNamesResponse
    async fn get_step_names(&self, request: Request<StepNamesRequest>) -> Result<Response<StepNamesResponse>, Status> {
        Ok(Response::new(StepNamesResponse{
            steps: (&[]).to_vec()
        }))
    }


    // GetStepPositions is a RPC to get positions of all available steps in a given file.
    //
    // Accepts a StepPositionsRequest message and returns a StepPositionsResponse message
    async fn get_step_positions(&self, request: Request<StepPositionsRequest>) -> Result<Response<StepPositionsResponse>, Status> {
        Ok(Response::new(StepPositionsResponse{
            error: "".to_string(),
            step_positions: (&[]).to_vec()
        }))
    }


    // GetImplementationFiles is a RPC get all the existing implementation files.
    //
    // Accepts a Empty and returns a ImplementationFileListResponse message.
    async fn get_implementation_files(&self, request: Request<Empty>) -> Result<Response<ImplementationFileListResponse>, Status> {
        Ok(Response::new(ImplementationFileListResponse{
            implementation_file_paths: (&[]).to_vec()
        }))
    }


    // ImplementStub is a RPC to to ask runner to add a given implementation to given file.
    //
    // Accepts a StubImplementationCodeRequest and returns a FileDiff message.
    async fn implement_stub(&self, request: Request<StubImplementationCodeRequest>) -> Result<Response<FileDiff>, Status> {
        Ok(Response::new(FileDiff{
            file_path: "".to_string(),
            text_diffs: (&[]).to_vec()
        }))
    }


    // Refactor is a RPC to refactor a given step in implementation file.
    //
    // Accepts a RefactorRequest message and returns a RefactorResponse message.
    async fn refactor(&self, request: Request<RefactorRequest>) -> Result<Response<RefactorResponse>, Status> {
        Ok(Response::new(RefactorResponse {
            success: true,
            error: "no".to_string(),
            files_changed:(&[]).to_vec(),
            file_changes:(&[]).to_vec()
        }))
    }


    // Kill is a RPC tell plugin to stop gasync fn server and kill the plugin process.
    //
    // Accepts a KillProcessRequest message and returns a Empty message.
    async fn kill(&self, request: Request<KillProcessRequest>) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty { }))
    }

    // async fn say_hello(
//         &self,
//         request: Request<HelloRequest>, // Accept request of type HelloRequest
//     ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
//         println!("Got a request: {:?}",request);

//         let reply = hello_world1::HelloReply {
//             message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
//         };

//         Ok(Response::new(reply)) // Send back our formatted greeting
//     }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "127.0.0.1:55445".parse()?;
//     let runner = MyRunner::default();
//     println!("Server listening on {}",addr);
//
//     Server::builder()
//         .add_service(RunnerServer::new(runner))
//         .serve(addr)
//         .await?;
//
//     Ok(())
// }

async fn server()-> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:0".parse()?;
    let runner = MyRunner::default();
    //println!("Server listening on {}",addr);

    Server::builder()
        .add_service(RunnerServer::new(runner))
        .serve(addr)
        .await?;
    // println!("Server listening on {}",addr);
    Ok(())
}


pub fn run() {

    scanner::scan();
    //server();
    // let mut rt = Runtime::new().expect("failed to obtain a new RunTime object");

    // let server_future = server();
    // // let server_future = Server::builder()
    // //     .add_service(GreeterServer::new(greeter))
    // //     .serve(addr);
    // rt.block_on(server_future).expect("failed to successfully run the future on RunTime");
}
