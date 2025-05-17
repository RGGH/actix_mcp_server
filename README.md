```
          ┌─────────────────────────────┐
          │        Claude Desktop       │
          │        (or other LLM)       │
          └────────────┬────────────────┘
                       │  (HTTP/SSE or stdio)
                       ▼
             ┌──────────────────────┐
             │                      │
             │    MCP Gateway       │
             │   (Actix + rmcp)     │
             │                      │
             ├──────────┬───────────┤
             │          │           │
   [1] /sse  │  [2] /message        │  [3] /servers
             │          ▼           │
             │  ┌───────────────┐   │
             │  │ Routing Layer │   │
             │  └──────┬────────┘   │
             │         │            │
             │         ▼            │
             │ ┌──────────────────┐ │
             │ │ Async Proxy Layer│ │
             │ └────┬──────┬──────┘ │
             │      │      │        │
             ▼      ▼      ▼        ▼
     ┌────────┐ ┌────────┐ ┌────────┐
     │ MCP #1 │ │ MCP #2 │ │ MCP #N │
     │(SSE or │ │(Stdio  │ │(...)   │
     │  rmcp) │ │  rmcp) │ │        │
     └────────┘ └────────┘ └────────┘



```

| Component                | Role                                                                  |
| ------------------------ | --------------------------------------------------------------------- |
| **Claude Desktop / LLM** | Sends messages to the Gateway and receives responses via SSE or stdio |
| **MCP Gateway**          | Main server that handles client requests and manages backends         |
| **Routing Layer**        | Decides which backend MCP server gets the request                     |
| **Async Proxy Layer**    | Uses `rmcp` to forward requests to each MCP server (SSE or stdio)     |
| **MCP Servers**          | Existing tools exposed via `rmcp::ServiceExt`                         |

