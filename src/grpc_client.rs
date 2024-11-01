use crate::grpc_request_dsl;

use tonic::*;
use grpc_request_dsl::*;
use tonic::transport::Channel;
use tonic::codegen::http::Uri;
use tonic::Request;
use prost::Message;

async fn create_channel(host: &Host, port: &Port) -> Result<Channel, String> {
    let uri = format!("http://{}:{}", host.0, port.0)
        .parse::<Uri>()
        .map_err(|e| e.to_string())?;
    Channel::builder(uri)
        .connect()
        .await
        .map_err(|e| e.to_string())
}

pub async fn request_service_list(host: &Host, port: &Port) -> Result<Vec<ServiceName>, String> {
    let channel = create_channel(host, port).await?;
    let mut client = tonic_reflection::v1alpha::server_reflection_client::ServerReflectionClient::new(channel);

    let request = tonic_reflection::v1alpha::ServerReflectionRequest {
        host: host.0.clone(),
        message_request: Some(tonic_reflection::v1alpha::server_reflection_request::MessageRequest::ListServices(tonic_reflection::v1alpha::ListServiceRequest {})),
    };

    let response = client.server_reflection_info(Request::new(request)).await?;
    let services = response.into_inner().message_response.unwrap().list_services_response.unwrap().service;
    Ok(services.into_iter().map(|s| ServiceName(s.name)).collect())
}

pub async fn request_function_list_by(request: &ServiceRequest) -> Result<Vec<ServiceFunction>, String> {
    request_function_list(&request.host, &request.port, &request.service_name).await
}

pub async fn request_function_list(host: &Host, port: &Port, service: &ServiceName) -> Result<Vec<ServiceFunction>, String> {
    let channel = create_channel(host, port).await?;
    let mut client = tonic_reflection::v1alpha::server_reflection_client::ServerReflectionClient::new(channel);

    let request = tonic_reflection::v1alpha::ServerReflectionRequest {
        host: host.0.clone(),
        message_request: Some(tonic_reflection::v1alpha::server_reflection_request::MessageRequest::FileContainingSymbol(service.0.clone())),
    };

    let response = client.server_reflection_info(Request::new(request)).await?;
    let file_descriptor_response = response.into_inner().message_response.unwrap().file_descriptor_response.unwrap();
    let file_descriptor_set = prost_types::FileDescriptorSet::decode(&*file_descriptor_response.file_descriptor_proto[0]).unwrap();
    let functions = file_descriptor_set.file[0].service[0].method.iter().map(|m| ServiceFunction(m.name.clone())).collect();
    Ok(functions)
}

pub async fn request(service_request: &ServiceRequest) -> Result<String, String> {
    let channel = create_channel(&service_request.host, &service_request.port).await?;
    let mut client = tonic::client::Grpc::new(channel);

    let request = tonic::Request::new(tonic::codegen::http::Request::builder()
        .uri(format!("http://{}:{}/{}", service_request.host.0, service_request.port.0, service_request.service_function.0))
        .body(tonic::codegen::http::Body::from(service_request.body.0.clone()))
        .unwrap());

    let response = client.unary_call(request).await?;
    let response_body = response.into_inner().into_body().data().await.unwrap().unwrap();
    let response_str = std::str::from_utf8(&response_body).map_err(|e| e.to_string())?;
    Ok(response_str.to_string())
}
