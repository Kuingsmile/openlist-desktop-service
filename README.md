# Generic Process Manager Service

A cross-platform desktop service with RESTful HTTP API for managing multiple processes.

## Architecture

This service provides a generic process management system that can manage multiple processes with custom configurations. It includes:

1. **HTTP API Service** - The persistent service that provides the REST API
2. **Process Management** - Manage multiple processes with custom binary paths, arguments, and configurations
3. **Process Monitoring** - Track process status, PIDs, restart counts, and exit codes
4. **Log Management** - Centralized logging for each managed process

The HTTP API service runs continuously and provides endpoints to create, update, delete, start, and stop processes. Each process has a unique ID and can be configured independently.

## Features

- RESTful HTTP API for process management
- Environment variable configuration
- Simple API key authentication
- Cross-platform support (Windows, Linux, macOS)
- Process monitoring and status tracking
- Individual process log management
- Unique process identification system
- Backward compatibility with legacy endpoints

## Configuration

The service can be configured using environment variables and will automatically persist process configurations to disk.

### Configuration File

Process configurations are automatically saved to a platform-specific configuration file:

- **Windows**: `%APPDATA%\OpenListService\process_configs.json`
- **macOS**: `~/Library/Application Support/OpenListService/process_configs.json`
- **Linux**: `~/.config/openlist-service/process_configs.json` (or `$XDG_CONFIG_HOME/openlist-service/process_configs.json`)

The configuration file is automatically created when you add your first process and updated whenever you:

- Create a new process
- Update an existing process configuration
- Delete a process

On service startup, all previously configured processes are automatically loaded from this file.

### Configuration File Format

The configuration file is stored in JSON format and contains an array of process configurations. Here's an example:

```json
{
  "processes": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "My Web Server",
      "bin_path": "/path/to/server",
      "args": ["--port", "8080", "--verbose"],
      "log_file": "server.log",
      "working_dir": "/path/to/workdir",
      "env_vars": {
        "NODE_ENV": "production",
        "API_KEY": "secret"
      },
      "auto_restart": true,
      "run_as_admin": false,
      "created_at": 1640995100,
      "updated_at": 1640995100
    }
  ]
}
```

**Important Notes:**

- Do not manually edit this file while the service is running, as changes will be overwritten
- If you need to manually edit the configuration, stop the service first
- Invalid configurations will be logged as errors during startup
- The service will start normally even if some process configurations are invalid

### Configuration Backup and Recovery

Since process configurations are now persistent, you may want to back up your configuration file:

**Backup:**

```bash
# Windows
copy "%APPDATA%\OpenListService\process_configs.json" "backup_location\"

# Linux/macOS
cp ~/.config/openlist-service/process_configs.json /backup/location/
# or for macOS
cp "~/Library/Application Support/OpenListService/process_configs.json" /backup/location/
```

**Recovery:**

1. Stop the service
2. Replace the configuration file with your backup
3. Start the service

The service will automatically load all process configurations from the file on startup.

### Environment Variables

| Variable                     | Default     | Description                     |
| ---------------------------- | ----------- | ------------------------------- |
| `PROCESS_MANAGER_HOST`       | `127.0.0.1` | API server host address         |
| `PROCESS_MANAGER_PORT`       | `53211`     | API server port                 |
| `PROCESS_MANAGER_API_KEY`    | (built-in)  | API authentication key          |
| `PROCESS_MANAGER_AUTO_START` | `true`      | Auto-start configured processes |

### Auto-Start Configuration

By default, the service will automatically start any configured processes when the service starts. You can control this behavior:

- `PROCESS_MANAGER_AUTO_START=true` or `1`: Enable auto-start (default)
- `PROCESS_MANAGER_AUTO_START=false` or `0`: Disable auto-start, manual start required

### Setting Environment Variables

**Windows (PowerShell):**

```powershell
$env:PROCESS_MANAGER_API_KEY="your-secure-api-key"
$env:PROCESS_MANAGER_PORT="8080"
$env:PROCESS_MANAGER_AUTO_START="true"
./openlist-desktop-service.exe
```

**Windows (CMD):**

```cmd
set PROCESS_MANAGER_API_KEY=your-secure-api-key
set PROCESS_MANAGER_PORT=8080
set PROCESS_MANAGER_AUTO_START=true
openlist-desktop-service.exe
```

