use std::collections::HashMap;
use crate::handler::Handler;

#[derive(Clone)]
pub struct Router{
    routes:HashMap<String, Handler>,
}

impl Router{
    pub fn new()->Self{
        let mut router=Router{
            routes:HashMap::new(),
        };
        // Register routes
        router.routes.insert("GET /".to_string(), Handler::get_index());
        router.routes.insert("GET /about".to_string(), Handler::get_about());
        router.routes.insert("POST /submit".to_string(), Handler::post_submit());
        
        router
    }

    pub fn handle_request(&self, request: &str) -> String {
        let request_line = request.lines().next().unwrap_or("");
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        
        if parts.len()>=2{
            let method=parts[0];
            let path=parts[1];
            let route_key=format!("{} {}", method, path);
            
            if let Some(handler)=self.routes.get(&route_key){
                return handler.handle(request);
            }
        }
        
        Handler::not_found()
    }
}