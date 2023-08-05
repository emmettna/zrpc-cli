use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub struct Host(pub String);

impl Host {
    pub fn from(s: String) -> Result<Host, String> {
        if s.contains(":") {
            Err(String::from("Invalid format. contains `:`"))
        } else if s.contains("/") {
            Err(String::from("Invalid format. contains `/`"))
        } else {
            Ok(Host(s))
        }
    }
}

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Port(pub u16);

impl Port {
    pub fn from(s: String) -> Result<Port, String> {
        match s.parse::<u16>() {
            Ok(p) => Ok(Port(p)),
            Err(e) => Err(String::from(format!("Failed to parse Port: {}", e.to_string())))
        }
    }
}

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RequestBody(pub String);

impl RequestBody {
    pub fn from(s: &str) -> RequestBody {
        RequestBody(String::from(s))
    }
}

impl std::fmt::Display for RequestBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ServiceFunction(pub String);

impl ServiceFunction {
    pub fn from(s: &str) -> ServiceFunction { ServiceFunction(String::from(s)) }
}

impl std::fmt::Display for ServiceFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ServiceName(pub String);

impl ServiceName {
    pub fn from(s: &str) -> ServiceName { ServiceName(String::from(s)) }
}

impl std::fmt::Display for ServiceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceRequest {
    pub host: Host,
    pub port: Port,
    pub service_name: ServiceName,
    pub service_function: ServiceFunction,
    pub body: RequestBody,
}

impl ServiceRequest {
    pub fn default() -> ServiceRequest {
        Self::new(Host::from(String::from("localhost")).unwrap())
    }

    pub fn new(host: Host) -> ServiceRequest {
        ServiceRequest {
            host,
            port: Port(9090),
            service_name: ServiceName::from(""),
            service_function: ServiceFunction::from(""),
            body: RequestBody::from("{}"),
        }
    }

    pub fn update_host(&mut self, host: Host) {
        self.host = host;
    }

    pub fn update_port(&mut self, port: Port) {
        self.port = port;
    }

    pub fn update_service(&mut self, service_name: ServiceName) {
        self.service_name = service_name;
    }

    pub fn update_function(&mut self, service_function: ServiceFunction) {
        let service_name = &self.service_name;
        let updated_function = if service_function.0.contains(&service_name.0) {
            let size = service_name.0.len() + 1;
            let new = &service_function.0[size..];
            ServiceFunction(String::from(new))
        } else {
            service_function
        };
        self.service_function = updated_function
    }

    pub fn update_body(&mut self, body: String) {
        self.body = RequestBody(body)
    }

    pub fn pretty_string(&self) -> String {
        format!("Host: {}\nPort: {}\nServiceName: {}\nMethod: {}\nbody: {}", self.host, self.port, self.service_name, self.service_function, self.body)
    }
}
