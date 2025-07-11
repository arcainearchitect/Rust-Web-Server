use std::fs;

#[derive(Clone)]
pub enum Handler{
    GetIndex,
    GetAbout,
    PostSubmit,
}

impl Handler{
    pub fn get_index()->Self{
        Handler::GetIndex
    }
    
    pub fn get_about()->Self{
        Handler::GetAbout
    }
    
    pub fn post_submit()->Self{
        Handler::PostSubmit
    }
    
    pub fn handle(&self, request:&str)->String{
        match self{
            Handler::GetIndex=>Self::serve_index(),
            Handler::GetAbout=>Self::serve_about(),
            Handler::PostSubmit=>Self::handle_post(request),
        }
    }
    
    fn serve_index()->String{
        let contents=fs::read_to_string("static/index.html")
            .unwrap_or_else(|_| Self::default_index());
        
        format!(
            "HTTP/1.1 200 Ok\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        )
    }
    
    fn serve_about()->String{
        let contents=r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>About</title>
            </head>
            <body>
                <h1>About This Server</h1>
                <p>Features:</p>
                <ul>
                    <li> >Multi-threaded request handling</li>
                    <li> >Basic routing (GET/POST></li>
                    <li> >Static file serving</li>
                    <li> >Error handling </li>
                </ul>
                <a href="/">Back to Home</a>
            </body>
        </html>
        "#;
        
        format!(
            "HTTP/1.1 200 OK\r\nConent-Type: text/html\r\nContent-Length: {}\r\n\n{}",
            contents.len(),
            contents
        )
    }


    fn handle_post(request: &str) -> String {
        // Extract POST data (basic implementation)
        let body = request.split("\r\n\r\n").nth(1).unwrap_or("");

        let response_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head><title>Form Submitted</title></head>
            <body>
                <h1>Form Submitted Successfully!</h1>
                <p>Received data: {}</p>
                <a href="/">Back to Home</a>
            </body>
            </html>
            "#,
            body
        );

        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            response_body.len(),
            response_body
        )
    }

    fn default_index() -> String {
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Rust Web Server</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                h1 { color: #333; }
                form { margin: 20px 0; }
                input, textarea { margin: 5px; padding: 8px; }
                button { padding: 10px 20px; background: #007acc; color: white; border: none; cursor: pointer; }
                button:hover { background: #005999; }
            </style>
        </head>
        <body>
            <h1>Welcome to My Rust Web Server!</h1>
            <p>This server demonstrates:</p>
            <ul>
                <li>Multi-threaded TCP handling</li>
                <li>HTTP GET/POST request processing</li>
                <li>Basic routing system</li>
                <li>Static content serving</li>
            </ul>
            
            <h2>Test POST Request</h2>
            <form method="POST" action="/submit">
                <div>
                    <label>Name: <input type="text" name="name" required></label>
                </div>
                <div>
                    <label>Message: <textarea name="message" required></textarea></label>
                </div>
                <button type="submit">Submit</button>
            </form>
            
            <p><a href="/about">About This Server</a></p>
        </body>
        </html>
        "#.to_string()
    }

    pub fn not_found() -> String {
        let contents = r#"
        <!DOCTYPE html>
        <html>
        <head><title>404 Not Found</title></head>
        <body>
            <h1>404 - Page Not Found</h1>
            <p>The requested resource was not found on this server.</p>
            <a href="/">Back to Home</a>
        </body>
        </html>
        "#;

        format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        )
    }
}