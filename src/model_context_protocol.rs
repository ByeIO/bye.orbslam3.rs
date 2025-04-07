use serde::{Deserialize, Serialize};

/* JSON-RPC 类型 */

/// 指代任何可以从网络解码或编码后发送的有效 JSON-RPC 对象。
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum JSONRPCMessage {
    Request(JSONRPCRequest),
    Notification(JSONRPCNotification),
    BatchRequest(JSONRPCBatchRequest),
    Response(JSONRPCResponse),
    Error(JSONRPCError),
    BatchResponse(JSONRPCBatchResponse),
}

/// 一个 JSON-RPC 批量请求，如 https://www.jsonrpc.org/specification#batch 所述。
pub type JSONRPCBatchRequest = Vec<Either<JSONRPCRequest, JSONRPCNotification>>;

/// 一个 JSON-RPC 批量响应，如 https://www.jsonrpc.org/specification#batch 所述。
pub type JSONRPCBatchResponse = Vec<Either<JSONRPCResponse, JSONRPCError>>;

pub const LATEST_PROTOCOL_VERSION: &str = "2025-03-26";
pub const JSONRPC_VERSION: &str = "2.0";

/// 一个进度令牌，用于将进度通知与原始请求关联起来。
pub type ProgressToken = Either<String, i64>;

/// 一个不透明的令牌，用于表示分页的游标。
pub type Cursor = String;

/// 请求结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub method: String,
    pub params: Option<RequestParams>,
}

/// 请求参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestParams {
    #[serde(rename = "_meta")]
    pub meta: Option<MetaParams>,
    #[serde(flatten)]
    pub other_params: serde_json::Map<String, serde_json::Value>,
}

/// 元参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaParams {
    #[serde(rename = "progressToken")]
    pub progress_token: Option<ProgressToken>,
}

/// 通知结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub method: String,
    pub params: Option<NotificationParams>,
}

/// 通知参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationParams {
    #[serde(rename = "_meta")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(flatten)]
    pub other_params: serde_json::Map<String, serde_json::Value>,
}

/// 结果结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    #[serde(rename = "_meta")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    #[serde(flatten)]
    pub other_results: serde_json::Map<String, serde_json::Value>,
}

/// 一个用于 JSON-RPC 请求的唯一标识符。
pub type RequestId = Either<String, i64>;

/// 一个期望响应的请求。
#[derive(Serialize, Deserialize, Debug)]
pub struct JSONRPCRequest {
    pub jsonrpc: &'static str,
    pub id: RequestId,
    #[serde(flatten)]
    pub request: Request,
}

/// 一个不期望响应的通知。
#[derive(Serialize, Deserialize, Debug)]
pub struct JSONRPCNotification {
    pub jsonrpc: &'static str,
    #[serde(flatten)]
    pub notification: Notification,
}

/// 对请求的成功（非错误）响应。
#[derive(Serialize, Deserialize, Debug)]
pub struct JSONRPCResponse {
    pub jsonrpc: &'static str,
    pub id: RequestId,
    pub result: Result,
}

// 标准 JSON-RPC 错误代码
pub const PARSE_ERROR: i64 = -32700;
pub const INVALID_REQUEST: i64 = -32600;
pub const METHOD_NOT_FOUND: i64 = -32601;
pub const INVALID_PARAMS: i64 = -32602;
pub const INTERNAL_ERROR: i64 = -32603;

/// 对请求的响应，表示发生了错误。
#[derive(Serialize, Deserialize, Debug)]
pub struct JSONRPCError {
    pub jsonrpc: &'static str,
    pub id: RequestId,
    pub error: ErrorInfo,
}

/// 错误信息结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorInfo {
    pub code: i64,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/* 空结果 */
/// 一个表示成功但不携带数据的响应。
pub type EmptyResult = Result;

/* 取消操作 */
/// 此通知可由任一方发送，以表明它正在取消先前发出的请求。
#[derive(Serialize, Deserialize, Debug)]
pub struct CancelledNotification {
    pub method: &'static str,
    pub params: CancelledParams,
}

/// 取消操作的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct CancelledParams {
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
    pub reason: Option<String>,
}

