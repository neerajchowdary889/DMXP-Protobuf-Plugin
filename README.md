# DMXP Protobuf Plugin

The DMXP-Protobuf-Plugin (protoc-gen-dmxp) is the code generation tool for DMXP (Direct Message Exchange Protocol).

It extends the standard Protobuf workflow by allowing developers to define channels directly inside .proto files. From these definitions, the plugin generates publish/subscribe and function call stubs for multiple languages (Python, Rust, Go, etc.), giving developers a gRPC-like developer experience, but backed by ultra-fast shared memory IPC instead of HTTP/2 or brokers.

🔑 Key Points
- Define channels via custom Protobuf options.
- Auto-generate idiomatic publish() / subscribe() APIs.
- Works alongside normal --go_out, --python_out, etc.
- Simplifies DMXP adoption — no manual IPC or serialization needed.
