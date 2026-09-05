//! 面向本地游戏窗口的极简静态文件服务器。
//!
//! GDevelop 网页版导出必须通过 HTTP 访问（ES Module / fetch 资源），
//! 因此启动器内嵌一个仅监听 127.0.0.1 随机端口的服务器来提供游戏目录的文件。

use std::fs::File;
use std::io::BufReader;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

use percent_encoding::percent_decode_str;
use tiny_http::{Header, Response, Server};
const WORKER_THREADS: usize = 8;

pub struct GameServer {
    pub port: u16,
    root: Arc<Mutex<PathBuf>>,
}

impl GameServer {
    /// 绑定 127.0.0.1 上的随机空闲端口并启动工作线程。
    pub fn start(initial_root: PathBuf) -> Result<Self, String> {
        let listener =
            TcpListener::bind(("127.0.0.1", 0)).map_err(|e| format!("无法绑定本地端口：{e}"))?;
        let port = listener
            .local_addr()
            .map_err(|e| format!("无法获取本地端口：{e}"))?
            .port();
        let server = Arc::new(
            Server::from_listener(listener, None)
                .map_err(|e| format!("无法启动本地游戏服务：{e}"))?,
        );
        let root = Arc::new(Mutex::new(initial_root));
        for _ in 0..WORKER_THREADS {
            let server = Arc::clone(&server);
            let root = Arc::clone(&root);
            thread::spawn(move || loop {
                match server.recv() {
                    Ok(request) => handle_request(request, &root),
                    Err(_) => break,
                }
            });
        }
        Ok(Self { port, root })
    }

    /// 切换服务的游戏目录（设置更改后无需重启服务器）。
    pub fn set_root(&self, root: PathBuf) {
        *self.root.lock().unwrap() = root;
    }

    /// 当前服务的游戏目录（用于判断复用游戏窗口前是否需要重新加载）。
    pub fn root(&self) -> PathBuf {
        self.root.lock().unwrap().clone()
    }
}

fn handle_request(request: tiny_http::Request, root: &Arc<Mutex<PathBuf>>) {
    let method = request.method().as_str().to_ascii_uppercase();
    if method != "GET" && method != "HEAD" {
        let _ = request.respond(
            Response::from_string("Method Not Allowed")
                .with_status_code(405)
                .boxed(),
        );
        return;
    }

    let url = request.url().to_string();
    let Some(rel) = sanitize_path(&url) else {
        let _ = request.respond(Response::from_string("Forbidden").with_status_code(403).boxed());
        return;
    };

    let root_now = root.lock().unwrap().clone();
    let mut full = root_now.join(&rel);
    if full.is_dir() {
        full = full.join("index.html");
    }

    let response: tiny_http::ResponseBox = match std::fs::metadata(&full) {
        Ok(meta) if meta.is_file() => match File::open(&full) {
            Ok(file) => {
                let len = meta.len();
                let mut headers = Vec::with_capacity(2);
                headers.push(header("Content-Type", content_type(&full)));
                headers.push(header("Cache-Control", "no-cache"));
                Response::new(
                    tiny_http::StatusCode(200),
                    headers,
                    Box::new(BufReader::new(file)),
                    Some(len as usize),
                    None,
                )
                .boxed()
            }
            Err(_) => Response::from_string("Internal Server Error")
                .with_status_code(500)
                .boxed(),
        },
        _ => Response::from_string("Not Found").with_status_code(404).boxed(),
    };
    let _ = request.respond(response);
}

/// 把 URL 路径解码为游戏目录内的相对路径，拒绝目录穿越。
///
/// `/` 与 `\` 均按分隔符切分：Windows 文件系统把两者等同，若只按 `/` 切分，
/// `%5C..%5C`（反斜杠）会作为普通组件绕过 `..` 检查实现穿越。
/// 含 `:` 的组件同样拒绝，防 `C:/...` 盘符绝对路径（PathBuf::push 会整体
/// 替换根目录）与 NTFS 备用数据流。
fn sanitize_path(url: &str) -> Option<PathBuf> {
    let without_query = url.split(['?', '#']).next()?;
    let decoded = percent_decode_str(without_query).decode_utf8_lossy();
    let mut out = PathBuf::new();
    for component in decoded.split(['/', '\\']) {
        match component {
            "" | "." => {}
            ".." => return None,
            c if c.contains(':') => return None,
            c => out.push(c),
        }
    }
    if out.as_os_str().is_empty() {
        out.push("index.html");
    }
    Some(out)
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes())
        .expect("静态响应头构造失败")
}

fn content_type(path: &Path) -> &'static str {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "html" | "htm" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" | "map" => "application/json; charset=utf-8",
        "txt" | "xml" | "csv" => "text/plain; charset=utf-8",
        "wasm" => "application/wasm",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "ico" => "image/x-icon",
        "bmp" => "image/bmp",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" | "oga" => "audio/ogg",
        "m4a" => "audio/mp4",
        "aac" => "audio/aac",
        "flac" => "audio/flac",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "eot" => "application/vnd.ms-fontobject",
        "glb" => "model/gltf-binary",
        "gltf" => "model/gltf+json",
        _ => "application/octet-stream",
    }
}