/* 初始化 */
/// 当客户端首次连接时，会向服务器发送此请求，要求其开始初始化。
#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeRequest {
    pub method: &'static str,
    pub params: InitializeParams,
}

/// 初始化请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeParams {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    pub clientInfo: Implementation,
}

/// 服务器在接收到客户端的初始化请求后，会发送此响应。
#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    pub capabilities: ServerCapabilities,
    pub serverInfo: Implementation,
    pub instructions: Option<String>,
}

/// 初始化完成后，客户端会向服务器发送此通知。
#[derive(Serialize, Deserialize, Debug)]
pub struct InitializedNotification {
    pub method: &'static str,
}

/// 客户端可能支持的功能。已知的功能在此模式中定义，但这不是一个封闭的集合：任何客户端都可以定义自己的额外功能。
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientCapabilities {
    pub experimental: Option<serde_json::Map<String, serde_json::Value>>,
    pub roots: Option<RootsCapabilities>,
    pub sampling: Option<serde_json::Value>,
}

/// 根目录功能结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct RootsCapabilities {
    #[serde(rename = "listChanged")]
    pub list_changed: Option<bool>,
}

/// 服务器可能支持的功能。已知的功能在此模式中定义，但这不是一个封闭的集合：任何服务器都可以定义自己的额外功能。
#[derive(Serialize, Deserialize, Debug)]
pub struct ServerCapabilities {
    pub experimental: Option<serde_json::Map<String, serde_json::Value>>,
    pub logging: Option<serde_json::Value>,
    pub completions: Option<serde_json::Value>,
    pub prompts: Option<PromptsCapabilities>,
    pub resources: Option<ResourcesCapabilities>,
    pub tools: Option<ToolsCapabilities>,
}

/// 提示功能结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct PromptsCapabilities {
    #[serde(rename = "listChanged")]
    pub list_changed: Option<bool>,
}

/// 资源功能结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourcesCapabilities {
    pub subscribe: Option<bool>,
    #[serde(rename = "listChanged")]
    pub list_changed: Option<bool>,
}

/// 工具功能结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ToolsCapabilities {
    #[serde(rename = "listChanged")]
    pub list_changed: Option<bool>,
}

/// 描述 MCP 实现的名称和版本。
#[derive(Serialize, Deserialize, Debug)]
pub struct Implementation {
    pub name: String,
    pub version: String,
}

/* 心跳 */
/// 由服务器或客户端发出的心跳，用于检查对方是否仍然存活。接收方必须立即响应，否则可能会被断开连接。
#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {
    pub method: &'static str,
}

/* 进度通知 */
/// 用于向接收方通知长时间运行请求的进度更新的带外通知。
#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressNotification {
    pub method: &'static str,
    pub params: ProgressParams,
}

/// 进度通知的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressParams {
    #[serde(rename = "progressToken")]
    pub progress_token: ProgressToken,
    pub progress: f64,
    pub total: Option<f64>,
    pub message: Option<String>,
}

/* 分页 */
/// 分页请求结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedRequest {
    pub method: String,
    pub params: Option<PaginatedParams>,
}

/// 分页请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedParams {
    pub cursor: Option<Cursor>,
}

/// 分页结果结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedResult {
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<Cursor>,
    #[serde(flatten)]
    pub other_results: serde_json::Map<String, serde_json::Value>,
}

/* 资源 */
/// 客户端发送此请求以获取服务器拥有的资源列表。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListResourcesRequest {
    pub method: &'static str,
    #[serde(flatten)]
    pub paginated_request: PaginatedRequest,
}

/// 服务器对客户端的 resources/list 请求的响应。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListResourcesResult {
    pub resources: Vec<Resource>,
    #[serde(flatten)]
    pub paginated_result: PaginatedResult,
}

/// 客户端发送此请求以获取服务器拥有的资源模板列表。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListResourceTemplatesRequest {
    pub method: &'static str,
    #[serde(flatten)]
    pub paginated_request: PaginatedRequest,
}

