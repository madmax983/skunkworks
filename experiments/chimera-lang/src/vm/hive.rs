#[cfg(feature = "hive")]
use super::{ChimeraVM, Value};
#[cfg(feature = "hive")]
use crate::ast::{Nucleotide, JunctionType};
#[cfg(feature = "hive")]
use crate::opcode::OpCode;
#[cfg(feature = "hive")]
use std::net::UdpSocket;
#[cfg(feature = "hive")]
use std::sync::Arc;

#[cfg(feature = "hive")]
pub fn exec_hive_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::HiveBind => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(port) = val {
                    let port = port as u16;
                    // Bind to 0.0.0.0:port
                    match UdpSocket::bind(format!("0.0.0.0:{}", port)) {
                        Ok(socket) => {
                            if let Err(e) = socket.set_nonblocking(true) {
                                vm.output.push(format!("HIVE: Failed to set nonblocking on {}: {}", port, e));
                            } else {
                                vm.hive_sockets.insert(port, Arc::new(socket));
                                vm.output.push(format!("HIVE: Bound port {}", port));
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("HIVE: Bind error on port {}: {}", port, e));
                        }
                    }
                } else {
                    vm.output.push("Error: Port must be Int".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for HiveBind".to_string());
            }
        }
        OpCode::HiveSend => {
            // [value, ip_str, port]
            if vm.stack.len() >= 3 {
                let port_val = vm.stack.pop().unwrap();
                let ip_val = vm.stack.pop().unwrap();
                let msg_val = vm.stack.pop().unwrap();

                if let (Value::Int(port), Value::Str(ip)) = (port_val, ip_val) {
                    let target = format!("{}:{}", ip, port);
                    // Always use ephemeral socket for sending to avoid blocking/contention on bound sockets
                    match UdpSocket::bind("0.0.0.0:0") {
                        Ok(socket) => {
                            match serde_json::to_string(&msg_val) {
                                Ok(json) => {
                                    if let Err(e) = socket.send_to(json.as_bytes(), &target) {
                                        vm.output.push(format!("HIVE: Send error to {}: {}", target, e));
                                    } else {
                                        vm.output.push(format!("HIVE: Sent to {}", target));
                                        vm.energy = vm.energy.saturating_sub(5);
                                    }
                                }
                                Err(e) => {
                                    vm.output.push(format!("HIVE: Serialization error: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                             vm.output.push(format!("HIVE: Socket creation error: {}", e));
                        }
                    }
                } else {
                    vm.output.push("Error: Invalid args for HiveSend".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for HiveSend".to_string());
            }
        }
        OpCode::HiveRecv => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(port) = val {
                    let port = port as u16;
                    if let Some(socket) = vm.hive_sockets.get(&port) {
                        let mut buf = [0u8; 65535]; // Max UDP size
                        match socket.recv_from(&mut buf) {
                            Ok((amt, src)) => {
                                let data = &buf[..amt];
                                match serde_json::from_slice::<Value>(data) {
                                    Ok(recv_val) => {
                                        // Return Junction(Dish, [port, ip, value])
                                        let packet = vec![
                                            Value::Int(src.port() as i64),
                                            Value::Str(src.ip().to_string()),
                                            recv_val
                                        ];
                                        vm.stack.push(Value::Junction(JunctionType::Dish, packet));
                                        vm.output.push(format!("HIVE: Recv from {}", src));
                                    }
                                    Err(e) => {
                                        vm.output.push(format!("HIVE: Deserialization error from {}: {}", src, e));
                                        vm.stack.push(Value::Int(0));
                                    }
                                }
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // No data
                                vm.stack.push(Value::Int(0));
                            }
                            Err(e) => {
                                vm.output.push(format!("HIVE: Recv error on port {}: {}", port, e));
                                vm.stack.push(Value::Int(0));
                            }
                        }
                    } else {
                        vm.output.push(format!("HIVE: Port {} not bound", port));
                        vm.stack.push(Value::Int(0));
                    }
                } else {
                     vm.output.push("Error: Port must be Int".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for HiveRecv".to_string());
            }
        }
        OpCode::HiveClose => {
            if let Some(val) = vm.stack.pop() {
                 if let Value::Int(port) = val {
                     let port = port as u16;
                     if vm.hive_sockets.remove(&port).is_some() {
                         vm.output.push(format!("HIVE: Closed port {}", port));
                     } else {
                         vm.output.push(format!("HIVE: Port {} was not bound", port));
                     }
                 }
            }
        }
        _ => {}
    }
}
