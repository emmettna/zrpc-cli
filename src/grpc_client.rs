use crate::grpc_request_dsl;

use tonic::*;
use grpc_request_dsl::*;
use tonic::transport::Channel;
use tonic::codegen::http::Uri;
use tonic::service::Interceptor;
use tonic::metadata::MetadataValue;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::codegen::http::Uri;
use tonic::service::Interceptor;
use tonic::metadata::MetadataValue;
use tonic::Request;
use tonic::Response;
use tonic::Status;

pub async fn request_service_list(host: &Host, port: &Port) -> Result<Vec<ServiceName>, Box<dyn std::error::Error>> {
    let uri = format!("http://{}:{}", host.0, port.0).parse::<Uri>()?;
    let channel = Channel::builder(uri).connect().await?;
    let mut client = tonic::client::Grpc::new(channel);

    let request = tonic::Request::new(());
    let response = client.unary(request, "/grpc.reflection.v1alpha.ServerReflection/ServerReflectionInfo", tonic::codec::ProstCodec::default()).await?;

    let services: Vec<ServiceName> = response.into_inner().services.iter().map(|s| ServiceName(s.clone())).collect();
    Ok(services)
}

pub async fn request_function_list_by(request: &ServiceRequest) -> Result<Vec<ServiceFunction>, Box<dyn std::error::Error>> {
    request_function_list(&request.host, &request.port, &request.service_name).await
}

pub async fn request_function_list(host: &Host, port: &Port, service: &ServiceName) -> Result<Vec<ServiceFunction>, Box<dyn std::error::Error>> {
    let uri = format!("http://{}:{}", host.0, port.0).parse::<Uri>()?;
    let channel = Channel::builder(uri).connect().await?;
    let mut client = tonic::client::Grpc::new(channel);

    let request = tonic::Request::new(());
    let response = client.unary(request, "/grpc.reflection.v1alpha.ServerReflection/ServerReflectionInfo", tonic::codec::ProstCodec::default()).await?;

    let functions: Vec<ServiceFunction> = response.into_inner().services.iter().filter(|s| s.starts_with(&service.0)).map(|s| ServiceFunction(s.clone())).collect();
    Ok(functions)
}

pub async fn request(service_request: &ServiceRequest) -> Result<String, Box<dyn std::error::Error>> {
    let uri = format!("http://{}:{}", service_request.host.0, service_request.port.0).parse::<Uri>()?;
    let channel = Channel::builder(uri).connect().await?;
    let mut client = tonic::client::Grpc::new(channel);

    let request = tonic::Request::new(service_request.body.0.clone());
    let response = client.unary(request, format!("/{}/{}", service_request.service_name.0, service_request.service_function.0), tonic::codec::ProstCodec::default()).await?;

    Ok(response.into_inner())
}