/// 服务器对客户端的 resources/templates/list 请求的响应。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListResourceTemplatesResult {
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: Vec<ResourceTemplate>,
    #[serde(flatten)]
    pub paginated_result: PaginatedResult,
}

/// 客户端向服务器发送此请求，以读取特定的资源 URI。
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadResourceRequest {
    pub method: &'static str,
    pub params: ReadResourceParams,
}

/// 读取资源请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadResourceParams {
    pub uri: String,
}

/// 服务器对客户端的 resources/read 请求的响应。
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadResourceResult {
    pub contents: Vec<Either<TextResourceContents, BlobResourceContents>>,
    #[serde(flatten)]
    pub result: Result,
}

/// 服务器向客户端发送的可选通知，告知其可以读取的资源列表已更改。服务器可以在客户端没有任何先前订阅的情况下发出此通知。
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceListChangedNotification {
    pub method: &'static str,
}

/// 客户端发送此请求，以在特定资源发生更改时从服务器接收 resources/updated 通知。
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeRequest {
    pub method: &'static str,
    pub params: SubscribeParams,
}

/// 订阅请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribeParams {
    pub uri: String,
}

/// 客户端发送此请求，以取消从服务器接收 resources/updated 通知。这应该在先前的 resources/subscribe 请求之后进行。
#[derive(Serialize, Deserialize, Debug)]
pub struct UnsubscribeRequest {
    pub method: &'static str,
    pub params: UnsubscribeParams,
}

/// 取消订阅请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct UnsubscribeParams {
    pub uri: String,
}

/// 服务器向客户端发送的通知，告知其资源已更改，可能需要再次读取。只有在客户端先前发送了 resources/subscribe 请求时，才应发送此通知。
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceUpdatedNotification {
    pub method: &'static str,
    pub params: ResourceUpdatedParams,
}

/// 资源更新通知的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceUpdatedParams {
    pub uri: String,
}

/// 服务器能够读取的已知资源。
#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub annotations: Option<Annotations>,
}

/// 服务器上可用资源的模板描述。
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceTemplate {
    #[serde(rename = "uriTemplate")]
    pub uri_template: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub annotations: Option<Annotations>,
}

/// 特定资源或子资源的内容。
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResourceContents {
    Text(TextResourceContents),
    Blob(BlobResourceContents),
}

/// 文本资源内容结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct TextResourceContents {
    pub uri: String,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub text: String,
}

/// 二进制资源内容结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct BlobResourceContents {
    pub uri: String,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub blob: String,
}

/* 提示 */
/// 客户端发送此请求以获取服务器拥有的提示和提示模板列表。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListPromptsRequest {
    pub method: &'static str,
    #[serde(flatten)]
    pub paginated_request: PaginatedRequest,
}

/// 服务器对客户端的 prompts/list 请求的响应。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListPromptsResult {
    pub prompts: Vec<Prompt>,
    #[serde(flatten)]
    pub paginated_result: PaginatedResult,
}

/// 客户端用于获取服务器提供的提示的请求。
#[derive(Serialize, Deserialize, Debug)]
pub struct GetPromptRequest {
    pub method: &'static str,
    pub params: GetPromptParams,
}

/// 获取提示请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct GetPromptParams {
    pub name: String,
    pub arguments: Option<serde_json::Map<String, String>>,
}

/// 服务器对客户端的 prompts/get 请求的响应。
#[derive(Serialize, Deserialize, Debug)]
pub struct GetPromptResult {
    pub description: Option<String>,
    pub messages: Vec<PromptMessage>,
    #[serde(flatten)]
    pub result: Result,
}

/// 服务器提供的提示或提示模板。
#[derive(Serialize, Deserialize, Debug)]
pub struct Prompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<PromptArgument>>,
}

/// 描述提示可以接受的参数。
#[derive(Serialize, Deserialize, Debug)]
pub struct PromptArgument {
    pub name: String,
    pub description: Option<String>,
    pub required: Option<bool>,
}

/// 对话中消息和数据的发送者或接收者。
#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

