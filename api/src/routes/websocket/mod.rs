use std::sync::Arc;
use axum::response::Html;
use axum::Router;
use axum::routing::get;
use crate::routes::websocket::models::ConnectionManager;

pub mod models;
pub mod ws;

/// websocket api 路由
pub fn set_websocket_api() -> Router {
    // 创建连接管理器
    let connection_manager = Arc::new(ConnectionManager::new());

    Router::new()
        .route("/", get(ws::websocket_handler))
        .route("/test", get(websocket_html))
        .with_state(connection_manager)
}

/// 长连接测试页面
async fn websocket_html() -> Html<&'static str> {
    let html_content = r#"
<!DOCTYPE html>
<html lang="zh-CN">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WebSocket 测试页面</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">
         <style>
             * {
                 box-sizing: border-box;
                 margin: 0;
                 padding: 0;
             }

         body {
             font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
             background-color: #1a1a2e;
             color: #e6e6e6;
             line-height: 1.6;
             padding: 20px;
             min-height: 100vh;
         }

             .container {
             max-width: 1200px;
             margin: 0 auto;
         }

         header {
             text-align: center;
             margin-bottom: 30px;
             padding-bottom: 20px;
             border-bottom: 1px solid #30475e;
         }

         h1 {
             color: #4cc9f0;
             margin-bottom: 10px;
             font-size: 2.5rem;
         }

             .subtitle {
             color: #a3a3c2;
             font-size: 1.1rem;
         }

             .main-content {
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
            margin-bottom: 30px;
        }

            .panel {
             background-color: #162447;
             border-radius: 10px;
             padding: 20px;
             box-shadow: 0 5px 15px rgba(0, 0, 0, 0.3);
             flex: 1;
             min-width: 400px;
         }

             .panel h2 {
            color: #4cc9f0;
            margin-bottom: 15px;
            padding-bottom: 10px;
            border-bottom: 1px solid #30475e;
            font-size: 1.5rem;
        }

             .control-group {
            margin-bottom: 20px;
        }

         label {
             display: block;
             margin-bottom: 8px;
             color: #a3a3c2;
             font-weight: 600;
         }

         input,
         select,
         textarea {
             width: 100%;
             padding: 12px;
             background-color: #1f4068;
             border: 1px solid #30475e;
             border-radius: 6px;
             color: #e6e6e6;
             font-size: 1rem;
         }

         input:focus,
         select:focus,
         textarea:focus {
             outline: none;
             border-color: #4cc9f0;
         }

         button {
             background-color: #2d6cdf;
             color: white;
             border: none;
             padding: 12px 20px;
             border-radius: 6px;
             cursor: pointer;
             font-size: 1rem;
             font-weight: 600;
             transition: all 0.3s ease;
             display: flex;
             align-items: center;
             justify-content: center;
             gap: 8px;
         }

         button:hover {
             background-color: #1a56c9;
             transform: translateY(-2px);
         }

         button:active {
             transform: translateY(0);
         }

         button.primary {
             background-color: #4cc9f0;
             color: #162447;
         }

         button.primary:hover {
             background-color: #3ab4d9;
         }

         button.success {
             background-color: #10b981;
         }

         button.success:hover {
             background-color: #0da271;
         }

         button.danger {
             background-color: #ef4444;
         }

         button.danger:hover {
             background-color: #dc2626;
         }

         button:disabled {
             background-color: #6b7280;
             cursor: not-allowed;
             transform: none;
         }

             .buttons-group {
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
        }

            .status-indicator {
            display: flex;
            align-items: center;
            gap: 8px;
            margin-bottom: 20px;
            padding: 10px;
            border-radius: 6px;
            background-color: #1f4068;
        }

            .status-dot {
            width: 12px;
            height: 12px;
            border-radius: 50%;
            background-color: #6b7280;
        }

            .status-dot.connected {
             background-color: #10b981;
             box-shadow: 0 0 8px #10b981;
         }

             .status-dot.connecting {
             background-color: #f59e0b;
             box-shadow: 0 0 8px #f59e0b;
             animation: pulse 1.5s infinite;
         }

         @keyframes pulse {
            0% {
            opacity: 1;
            }

            50% {
            opacity: 0.5;
            }

            100% {
            opacity: 1;
            }
        }

             .message-log {
            height: 300px;
            overflow-y: auto;
            background-color: #1f4068;
            border-radius: 6px;
            padding: 15px;
            margin-top: 10px;
            font-family: 'Courier New', monospace;
            font-size: 0.9rem;
        }

            .message-item {
            padding: 8px 0;
            border-bottom: 1px solid #30475e;
        }

            .message-item:last-child {
            border-bottom: none;
        }

            .message-time {
            color: #9ca3af;
            font-size: 0.8rem;
            margin-right: 10px;
        }

            .message-direction {
            display: inline-block;
            padding: 2px 8px;
            border-radius: 4px;
            font-size: 0.8rem;
            font-weight: bold;
            margin-right: 8px;
        }

            .message-out {
            background-color: #3b82f6;
            color: white;
        }

            .message-in {
             background-color: #10b981;
             color: white;
         }

             .message-system {
            background-color: #6b7280;
            color: white;
        }

            .heartbeat-status {
            display: flex;
            align-items: center;
            justify-content: space-between;
            margin-top: 20px;
            padding-top: 15px;
            border-top: 1px solid #30475e;
        }

            .heartbeat-info {
            display: flex;
            flex-direction: column;
            gap: 5px;
        }

            .heartbeat-count {
            font-weight: bold;
            color: #4cc9f0;
        }

            .config-section {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 15px;
            margin-bottom: 20px;
        }

         @media (max-width: 768px) {
             .config-section {
                grid-template-columns: 1fr;
            }

                .buttons-group {
                flex-direction: column;
            }

             button {
                 width: 100%;
             }
         }

         footer {
             text-align: center;
             color: #a3a3c2;
             padding-top: 20px;
             border-top: 1px solid #30475e;
             font-size: 0.9rem;
         }

             .icon {
             font-size: 1.2rem;
         }
             </style>
             </head>

             <body>
             <div class="container">
            <header>
            <h1><i class="fas fa-broadcast-tower icon"></i> WebSocket 测试工具</h1>
             <p class="subtitle">测试WebSocket连接，发送和接收消息，管理心跳包</p>
             </header>

             <div class="main-content">
            <div class="panel" style="flex: 1;">
            <h2><i class="fas fa-cogs icon"></i> 连接配置</h2>

            <div class="config-section">
            <div class="control-group">
            <label for="ws-url">WebSocket 服务器地址</label>
             <input type="text" id="ws-url" value="wss://echo.websocket.org">
        <p style="font-size: 0.9rem; margin-top: 5px; color: #9ca3af;">使用 wss://echo.websocket.org 进行测试
    </p>
        </div>

        <div class="control-group">
        <label for="heartbeat-interval">心跳间隔（秒）</label>
        <select id="heartbeat-interval">
        <option value="5">5 秒</option>
        <option value="10" selected>10 秒</option>
        <option value="30">30 秒</option>
        <option value="60">60 秒</option>
        <option value="0">关闭心跳</option>
        </select>
        </div>
        </div>

        <div class="control-group">
        <div class="status-indicator">
        <div class="status-dot" id="status-dot"></div>
        <span id="status-text">未连接</span>
        </div>

        <div class="buttons-group">
        <button id="connect-btn" class="primary">
        <i class="fas fa-plug icon"></i> 连接
        </button>
        <button id="disconnect-btn" disabled>
        <i class="fas fa-plug icon"></i> 断开连接
        </button>
        <button id="clear-btn">
        <i class="fas fa-trash-alt icon"></i> 清空消息
        </button>
        </div>
        </div>

        <div class="heartbeat-status">
        <div class="heartbeat-info">
        <span>心跳包状态: <span id="heartbeat-status">未激活</span></span>
        <span>发送计数: <span class="heartbeat-count" id="heartbeat-count">0</span></span>
        </div>
        <button id="heartbeat-btn" class="success">
        <i class="fas fa-heartbeat icon"></i> 发送心跳包
        </button>
        </div>
        </div>

        <div class="panel" style="flex: 1;">
        <h2><i class="fas fa-paper-plane icon"></i> 发送消息</h2>

        <div class="control-group">
        <label for="message-type">消息类型</label>
        <select id="message-type">
        <option value="text">文本消息</option>
        <option value="json">JSON数据</option>
        <option value="ping">Ping测试</option>
        </select>
        </div>

        <div class="control-group">
        <label for="message-content">消息内容</label>
        <textarea id="message-content" rows="6"
    placeholder='输入要发送的消息内容，对于JSON类型，请确保输入有效的JSON格式，例如：{"type": "message", "content": "Hello"}'></textarea>
        </div>

        <div class="buttons-group">
        <button id="send-btn" disabled>
        <i class="fas fa-paper-plane icon"></i> 发送消息
        </button>
        <button id="sample-json-btn">
        <i class="fas fa-code icon"></i> 填充示例JSON
        </button>
        </div>
        </div>

        <div class="panel" style="flex: 2;">
        <h2><i class="fas fa-inbox icon"></i> 接收消息</h2>

        <div class="control-group">
        <label>消息日志</label>
        <div class="message-log" id="message-log">
        <div class="message-item">
        <span class="message-time">--:--:--</span>
        <span class="message-direction message-system">系统</span>
        <span>等待连接WebSocket服务器...</span>
        </div>
        </div>
        </div>

        <div class="buttons-group">
        <button id="auto-scroll-toggle" class="primary">
        <i class="fas fa-arrow-down icon"></i> 自动滚动
        </button>
        <button id="export-log-btn">
        <i class="fas fa-download icon"></i> 导出日志
        </button>
        </div>
        </div>
        </div>

        <footer>
        <p>WebSocket 测试工具 &copy; 2023 | 使用HTML5 WebSocket API构建</p>
        <p>提示: 您可以使用 wss://echo.websocket.org 作为测试服务器，它会将您发送的消息原样返回</p>
    </footer>
        </div>

        <script>
    // WebSocket连接状态和变量
    let socket = null;
    let isConnected = false;
    let heartbeatInterval = null;
    let heartbeatCount = 0;
    let autoScroll = true;
    let messageLog = [];

    // DOM元素
    const connectBtn = document.getElementById('connect-btn');
    const disconnectBtn = document.getElementById('disconnect-btn');
    const clearBtn = document.getElementById('clear-btn');
    const sendBtn = document.getElementById('send-btn');
    const heartbeatBtn = document.getElementById('heartbeat-btn');
    const sampleJsonBtn = document.getElementById('sample-json-btn');
    const autoScrollToggle = document.getElementById('auto-scroll-toggle');
    const exportLogBtn = document.getElementById('export-log-btn');

    const wsUrlInput = document.getElementById('ws-url');
    const heartbeatIntervalSelect = document.getElementById('heartbeat-interval');
    const messageTypeSelect = document.getElementById('message-type');
    const messageContentTextarea = document.getElementById('message-content');
    const messageLogDiv = document.getElementById('message-log');
    const statusDot = document.getElementById('status-dot');
    const statusText = document.getElementById('status-text');
    const heartbeatStatus = document.getElementById('heartbeat-status');
    const heartbeatCountSpan = document.getElementById('heartbeat-count');

    // 更新连接状态显示
    function updateConnectionStatus(connected, connecting = false) {
        isConnected = connected;

        if (connecting) {
            statusDot.className = 'status-dot connecting';
            statusText.textContent = '连接中...';
        } else if (connected) {
            statusDot.className = 'status-dot connected';
            statusText.textContent = '已连接';
        } else {
            statusDot.className = 'status-dot';
            statusText.textContent = '未连接';
        }

        connectBtn.disabled = connected || connecting;
        disconnectBtn.disabled = !connected;
        sendBtn.disabled = !connected;

        // 更新心跳按钮状态
        const interval = parseInt(heartbeatIntervalSelect.value);
        heartbeatBtn.disabled = !connected || interval === 0;
    }

    // 添加消息到日志
    function addMessageToLog(direction, content) {
        const now = new Date();
        const timeString = now.toTimeString().split(' ')[0];

        let directionClass = 'message-system';
        let directionText = '系统';

        if (direction === 'out') {
            directionClass = 'message-out';
            directionText = '发送';
        } else if (direction === 'in') {
            directionClass = 'message-in';
            directionText = '接收';
        }

        const messageItem = document.createElement('div');
        messageItem.className = 'message-item';
        messageItem.innerHTML = `
        <span class="message-time">${timeString}</span>
            <span class="message-direction ${directionClass}">${directionText}</span>
            <span>${content}</span>
        `;

        messageLogDiv.appendChild(messageItem);

        // 保存消息到数组
        messageLog.push({
            time: now.toISOString(),
            direction: directionText,
            content: content
        });

        // 自动滚动到底部
        if (autoScroll) {
            messageLogDiv.scrollTop = messageLogDiv.scrollHeight;
        }
    }

    // 连接WebSocket
    function connectWebSocket() {
        const url = wsUrlInput.value.trim();
        if (!url) {
            alert('请输入WebSocket服务器地址');
            return;
        }

        console.log(url);
        updateConnectionStatus(false, true);

        try {
            socket = new WebSocket(url);

            socket.onopen = function (event) {
                console.log('WebSocket连接已打开');
                updateConnectionStatus(true);
                addMessageToLog('system', `已连接到服务器: ${url}`);

                // 设置心跳包
                setupHeartbeat();
            };

            socket.onmessage = function (event) {
                console.log('收到消息:', event.data);

                // 检查是否是ping响应
                if (event.data === 'pong' || event.data === 'PONG') {
                    addMessageToLog('in', '收到心跳响应 (pong)');
                    return;
                }

                // 尝试解析JSON
                try {
                    const jsonData = JSON.parse(event.data);
                    addMessageToLog('in', `JSON: ${JSON.stringify(jsonData, null, 2)}`);
                } catch (e) {
                    // 如果不是JSON，则作为普通文本显示
                    addMessageToLog('in', event.data);
                }
            };

            socket.onclose = function (event) {
                console.log('WebSocket连接已关闭:', event.code, event.reason);
                updateConnectionStatus(false);

                // 清除心跳定时器
                if (heartbeatInterval) {
                    clearInterval(heartbeatInterval);
                    heartbeatInterval = null;
                }

                let closeMessage = `连接已关闭`;
                if (event.code === 1000) {
                    closeMessage += ' (正常关闭)';
                } else {
                    closeMessage += ` (代码: ${event.code}, 原因: ${event.reason || '未知'})`;
                }

                addMessageToLog('system', closeMessage);
                heartbeatStatus.textContent = '未激活';
            };

            socket.onerror = function (error) {
                console.error('WebSocket错误:', error);
                updateConnectionStatus(false);
                addMessageToLog('system', `连接错误: 无法连接到服务器`);

                // 清除心跳定时器
                if (heartbeatInterval) {
                    clearInterval(heartbeatInterval);
                    heartbeatInterval = null;
                }
            };

        } catch (error) {
            console.error('创建WebSocket连接时出错:', error);
            updateConnectionStatus(false);
            addMessageToLog('system', `创建连接时出错: ${error.message}`);
        }
    }

    // 断开WebSocket连接
    function disconnectWebSocket() {
        if (socket && isConnected) {
            socket.close(1000, JSON.stringify({ type: "close" }));
        }
    }

    // 设置心跳包
    function setupHeartbeat() {
        // 清除现有的心跳定时器
        if (heartbeatInterval) {
            clearInterval(heartbeatInterval);
            heartbeatInterval = null;
        }

        const interval = parseInt(heartbeatIntervalSelect.value);

        if (interval > 0 && isConnected) {
            heartbeatStatus.textContent = `每 ${interval} 秒发送`;

            // 立即发送第一个心跳包
            sendHeartbeat();

            // 设置定时器
            heartbeatInterval = setInterval(sendHeartbeat, interval * 1000);
        } else {
            heartbeatStatus.textContent = '未激活';
        }
    }

    // 发送心跳包
    function sendHeartbeat() {
        if (socket && socket.readyState === WebSocket.OPEN) {
            socket.send(JSON.stringify({ 'type': 'ping' }));
            heartbeatCount++;
            heartbeatCountSpan.textContent = heartbeatCount;
            addMessageToLog('out', '心跳包 (ping)');
        }
    }

    // 发送消息
    function sendMessage() {
        if (!socket || socket.readyState !== WebSocket.OPEN) {
            alert('WebSocket未连接');
            return;
        }

        const messageType = messageTypeSelect.value;
        let messageContent = messageContentTextarea.value.trim();

        if (!messageContent) {
            alert('请输入消息内容');
            return;
        }

        try {
            let messageToSend = messageContent;

            if (messageType === 'json') {
                // 验证JSON格式
                const jsonObj = JSON.parse(messageContent);
                messageToSend = JSON.stringify(jsonObj);
            } else if (messageType === 'ping') {
                messageToSend = 'ping';
            }

            // 发送消息
            socket.send(messageToSend);

            // 添加到日志
            let logMessage = messageContent;
            if (messageType === 'json') {
                logMessage = `JSON: ${messageContent}`;
            } else if (messageType === 'ping') {
                logMessage = 'ping (测试消息)';
            }

            addMessageToLog('out', logMessage);

            // 清空输入框（ping类型除外）
            if (messageType !== 'ping') {
                messageContentTextarea.value = '';
            }

        } catch (error) {
            alert(`消息格式错误: ${error.message}`);
        }
    }

    // 清空消息日志
    function clearMessageLog() {
        messageLogDiv.innerHTML = '';
        messageLog = [];

        // 添加初始消息
        addMessageToLog('system', '消息日志已清空');
    }

    // 填充示例JSON
    function fillSampleJson() {
        const sampleJson = {
            "type": "chat",
            "content": "Hello WebSocket!",
            "user": "测试用户",
            "timestamp": new Date().toISOString(),
            "data": {
                "id": 12345,
                "status": "active"
            }
        };

        messageTypeSelect.value = 'json';
        messageContentTextarea.value = JSON.stringify(sampleJson, null, 2);
    }

    // 导出日志
    function exportLog() {
        if (messageLog.length === 0) {
            alert('没有可导出的日志内容');
            return;
        }

        let logText = 'WebSocket 消息日志\n';
        logText += '生成时间: ' + new Date().toLocaleString() + '\n';
        logText += '服务器: ' + wsUrlInput.value + '\n';
        logText += '状态: ' + (isConnected ? '已连接' : '未连接') + '\n\n';

    messageLog.forEach(entry => {
    logText += `[${new Date(entry.time).toLocaleTimeString()}] [${entry.direction}] ${entry.content}\n`;
    });

    const blob = new Blob([logText], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `websocket-log-${new Date().toISOString().slice(0, 10)}.txt`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
    }

    // 事件监听器
    connectBtn.addEventListener('click', connectWebSocket);
    disconnectBtn.addEventListener('click', disconnectWebSocket);
    sendBtn.addEventListener('click', sendMessage);
    heartbeatBtn.addEventListener('click', sendHeartbeat);
    clearBtn.addEventListener('click', clearMessageLog);
    sampleJsonBtn.addEventListener('click', fillSampleJson);
    exportLogBtn.addEventListener('click', exportLog);

    // 自动滚动切换
    autoScrollToggle.addEventListener('click', function () {
    autoScroll = !autoScroll;
    autoScrollToggle.innerHTML = autoScroll ?
    '<i class="fas fa-arrow-down icon"></i> 自动滚动 (开)' :
    '<i class="fas fa-ban icon"></i> 自动滚动 (关)';

    autoScrollToggle.classList.toggle('primary', autoScroll);
    });

    // 心跳间隔改变
    heartbeatIntervalSelect.addEventListener('change', function () {
    const interval = parseInt(this.value);
    heartbeatBtn.disabled = !isConnected || interval === 0;

    if (isConnected) {
    setupHeartbeat();
    }
    });

    // 消息类型改变
    messageTypeSelect.addEventListener('change', function () {
    const placeholder = this.value === 'json' ?
    '输入要发送的消息内容，对于JSON类型，请确保输入有效的JSON格式，例如：{"type": "message", "content": "Hello"}' :
    '输入要发送的消息内容';

    messageContentTextarea.placeholder = placeholder;

    // 如果是ping类型，自动填充内容
    if (this.value === 'ping') {
    messageContentTextarea.value = 'ping';
    }
    });

    // 使用Enter键发送消息 (Ctrl+Enter 或 Cmd+Enter)
    messageContentTextarea.addEventListener('keydown', function (event) {
    if ((event.ctrlKey || event.metaKey) && event.key === 'Enter') {
    event.preventDefault();
    sendMessage();
    }
    });

    // 初始化
    updateConnectionStatus(false);

    // 添加示例消息到日志
    addMessageToLog('system', 'WebSocket测试工具已就绪');
    addMessageToLog('system', '使用 wss://echo.websocket.org 作为测试服务器');
    addMessageToLog('system', '点击"连接"按钮开始测试');
    </script>
    </body>

    </html>
"#;

    Html(html_content)
}