**Linux/macOS:**

```bash
export PROCESS_MANAGER_API_KEY="your-secure-api-key"
export PROCESS_MANAGER_PORT="8080"
export PROCESS_MANAGER_AUTO_START="true"
./openlist-desktop-service
```

## Running the Service

### Windows Service Mode (Production)

When installed as a Windows service, the application runs through the Windows Service Control Manager (SCM):

```bash
# Install as service
.\install-openlist-service.exe

# The service will start automatically
# Control via Windows Services or command line:
sc start openlist_desktop_service
sc stop openlist_desktop_service
```

### Windows Console Mode (Development/Testing)

For development and testing purposes, you can run the service directly in console mode:

```bash
# Run in console mode (bypasses Windows Service Manager)
.\openlist-desktop-service.exe --console
# or
.\openlist-desktop-service.exe -c
```

**Note:** If you try to run the Windows service executable without the `--console` flag, you'll get error 1063 ("The service process could not connect to the service controller"). This is because Windows service executables must be launched by the Service Control Manager, not directly.

### Linux/macOS (Standard Mode)

On Linux and macOS, the service runs as a standard application:

```bash
# Direct execution
./openlist-desktop-service

# With systemd (Linux)
sudo systemctl start openlist-desktop-service
sudo systemctl stop openlist-desktop-service

# With OpenRC (Linux)
sudo rc-service openlist-desktop-service start
sudo rc-service openlist-desktop-service stop

# With launchd (macOS)
launchctl start io.github.openlistteam.openlist.service
launchctl stop io.github.openlistteam.openlist.service
```

## API Endpoints

### Authentication

All protected endpoints require an API key in the `Authorization` header:

```bash
Authorization: your-api-key
# or
Authorization: Bearer your-api-key
```

### Process Management Endpoints

| Method | Endpoint                      | Description                  |
| ------ | ----------------------------- | ---------------------------- |
| GET    | `/health`                     | Health check (no auth)       |
| GET    | `/api/v1/status`              | Get service status           |
| GET    | `/api/v1/version`             | Get version information      |
| POST   | `/api/v1/shutdown`            | Shutdown entire service      |
| GET    | `/api/v1/processes`           | List all processes           |
| POST   | `/api/v1/processes`           | Create new process           |
| GET    | `/api/v1/processes/:id`       | Get process details          |
| PUT    | `/api/v1/processes/:id`       | Update process configuration |
| DELETE | `/api/v1/processes/:id`       | Delete process               |
| POST   | `/api/v1/processes/:id/start` | Start process                |
| POST   | `/api/v1/processes/:id/stop`  | Stop process                 |
| GET    | `/api/v1/processes/:id/logs`  | Get process logs             |

### Usage Examples

**Health Check (no auth required):**

```bash
curl http://127.0.0.1:53211/health
```

**List All Processes:**

```bash
curl -H "Authorization: your-api-key" http://127.0.0.1:53211/api/v1/processes
```

**Create New Process:**

```bash
curl -X POST -H "Authorization: your-api-key" \
     -H "Content-Type: application/json" \
     -d '{
       "name": "My App",
       "bin_path": "/path/to/binary",
       "args": ["--port", "8080", "--verbose"],
       "log_file": "/path/to/app.log",
       "working_dir": "/path/to/workdir",
       "auto_restart": false,
       "run_as_admin": true
     }' \
     http://127.0.0.1:53211/api/v1/processes
```

**Start Process:**

```bash
curl -X POST -H "Authorization: your-api-key" \
     http://127.0.0.1:53211/api/v1/processes/{process-id}/start
```

**Stop Process:**

```bash
curl -X POST -H "Authorization: your-api-key" \
     http://127.0.0.1:53211/api/v1/processes/{process-id}/stop
```

**Get Process Logs:**

```bash
# Get last 50 lines
curl -H "Authorization: your-api-key" \
     "http://127.0.0.1:53211/api/v1/processes/{process-id}/logs?lines=50"
```

**Update Process Configuration:**

```bash
curl -X PUT -H "Authorization: your-api-key" \
     -H "Content-Type: application/json" \
     -d '{
       "name": "Updated App Name",
       "args": ["--port", "9090"],
       "run_as_admin": false
     }' \
     http://127.0.0.1:53211/api/v1/processes/{process-id}
```