/// 描述作为提示的一部分返回的消息。
/// 这类似于 `SamplingMessage`，但也支持嵌入 MCP 服务器的资源。
#[derive(Serialize, Deserialize, Debug)]
pub struct PromptMessage {
    pub role: Role,
    pub content: Either<TextContent, Either<ImageContent, Either<AudioContent, EmbeddedResource>>>,
}

/// 资源的内容，嵌入到提示或工具调用结果中。
/// 由客户端决定如何最好地渲染嵌入的资源，以利于大语言模型和/或用户。
#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddedResource {
    pub r#type: &'static str,
    pub resource: Either<TextResourceContents, BlobResourceContents>,
    pub annotations: Option<Annotations>,
}

/// 服务器向客户端发送的可选通知，告知其提供的提示列表已更改。服务器可以在客户端没有任何先前订阅的情况下发出此通知。
#[derive(Serialize, Deserialize, Debug)]
pub struct PromptListChangedNotification {
    pub method: &'static str,
}

/* 工具 */
/// 客户端发送此请求以获取服务器拥有的工具列表。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListToolsRequest {
    pub method: &'static str,
    #[serde(flatten)]
    pub paginated_request: PaginatedRequest,
}

/// 服务器对客户端的 tools/list 请求的响应。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListToolsResult {
    pub tools: Vec<Tool>,
    #[serde(flatten)]
    pub paginated_result: PaginatedResult,
}

/// 服务器对工具调用的响应。
/// 工具产生的任何错误都应该在结果对象中报告，将 `isError` 设置为 true，而不是作为 MCP 协议级别的错误响应。否则，大语言模型将无法看到错误发生并进行自我纠正。
/// 然而，任何查找工具的错误、表明服务器不支持工具调用的错误或任何其他异常情况，都应该作为 MCP 错误响应报告。
#[derive(Serialize, Deserialize, Debug)]
pub struct CallToolResult {
    pub content: Vec<Either<TextContent, Either<ImageContent, Either<AudioContent, EmbeddedResource>>>>,
    #[serde(rename = "isError")]
    pub is_error: Option<bool>,
    #[serde(flatten)]
    pub result: Result,
}

/// 客户端用于调用服务器提供的工具的请求。
#[derive(Serialize, Deserialize, Debug)]
pub struct CallToolRequest {
    pub method: &'static str,
    pub params: CallToolParams,
}

/// 调用工具请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct CallToolParams {
    pub name: String,
    pub arguments: Option<serde_json::Map<String, serde_json::Value>>,
}

/// 服务器向客户端发送的可选通知，告知其提供的工具列表已更改。服务器可以在客户端没有任何先前订阅的情况下发出此通知。
#[derive(Serialize, Deserialize, Debug)]
pub struct ToolListChangedNotification {
    pub method: &'static str,
}

/// 向客户端描述工具的附加属性。
/// 注意：ToolAnnotations 中的所有属性都是 **提示**。
/// 它们不能保证提供工具行为的真实描述（包括像 `title` 这样的描述性属性）。
/// 客户端永远不应该根据从不信任的服务器接收到的 ToolAnnotations 来做出工具使用决策。
#[derive(Serialize, Deserialize, Debug)]
pub struct ToolAnnotations {
    pub title: Option<String>,
    #[serde(rename = "readOnlyHint")]
    pub read_only_hint: Option<bool>,
    #[serde(rename = "destructiveHint")]
    pub destructive_hint: Option<bool>,
    #[serde(rename = "idempotentHint")]
    pub idempotent_hint: Option<bool>,
    #[serde(rename = "openWorldHint")]
    pub open_world_hint: Option<bool>,
}

/// 客户端可以调用的工具的定义。
#[derive(Serialize, Deserialize, Debug)]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "inputSchema")]
    pub input_schema: InputSchema,
    pub annotations: Option<ToolAnnotations>,
}

/// 输入模式结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct InputSchema {
    pub r#type: &'static str,
    pub properties: Option<serde_json::Map<String, serde_json::Value>>,
    pub required: Option<Vec<String>>,
}

/* 日志记录 */
/// 客户端向服务器发送的请求，用于启用或调整日志记录。
#[derive(Serialize, Deserialize, Debug)]
pub struct SetLevelRequest {
    pub method: &'static str,
    pub params: SetLevelParams,
}

