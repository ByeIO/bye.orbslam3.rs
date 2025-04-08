#![allow(unused)]

//! 翻译c2w-net-proxy为rust

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use rand::Rng;

// 常量定义
const PROXY_IP: &str = "192.168.127.253";
const GATEWAY_IP: &str = "192.168.127.1";
const VM_IP: &str = "192.168.127.3";

// 生成证书
fn generate_cert(host: &str) -> Result<(Vec<Certificate>, PrivateKey), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let serial = rng.gen::<u64>();
    let not_after = chrono::Utc::now() + chrono::Duration::days(365);

    // 创建证书模板
    let cert = rcgen::CertificateParams {
        serial_number: Some(serial.into()),
        subject_alt_names: vec![rcgen::SanType::DnsName(host.to_string())],
        not_after: not_after.into(),
        key_usages: vec![rcgen::KeyUsagePurpose::DigitalSignature],
        extended_key_usages: vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth],
        is_ca: rcgen::IsCa::No,
        ..Default::default()
    };

    let cert = rcgen::Certificate::from_params(cert)?;
    let cert_der = cert.serialize_der()?;
    let key_der = cert.serialize_private_key_der();

    Ok((vec![Certificate(cert_der)], PrivateKey(key_der)))
}

// 处理 HTTP 请求
async fn handle_http(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error>> {
    let client = Client::new();
    let resp = client.request(req).await?;
    Ok(resp)
}

// 处理隧道请求
async fn handle_tunneling(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error>> {
    let server_url = req.uri().host().unwrap_or("");
    let (cert, key) = generate_cert(server_url)?;

    let config = ServerConfig::builder()
       .with_safe_defaults()
       .with_no_client_auth()
       .with_single_cert(cert, key)?;

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let make_service = make_service_fn(|_conn: &AddrStream| async {
        Ok::<_, Box<dyn Error>>(service_fn(handle_http))
    });

    let server = Server::bind(&addr).serve(make_service);

    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    });

    Ok(Response::builder()
       .status(StatusCode::OK)
       .body(Body::empty())
       .unwrap())
}

// 主函数
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);

    let make_service = make_service_fn(|_conn: &AddrStream| async {
        Ok::<_, Box<dyn Error>>(service_fn(|req| async {
            if req.method() == hyper::Method::CONNECT {
                handle_tunneling(req).await
            } else {
                handle_http(req).await
            }
        }))
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