### Process Configuration

When creating or updating a process, you can specify:

- `name`: Display name for the process
- `bin_path`: Path to the executable binary
- `args`: Array of command-line arguments
- `log_file`: Path to log file (optional, auto-generated if not provided)
- `working_dir`: Working directory for the process (optional)
- `env_vars`: Environment variables as key-value pairs (optional)
- `auto_restart`: Whether to automatically restart on failure (optional)
- `run_as_admin`: Whether to run with administrator/root privileges (optional)

### Administrator/Root Privileges

The service supports running processes with elevated privileges:

- **Windows**: Uses PowerShell's `Start-Process -Verb RunAs` to launch processes with administrator privileges
- **Linux**: Uses `sudo` to run processes as root (requires sudo to be available)
- **macOS**: Uses `sudo` to run processes with administrator privileges (requires sudo to be available)

**Important Security Notes:**

- The service itself must be running with sufficient privileges to escalate process privileges
- On Windows, UAC prompts may appear unless the service is running as an administrator
- On Linux/macOS, the user running the service must have sudo privileges without password prompts for seamless operation
- Use `run_as_admin: true` carefully and only when necessary

### Response Format

All API responses follow this format:

```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "timestamp": 1640995200
}
```

### Process Status Response

```json
{
  "success": true,
  "data": {
    "id": "uuid-here",
    "name": "My App",
    "is_running": true,
    "pid": 12345,
    "started_at": 1640995200,
    "restart_count": 0,
    "last_exit_code": null,
    "config": {
      "id": "uuid-here",
      "name": "My App",
      "bin_path": "/path/to/binary",
      "args": ["--port", "8080"],
      "log_file": "/path/to/app.log",
      "working_dir": "/path/to/workdir",
      "env_vars": {},
      "auto_restart": false,
      "run_as_admin": true,
      "created_at": 1640995100,
      "updated_at": 1640995100
    }
  },
  "error": null,
  "timestamp": 1640995300
}
```

## Building

```bash
cargo build --release
```

## Installation

The service supports automatic installation and management on different platforms:

### Linux

The service automatically detects your Linux init system and installs accordingly:

- **systemd** (most common): Creates service file in `/etc/systemd/system/`
- **OpenRC** (Alpine, Gentoo, etc.): Creates init script in `/etc/init.d/`

**Install Service:**

```bash
sudo ./install-openlist-service
```

**Uninstall Service:**

```bash
sudo ./uninstall-openlist-service
```

#### Init System Detection

The service automatically detects your init system by checking for:

- OpenRC: `/sbin/openrc` or `/usr/bin/rc-update`
- systemd: Default fallback

#### OpenRC Support

For OpenRC-based systems (Alpine Linux, Gentoo, etc.), the service will:

- Create an OpenRC init script at `/etc/init.d/openlist-desktop-service`
- Add the service to the default runlevel using `rc-update`
- Support standard OpenRC commands:
  - `rc-service openlist-desktop-service start`
  - `rc-service openlist-desktop-service stop`
  - `rc-service openlist-desktop-service status`

#### systemd Support

For systemd-based systems, the service will:

- Create a systemd unit file at `/etc/systemd/system/openlist-desktop-service.service`
- Enable the service using `systemctl enable`
- Support standard systemctl commands:
  - `systemctl start openlist-desktop-service`
  - `systemctl stop openlist-desktop-service`
  - `systemctl status openlist-desktop-service`

### Windows

The service installs as a Windows Service that starts automatically with the system.

### macOS

The service installs as a Launch Agent (user service) that runs in user space.

- Service is installed in `~/Library/LaunchAgents/` (user-writable location)
- Service binary is stored in `~/Library/Application Support/`

**Install Service:**

```bash
./install-openlist-service
```

**Uninstall Service:**

```bash
./uninstall-openlist-service
```

See the `install.rs` and `uninstall.rs` for platform-specific service installation details.

### License

This project is inspired by the [clash-verge-service](https://github.com/clash-verge-rev/clash-verge-service) for the original idea and architecture. It is released under the GNU General Public License v3.0 (GPL-3.0).

The [LICENSE](LICENSE) file is included in the repository.