/// 设置日志级别请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct SetLevelParams {
    pub level: LoggingLevel,
}

/// 服务器向客户端传递的日志消息通知。如果客户端没有发送 logging/setLevel 请求，服务器可以决定自动发送哪些消息。
#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingMessageNotification {
    pub method: &'static str,
    pub params: LoggingMessageParams,
}

/// 日志消息参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct LoggingMessageParams {
    pub level: LoggingLevel,
    pub logger: Option<String>,
    pub data: serde_json::Value,
}

/// 日志消息的严重性。
/// 这些映射到 syslog 消息严重性，如 RFC-5424 中所规定：
/// https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1
#[derive(Serialize, Deserialize, Debug)]
pub enum LoggingLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "emergency")]
    Emergency,
}

/* 采样 */
/// 服务器向客户端发送的请求，用于通过客户端对大语言模型进行采样。客户端完全有权决定选择哪个模型。客户端还应该在开始采样之前通知用户，以便他们检查请求（人工介入）并决定是否批准。
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessageRequest {
    pub method: &'static str,
    pub params: CreateMessageParams,
}

/// 创建消息请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessageParams {
    pub messages: Vec<SamplingMessage>,
    #[serde(rename = "modelPreferences")]
    pub model_preferences: Option<ModelPreferences>,
    #[serde(rename = "systemPrompt")]
    pub system_prompt: Option<String>,
    #[serde(rename = "includeContext")]
    pub include_context: Option<IncludeContext>,
    pub temperature: Option<f64>,
    #[serde(rename = "maxTokens")]
    pub max_tokens: i64,
    #[serde(rename = "stopSequences")]
    pub stop_sequences: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
}

/// 客户端对服务器的 sampling/create_message 请求的响应。客户端应该在返回采样消息之前通知用户，以便他们检查响应（人工介入）并决定是否允许服务器查看。
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessageResult {
    pub model: String,
    #[serde(rename = "stopReason")]
    pub stop_reason: Option<StopReason>,
    #[serde(flatten)]
    pub result: Result,
    #[serde(flatten)]
    pub sampling_message: SamplingMessage,
}

/// 描述发送到或从大语言模型 API 接收的消息。
#[derive(Serialize, Deserialize, Debug)]
pub struct SamplingMessage {
    pub role: Role,
    pub content: Either<TextContent, Either<ImageContent, AudioContent>>,
}

/// 客户端的可选注释。客户端可以使用注释来告知如何使用或显示对象。
#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations {
    pub audience: Option<Vec<Role>>,
    pub priority: Option<f64>,
}

/// 提供给大语言模型或从大语言模型返回的文本。
#[derive(Serialize, Deserialize, Debug)]
pub struct TextContent {
    pub r#type: &'static str,
    pub text: String,
    pub annotations: Option<Annotations>,
}

/// 提供给大语言模型或从大语言模型返回的图像。
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageContent {
    pub r#type: &'static str,
    pub data: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub annotations: Option<Annotations>,
}

/// 提供给大语言模型或从大语言模型返回的音频。
#[derive(Serialize, Deserialize, Debug)]
pub struct AudioContent {
    pub r#type: &'static str,
    pub data: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub annotations: Option<Annotations>,
}

/// 服务器在采样期间向客户端请求的模型选择偏好。
/// 由于大语言模型可以在多个维度上有所不同，选择 “最佳” 模型很少是直接的。不同的模型在不同的领域表现出色 —— 有些更快但能力较弱，有些能力更强但更昂贵，等等。此接口允许服务器在多个维度上表达其优先级，以帮助客户端为其用例做出适当的选择。
/// 这些偏好始终是建议性的。客户端可以忽略它们。客户端也可以决定如何解释这些偏好以及如何将它们与其他考虑因素进行权衡。
#[derive(Serialize, Deserialize, Debug)]
pub struct ModelPreferences {
    pub hints: Option<Vec<ModelHint>>,
    #[serde(rename = "costPriority")]
    pub cost_priority: Option<f64>,
    #[serde(rename = "speedPriority")]
    pub speed_priority: Option<f64>,
    #[serde(rename = "intelligencePriority")]
    pub intelligence_priority: Option<f64>,
}

/// 用于模型选择的提示。
/// 此处未声明的键目前由规范留空，由客户端解释。
#[derive(Serialize, Deserialize, Debug)]
pub struct ModelHint {
    pub name: Option<String>,
}

/* 自动完成 */
/// 客户端向服务器发送的请求，用于请求完成选项。
#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteRequest {
    pub method: &'static str,
    pub params: CompleteParams,
}

/// 完成请求的参数结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteParams {
    pub ref: Either<PromptReference, ResourceReference>,
    pub argument: ArgumentInfo,
}

/// 参数信息结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct ArgumentInfo {
    pub name: String,
    pub value: String,
}

/// 服务器对 completion/complete 请求的响应
#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteResult {
    pub completion: CompletionInfo,
    #[serde(flatten)]
    pub result: Result,
}

/// 完成信息结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionInfo {
    pub values: Vec<String>,
    pub total: Option<i64>,
    #[serde(rename = "hasMore")]
    pub has_more: Option<bool>,
}

/// 对资源或资源模板定义的引用。
#[derive(Serialize, Deserialize, Debug)]
pub struct ResourceReference {
    pub r#type: &'static str,
    pub uri: String,
}

/// 标识一个提示。
#[derive(Serialize, Deserialize, Debug)]
pub struct PromptReference {
    pub r#type: &'static str,
    pub name: String,
}

/* 根目录 */
/// 服务器向客户端发送此请求，以获取根 URI 列表。根目录允许服务器请求特定的目录或文件进行操作。根目录的一个常见示例是提供服务器应操作的一组存储库或目录。
/// 当服务器需要了解文件系统结构或访问客户端有权限读取的特定位置时，通常会使用此请求。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListRootsRequest {
    pub method: &'static str,
}

/// 客户端对服务器的 roots/list 请求的响应。
/// 此结果包含一个 Root 对象数组，每个对象代表服务器可以操作的根目录或文件。
#[derive(Serialize, Deserialize, Debug)]
pub struct ListRootsResult {
    pub roots: Vec<Root>,
    #[serde(flatten)]
    pub result: Result,
}

/// 表示服务器可以操作的根目录或文件。
#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub uri: String,
    pub name: Option<String>,
}

/// 客户端向服务器发送的通知，告知其根目录列表已更改。
/// 每当客户端添加、删除或修改任何根目录时，都应发送此通知。然后，服务器应使用 ListRootsRequest 请求更新后的根目录列表。
#[derive(Serialize, Deserialize, Debug)]
pub struct RootsListChangedNotification {
    pub method: &'static str,
}

/* 客户端消息 */
pub type ClientRequest = Either<PingRequest, Either<InitializeRequest, Either<CompleteRequest, Either<SetLevelRequest, Either<GetPromptRequest, Either<ListPromptsRequest, Either<ListResourcesRequest, Either<ReadResourceRequest, Either<SubscribeRequest, Either<UnsubscribeRequest, Either<CallToolRequest, ListToolsRequest>>>>>>>>>>>;

pub type ClientNotification = Either<CancelledNotification, Either<ProgressNotification, Either<InitializedNotification, RootsListChangedNotification>>>;

pub type ClientResult = Either<EmptyResult, Either<CreateMessageResult, ListRootsResult>>;

/* 服务器消息 */
pub type ServerRequest = Either<PingRequest, Either<CreateMessageRequest, ListRootsRequest>>;

pub type ServerNotification = Either<CancelledNotification, Either<ProgressNotification, Either<LoggingMessageNotification, Either<ResourceUpdatedNotification, Either<ResourceListChangedNotification, Either<ToolListChangedNotification, PromptListChangedNotification>>>>>>>>;

pub type ServerResult = Either<EmptyResult, Either<InitializeResult, Either<CompleteResult, Either<GetPromptResult, Either<ListPromptsResult, Either<ListResourcesResult, Either<ReadResourceResult, Either<CallToolResult, ListToolsResult>>>>>>>>;

// 用于 Rust 中表示 Either 类型
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}
